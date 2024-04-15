use byteorder::{WriteBytesExt, LE};
use paste::item;
use serde::{ser, Serialize};
use std::io::Write;

use crate::wire::FNWireError;

impl ser::Error for FNWireError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        FNWireError::Other(msg.to_string())
    }
}

#[derive(Debug)]
pub struct Serializer<W: Write> {
    out: W,
}

impl<W: Write> Serializer<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

macro_rules! ser_primitive {
    ($tname:ident) => {
        item! {
            fn [< serialize_ $tname >](self, v: [< $tname >]) -> Result<Self::Ok, Self::Error> {
                self.out.[< write_ $tname >](v)?;
                Ok(())
            }
        }
    };
}

macro_rules! ser_primitive_le {
    ($tname:ident) => {
        item! {
            fn [< serialize_ $tname >](self, v: [< $tname >]) -> Result<Self::Ok, Self::Error> {
                self.out.[< write_ $tname >]::<LE>(v)?;
                Ok(())
            }
        }
    };
}

impl<W: Write> ser::Serializer for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    type SerializeSeq = Self;
    type SerializeMap = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    ser_primitive!(i8);
    ser_primitive!(u8);
    ser_primitive_le!(i16);
    ser_primitive_le!(u16);
    ser_primitive_le!(i32);
    ser_primitive_le!(u32);
    ser_primitive_le!(i64);
    ser_primitive_le!(u64);
    ser_primitive_le!(i128);
    ser_primitive_le!(u128);
    ser_primitive_le!(f32);
    ser_primitive_le!(f64);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if v.len() > u32::MAX as usize {
            return Err(FNWireError::SizeOverflow(v.len()));
        }
        self.out.write_u32::<LE>(v.len() as u32)?;
        self.out.write(v)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.out.write_u8(match v {
            true => 1,
            false => 0,
        })?;
        Ok(())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(variant_index)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serialize_u32(variant_index)?;
        self.serialize_newtype_struct(variant, value)?;
        Ok(())
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(l) => {
                if l > u32::MAX as usize {
                    Err(FNWireError::SizeOverflow(l))
                } else {
                    self.out.write_u32::<LE>(l as u32)?;
                    Ok(self)
                }
            }
            None => Err(FNWireError::SizelessSeq),
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(FNWireError::UnsupportedType("map"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.out.write_u8(0)?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.out.write_u8(1)?;
        value.serialize(self)
    }
}

impl<W: Write> ser::SerializeSeq for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeTuple for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeTupleStruct for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeTupleVariant for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeStruct for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeStructVariant for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeMap for &mut Serializer<W> {
    type Ok = ();
    type Error = FNWireError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(FNWireError::UnsupportedType("map"))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(FNWireError::UnsupportedType("map"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(FNWireError::UnsupportedType("map"))
    }
}


