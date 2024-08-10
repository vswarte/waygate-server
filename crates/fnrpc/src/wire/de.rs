use byteorder::{ReadBytesExt, LE};
use paste::item;
use serde::de;

use crate::wire::FNWireError;

impl de::Error for FNWireError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        FNWireError::Other(msg.to_string())
    }
}

#[derive(Debug)]
pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn new(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

macro_rules! de_primitive {
    ($tname:ident) => {
        item! {
            fn [< deserialize_ $tname >] <V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de> {

                visitor.[< visit_ $tname >](self.input.[< read_ $tname >]()?)
            }
        }
    };
}

macro_rules! de_primitive_le {
    ($tname:ident) => {
        item! {
            fn [< deserialize_ $tname >] <V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de> {

                visitor.[< visit_ $tname >](self.input.[< read_ $tname >]::<LE>()?)
            }
        }
    };
}

struct SeqAccess<'de, 'a> {
    deserializer: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'de, 'a> {
    type Error = FNWireError;

    #[inline]
    fn next_element_seed<V: de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<Option<V::Value>, Self::Error> {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(seed.deserialize(&mut *self.deserializer)?))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = FNWireError;

    fn is_human_readable(&self) -> bool {
        false
    }

    de_primitive!(i8);
    de_primitive!(u8);
    de_primitive_le!(i16);
    de_primitive_le!(u16);
    de_primitive_le!(i32);
    de_primitive_le!(u32);
    de_primitive_le!(i64);
    de_primitive_le!(u64);
    de_primitive_le!(i128);
    de_primitive_le!(u128);
    de_primitive_le!(f32);
    de_primitive_le!(f64);

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.input.read_u8()? {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            v => Err(FNWireError::BadValue(v as u64, "bool")),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(FNWireError::UnsupportedType("char")) // TODO: How to deserialize single utf8 char?
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let array_len = self.input.read_u32::<LE>()? as usize;
        if self.input.len() < array_len {
            Err(FNWireError::Eof)
        } else {
            let (buf, rem) = self.input.split_at(array_len);
            self.input = rem;
            visitor.visit_borrowed_bytes(buf)
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let str_len = self.input.read_u32::<LE>()? as usize;
        if self.input.len() < str_len {
            Err(FNWireError::Eof)
        } else {
            let (buf, rem) = self.input.split_at(str_len);
            self.input = rem;
            visitor.visit_borrowed_str(std::str::from_utf8(buf)?)
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.input.read_u8()? {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            v => Err(FNWireError::BadValue(v as u64, "option discriminant")),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = self.input.read_u32::<LE>()? as usize;
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len,
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(FNWireError::UnsupportedType("map"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(FNWireError::NotSelfDescribing)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(FNWireError::NotSelfDescribing)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(FNWireError::NotSelfDescribing)
    }
}

impl<'de> de::VariantAccess<'de> for &mut Deserializer<'de> {
    type Error = FNWireError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

impl<'de> de::EnumAccess<'de> for &mut Deserializer<'de> {
    type Error = FNWireError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let discrim = self.input.read_u32::<LE>()?;
        let v = seed.deserialize(de::value::U32Deserializer::<Self::Error>::new(discrim))?;
        Ok((v, self))
    }
}
