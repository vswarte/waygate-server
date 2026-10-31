/// Declares a `#[repr($int)]` enum that serializes as a raw
/// `$int` on the wire but as its variant name in human-readable formats.
///
/// Invalid input errors rather than falling back to a default variant.
/// Supported repr: `u8`, `u16`, `u32`, `u64`.
///
/// ```
/// wire::repr_enum! {
///     #[repr(u8)]
///     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
///     pub enum QuickmatchTeamSlot {
///         Ally = 0,
///         Enemy = 1,
///     }
/// }
/// ```
#[macro_export]
macro_rules! repr_enum {
    (
        #[repr($int:ident)]
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident = $value:literal),+ $(,)?
        }
    ) => {
        #[repr($int)]
        $(#[$meta])*
        $vis enum $name {
            $($variant = $value),+
        }

        impl $name {
            fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => ::std::stringify!($variant)),+
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                if ::serde::Serializer::is_human_readable(&serializer) {
                    serializer.serialize_str(self.as_str())
                } else {
                    $crate::__repr_enum_serialize_wire!(serializer, *self as $int)
                }
            }
        }

        const _: () = {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = $name;

                fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    ::std::write!(
                        formatter,
                        "a {} as a {} (wire) or a string (human-readable)",
                        ::std::stringify!($name),
                        ::std::stringify!($int),
                    )
                }

                $crate::__repr_enum_visit_wire!($name, $int, $($variant = $value),+);

                fn visit_str<E: ::serde::de::Error>(
                    self,
                    v: &str,
                ) -> ::std::result::Result<Self::Value, E> {
                    match v {
                        $(::std::stringify!($variant) => Ok($name::$variant),)+
                        other => Err(E::unknown_variant(other, &[$(::std::stringify!($variant)),+])),
                    }
                }
            }

            impl<'de> ::serde::Deserialize<'de> for $name {
                fn deserialize<D: ::serde::Deserializer<'de>>(
                    deserializer: D,
                ) -> ::std::result::Result<Self, D::Error> {
                    if ::serde::Deserializer::is_human_readable(&deserializer) {
                        deserializer.deserialize_str(Visitor)
                    } else {
                        $crate::__repr_enum_deserialize_wire!(deserializer, Visitor, $int)
                    }
                }
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __repr_enum_serialize_wire {
    ($serializer:ident, $value:expr) => {
        ::serde::Serialize::serialize(&($value), $serializer)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __repr_enum_deserialize_wire {
    ($deserializer:ident, $visitor:expr, u8) => {
        ::serde::Deserializer::deserialize_u8($deserializer, $visitor)
    };
    ($deserializer:ident, $visitor:expr, u16) => {
        ::serde::Deserializer::deserialize_u16($deserializer, $visitor)
    };
    ($deserializer:ident, $visitor:expr, u32) => {
        ::serde::Deserializer::deserialize_u32($deserializer, $visitor)
    };
    ($deserializer:ident, $visitor:expr, u64) => {
        ::serde::Deserializer::deserialize_u64($deserializer, $visitor)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __repr_enum_visit_wire {
    ($name:ident, u8, $($variant:ident = $value:literal),+) => {
        fn visit_u8<E: ::serde::de::Error>(self, v: u8) -> ::std::result::Result<Self::Value, E> {
            match v {
                $($value => Ok($name::$variant),)+
                other => Err(E::invalid_value(
                    ::serde::de::Unexpected::Unsigned(other as u64),
                    &concat!("one of: ", $(::std::stringify!($value), " "),+),
                )),
            }
        }
    };
    ($name:ident, u16, $($variant:ident = $value:literal),+) => {
        fn visit_u16<E: ::serde::de::Error>(self, v: u16) -> ::std::result::Result<Self::Value, E> {
            match v {
                $($value => Ok($name::$variant),)+
                other => Err(E::invalid_value(
                    ::serde::de::Unexpected::Unsigned(other as u64),
                    &concat!("one of: ", $(::std::stringify!($value), " "),+),
                )),
            }
        }
    };
    ($name:ident, u32, $($variant:ident = $value:literal),+) => {
        fn visit_u32<E: ::serde::de::Error>(self, v: u32) -> ::std::result::Result<Self::Value, E> {
            match v {
                $($value => Ok($name::$variant),)+
                other => Err(E::invalid_value(
                    ::serde::de::Unexpected::Unsigned(other as u64),
                    &concat!("one of: ", $(::std::stringify!($value), " "),+),
                )),
            }
        }
    };
    ($name:ident, u64, $($variant:ident = $value:literal),+) => {
        fn visit_u64<E: ::serde::de::Error>(self, v: u64) -> ::std::result::Result<Self::Value, E> {
            match v {
                $($value => Ok($name::$variant),)+
                other => Err(E::invalid_value(
                    ::serde::de::Unexpected::Unsigned(other),
                    &concat!("one of: ", $(::std::stringify!($value), " "),+),
                )),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{deserialize, serialize};

    crate::repr_enum! {
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        enum TestSlot {
            Ally = 0,
            Enemy = 1,
        }
    }

    #[test]
    fn wire_roundtrip_is_one_byte() {
        let bytes = serialize(TestSlot::Enemy).unwrap();
        assert_eq!(bytes, vec![1u8]);

        let decoded: TestSlot = deserialize(&bytes).unwrap();
        assert_eq!(decoded, TestSlot::Enemy);
    }

    #[test]
    fn wire_rejects_out_of_range_value() {
        let result: Result<TestSlot, _> = deserialize(&[2u8]);
        assert!(result.is_err());
    }
}
