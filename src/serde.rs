use std::io::prelude::*;
use std::{error, fmt, io};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use serde::ser::{self, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        source.into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Serializer {
    buf: Vec<u8>,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer { buf: vec![0; 4] }
    }

    pub fn bytes(mut self) -> Vec<u8> {
        let size = self.buf.len() as i32 - 4;
        self.buf.splice(..4, (&size.to_be_bytes()).iter().cloned());
        self.buf
    }
}

pub fn to_bytes<T: Serialize>(val: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    val.serialize(&mut serializer)?;
    Ok(serializer.bytes())
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, val: bool) -> Result<()> {
        let val = val as i8;
        val.serialize(self)?;
        Ok(())
    }

    fn serialize_i8(self, val: i8) -> Result<()> {
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i16(self, val: i16) -> Result<()> {
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, val: i32) -> Result<()> {
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, val: i64) -> Result<()> {
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, _: u8) -> Result<()> {
        Err(ser::Error::custom("Not part of Kafka binary protocol"))
    }

    fn serialize_u16(self, _: u16) -> Result<()> {
        Err(ser::Error::custom("Not part of Kafka binary protocol"))
    }

    fn serialize_u32(self, val: u32) -> Result<()> {
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, val: &str) -> Result<()> {
        let size = val.len() as i16;
        self.buf.write(&size.to_be_bytes())?;
        self.buf.write_all(val.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        let size = -1 as i16;
        size.serialize(self)?;
        Ok(())
    }

    fn serialize_some<T>(self, val: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        index: u32,
        variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, val: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        val: &T,
    ) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        val.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

// TODO: read header somehow
pub fn from_bytes<'a, T>(input: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(input);

    use crate::protocol::HeaderResponse;
    let header = HeaderResponse::deserialize(&mut deserializer)?;
    println!("{:?}", header);

    let mut deserializer = Deserializer::from_bytes(deserializer.input);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.len() == 0 {
        Ok(t)
    } else {
        // Err(de::Error::custom("Bytes remaining"))
        println!("----> Bytes remaining {:?}", deserializer.input);
        Ok(t)
    }
}

pub struct Deserializer<'de> {
    input: &'de [u8],
    identifiers: Vec<&'de str>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            identifiers: vec![],
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (val, rest) = self.input.split_at(1);
        self.input = rest;
        let val = match val[0] {
            0u8 => false,
            1u8 => true,
            _ => return Err(de::Error::custom("")),
        };
        visitor.visit_bool(val)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (val, rest) = self.input.split_at(1);
        self.input = rest;
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(val);
        visitor.visit_i8(i8::from_be_bytes(bytes))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (val, rest) = self.input.split_at(2);
        self.input = rest;
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(val);
        visitor.visit_i16(i16::from_be_bytes(bytes))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (val, rest) = self.input.split_at(4);
        self.input = rest;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(val);
        visitor.visit_i32(i32::from_be_bytes(bytes))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (val, rest) = self.input.split_at(8);
        self.input = rest;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(val);
        visitor.visit_i64(i64::from_be_bytes(bytes))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_str(s)
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (val, rest) = self.input.split_at(4);
        self.input = rest;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(val);
        let len = i32::from_be_bytes(bytes);
        visitor.visit_seq(SeqDeserializer::new(&mut self, len))
    }

    fn deserialize_tuple<V>(self, _: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        mut self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(StructDeserializer::new(&mut self, fields))
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(identifier) = self.identifiers.pop() {
            visitor.visit_borrowed_str(identifier)
        } else {
            Err(de::Error::custom("No identifiers left on the stack"))
        }
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct SeqDeserializer<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: i32,
}

impl<'a, 'de> SeqDeserializer<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: i32) -> Self {
        SeqDeserializer { de, len }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqDeserializer<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len = self.len - 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct StructDeserializer<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    fields: &'static [&'static str],
    i: usize,
}

impl<'a, 'de> StructDeserializer<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, fields: &'static [&'static str]) -> Self {
        StructDeserializer {
            de,
            fields,
            i: fields.len(),
        }
    }
}

impl<'de, 'a> MapAccess<'de> for StructDeserializer<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.i > 0 {
            self.i = self.i - 1;
            self.de.identifiers.push(self.fields[self.i]);
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::*;

    #[test]
    fn test_ser() {
        let header = HeaderRequest {
            api_key: 18,
            api_version: 0,
            correlation_id: 42,
            client_id: None,
        };
        let bytes = to_bytes(&header).unwrap();
        // [0, 0, 0, 10, 0, 18, 0, 0, 0, 0, 0, 42, 255, 255]
        println!("{:?}", bytes);
    }

    #[test]
    fn test_de() {
        let bytes = [
            0, 0, 1, 12, 0, 0, 0, 42, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 5, 0, 1, 0, 0, 0, 7, 0, 2,
            0, 0, 0, 2, 0, 3, 0, 0, 0, 5, 0, 4, 0, 0, 0, 1, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 0, 4, 0,
            7, 0, 0, 0, 1, 0, 8, 0, 0, 0, 3, 0, 9, 0, 0, 0, 3, 0, 10, 0, 0, 0, 1, 0, 11, 0, 0, 0,
            2, 0, 12, 0, 0, 0, 1, 0, 13, 0, 0, 0, 1, 0, 14, 0, 0, 0, 1, 0, 15, 0, 0, 0, 1, 0, 16,
            0, 0, 0, 1, 0, 17, 0, 0, 0, 1, 0, 18, 0, 0, 0, 1, 0, 19, 0, 0, 0, 2, 0, 20, 0, 0, 0, 1,
            0, 21, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 0,
            0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
            30, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 32, 0, 0, 0, 1, 0, 33, 0, 0, 0, 0, 0, 34, 0, 0,
            0, 0, 0, 35, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0,
            39, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 42, 0, 0, 0,
        ];
        let val = from_bytes::<ApiVersionsResponse>(&bytes[4..]).unwrap();
        println!("{:?}", val);
    }
}
