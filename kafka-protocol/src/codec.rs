use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;
use std::{error, fmt, io};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};
use serde::ser::{self, Serialize};

use crate::types::{Bytes, NullableBytes, NullableString, Varint, Varlong};

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

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Self {
        source.into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Serializer {
    buf: Vec<u8>,
}

pub fn encode_req<T: Serialize>(val: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    val.serialize(&mut serializer)?;
    Ok(serializer.bytes())
}

pub fn encode_single<T: Serialize>(val: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    val.serialize(&mut serializer)?;
    Ok(serializer.buf[4..].to_vec())
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

    fn serialize_u8(self, _val: u8) -> Result<()> {
        Err(ser::Error::custom("not part of Kafka binary protocol: u8"))
    }

    fn serialize_u16(self, _val: u16) -> Result<()> {
        Err(ser::Error::custom("not part of Kafka binary protocol: u16"))
    }

    fn serialize_u32(self, val: u32) -> Result<()> {
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, _val: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, _val: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _val: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _val: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, val: &str) -> Result<()> {
        if val.len() > std::i16::MAX as usize {
            return Err(ser::Error::custom(format!(
                "str slice is too long: {}",
                val.len()
            )));
        }

        let size = val.len() as i16;
        self.buf.write(&size.to_be_bytes())?;
        self.buf.write_all(val.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, val: &[u8]) -> Result<()> {
        self.buf.write(val)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(ser::Error::custom(
            "invalid none, use a dedicated wrapper type",
        ))
    }

    fn serialize_some<T>(self, _val: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        Err(ser::Error::custom(
            "invalid some, use a dedicated wrapper type",
        ))
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _val: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _val: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _val: &T,
    ) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            None => Err(ser::Error::custom("seq length must be known")),
            Some(len) => {
                if len > std::i32::MAX as usize {
                    Err(ser::Error::custom(format!("seq is too long: {}", len)))
                } else {
                    let size = len as i32;
                    self.buf.write(&size.to_be_bytes())?;
                    Ok(self)
                }
            }
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        val.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _val: &T) -> Result<()>
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

    fn serialize_field<T>(&mut self, _val: &T) -> Result<()>
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

    fn serialize_field<T>(&mut self, _val: &T) -> Result<()>
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

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _val: &T) -> Result<()>
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

    fn serialize_field<T>(&mut self, _name: &'static str, val: &T) -> Result<()>
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

    fn serialize_field<T>(&mut self, _name: &'static str, val: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        val.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.0.len() > std::i32::MAX as usize {
            return Err(ser::Error::custom(format!(
                "byte buf is too long: {}",
                self.0.len()
            )));
        }

        let size = self.0.len() as i32;
        let mut buf = Vec::with_capacity(size as usize + 4);
        buf.write(&size.to_be_bytes()).map_err(ser::Error::custom)?;
        buf.write(&self.0).map_err(ser::Error::custom)?;
        serializer.serialize_bytes(&buf)
    }
}

impl Serialize for NullableBytes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match &self.0 {
            None => serializer.serialize_i32(-1),
            Some(val) => {
                if val.len() > std::i32::MAX as usize {
                    return Err(ser::Error::custom(format!(
                        "byte buf is too long: {}",
                        val.len()
                    )));
                }

                let size = val.len() as i32;
                let mut buf = Vec::with_capacity(size as usize + 4);
                buf.write(&size.to_be_bytes()).map_err(ser::Error::custom)?;
                buf.write(&val).map_err(ser::Error::custom)?;
                serializer.serialize_bytes(&buf)
            }
        }
    }
}

impl Serialize for NullableString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match &self.0 {
            None => serializer.serialize_i16(-1),
            Some(val) => serializer.serialize_str(&val),
        }
    }
}

impl Serialize for Varint {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut buf = vec![];
        zig_i32(self.0, &mut buf);
        serializer.serialize_bytes(&buf)
    }
}

impl Serialize for Varlong {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut buf = vec![];
        zig_i64(self.0, &mut buf);
        serializer.serialize_bytes(&buf)
    }
}

fn zig_i32(n: i32, buf: &mut Vec<u8>) {
    zig_i64(n as i64, buf)
}

fn zig_i64(n: i64, buf: &mut Vec<u8>) {
    encode_variable(((n << 1) ^ (n >> 63)) as u64, buf)
}

fn encode_variable(mut z: u64, buf: &mut Vec<u8>) {
    loop {
        if z <= 0x7F {
            buf.push((z & 0x7F) as u8);
            break;
        } else {
            buf.push((0x80 | (z & 0x7F)) as u8);
            z >>= 7;
        }
    }
}

pub fn read_resp<R, H, T>(rdr: &mut R, version: Option<usize>) -> Result<(H, T)>
where
    R: io::Read,
    H: de::DeserializeOwned,
    T: de::DeserializeOwned,
{
    let mut buf = [0u8; 4];
    rdr.read_exact(&mut buf)?;
    let size = i32::from_be_bytes(buf);
    let mut bytes = vec![0; size as usize];
    rdr.read_exact(&mut bytes)?;
    decode_resp::<H, T>(&bytes, version)
}

pub fn decode_resp<'a, H, T>(input: &'a [u8], version: Option<usize>) -> Result<(H, T)>
where
    H: Deserialize<'a>,
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(input, version);

    let header = H::deserialize(&mut deserializer)?;
    let resp = T::deserialize(&mut deserializer)?;

    if deserializer.input.len() == 0 {
        Ok((header, resp))
    } else {
        Err(de::Error::custom(format!(
            "{} bytes remaining",
            deserializer.input.len()
        )))
    }
}

pub fn read_single<R, T>(rdr: &mut R, version: Option<usize>) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut buf = [0u8; 4];
    rdr.read_exact(&mut buf)?;
    let size = i32::from_be_bytes(buf);
    let mut bytes = vec![0; size as usize];
    rdr.read_exact(&mut bytes)?;
    decode_single::<T>(&bytes, version)
}

pub fn decode_single<'a, T>(input: &'a [u8], version: Option<usize>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(input, version);
    let resp = T::deserialize(&mut deserializer)?;

    if deserializer.input.len() == 0 {
        Ok(resp)
    } else {
        Err(de::Error::custom(format!(
            "{} bytes remaining",
            deserializer.input.len()
        )))
    }
}

pub struct Deserializer<'de> {
    input: &'de [u8],
    identifiers: Vec<&'de str>,
    version: Option<usize>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8], version: Option<usize>) -> Self {
        Deserializer {
            input,
            identifiers: vec![],
            version,
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.len() < 1 {
            return Err(de::Error::custom("not enough bytes to deserialize bool"));
        }
        let (val, rest) = self.input.split_at(1);
        self.input = rest;
        let val = match val[0] {
            0u8 => false,
            1u8 => true,
            _ => return Err(de::Error::custom("not a boolean")),
        };
        visitor.visit_bool(val)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.len() < 1 {
            return Err(de::Error::custom("not enough bytes to deserialize i8"));
        }
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
        if self.input.len() < 2 {
            return Err(de::Error::custom("not enough bytes to deserialize i16"));
        }
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
        if self.input.len() < 4 {
            return Err(de::Error::custom("not enough bytes to deserialize i32"));
        }
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
        if self.input.len() < 8 {
            return Err(de::Error::custom("not enough bytes to deserialize i64"));
        }
        let (val, rest) = self.input.split_at(8);
        self.input = rest;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(val);
        visitor.visit_i64(i64::from_be_bytes(bytes))
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.len() < 4 {
            return Err(de::Error::custom("not enough bytes to deserialize u32"));
        }
        let (val, rest) = self.input.split_at(4);
        self.input = rest;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(val);
        visitor.visit_u32(u32::from_be_bytes(bytes))
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
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

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.len() < 2 {
            return Err(de::Error::custom(
                "not enough bytes to deserialize string size (i16)",
            ));
        }
        let (val, rest) = self.input.split_at(2);
        self.input = rest;

        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(val);
        let size = i16::from_be_bytes(bytes) as usize;

        if self.input.len() < size {
            return Err(de::Error::custom(format!(
                "not enough bytes ({}) to deserialize string of length {}",
                self.input.len(),
                size
            )));
        }
        let (val, rest) = self.input.split_at(size);
        self.input = rest;

        let val = String::from_utf8(val.to_vec())?;
        visitor.visit_string(val)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let c = visitor.consumed();
        let val = visitor.visit_bytes(self.input);
        let (_, rest) = self.input.split_at(*c.borrow());
        self.input = rest;
        val
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.len() < 4 {
            return Err(de::Error::custom(
                "not enough bytes to deserialize seq size (i32)",
            ));
        }
        let (val, rest) = self.input.split_at(4);
        self.input = rest;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(val);
        let len = i32::from_be_bytes(bytes);
        visitor.visit_seq(SeqDeserializer::new(&mut self, len))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
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
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let variant = match self.version {
            Some(i) => variants.get(i).ok_or_else::<Error, _>(|| {
                de::Error::custom(format!("no variant {} within {:?}", i, variants))
            }),
            _ => Err(de::Error::custom(format!(
                "invalid variant version: {:?}",
                self.version
            ))),
        }?;
        let value = visitor.visit_enum(Enum::new(self, variant))?;
        Ok(value)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(identifier) = self.identifiers.pop() {
            visitor.visit_borrowed_str(identifier)
        } else {
            Err(de::Error::custom("no identifiers left on the stack"))
        }
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
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
        StructDeserializer { de, fields, i: 0 }
    }
}

impl<'de, 'a> MapAccess<'de> for StructDeserializer<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.i < self.fields.len() {
            self.de.identifiers.push(self.fields[self.i]);
            self.i = self.i + 1;
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

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    variant: &'static str,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, variant: &'static str) -> Self {
        Enum { de, variant }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        self.de.identifiers.push(self.variant);
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        unimplemented!()
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        unimplemented!()
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        for field in fields {
            self.de.identifiers.push(field);
        }
        de::Deserializer::deserialize_struct(self.de, self.variant, fields, visitor)
    }
}

trait Consumed {
    fn consumed(&self) -> Rc<RefCell<usize>>;
}

impl<'de, T: Visitor<'de>> Consumed for T {
    default fn consumed(&self) -> Rc<RefCell<usize>> {
        Rc::new(RefCell::new(0))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Bytes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BytesVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

struct BytesVisitor {
    nb_read: Rc<RefCell<usize>>,
}

impl Consumed for BytesVisitor {
    fn consumed(&self) -> Rc<RefCell<usize>> {
        self.nb_read.clone()
    }
}

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "kafka bytes")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        if bytes.len() < 4 {
            return Err(de::Error::custom(
                "not enough bytes to deserialize byte buf size (i32)",
            ));
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&bytes[..4]);
        let size = i32::from_be_bytes(buf);

        let size = size as usize;
        if bytes.len() < size + 4 {
            return Err(de::Error::custom(format!(
                "not enough bytes to deserialize byte buf of length {} + 4",
                size
            )));
        }

        let mut buf = vec![0u8; size];
        buf.copy_from_slice(&bytes[4..size + 4]);
        *self.nb_read.borrow_mut() = size + 4;

        Ok(Bytes(buf))
    }
}

impl<'de> Deserialize<'de> for NullableBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<NullableBytes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(NullableBytesVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

struct NullableBytesVisitor {
    nb_read: Rc<RefCell<usize>>,
}

impl Consumed for NullableBytesVisitor {
    fn consumed(&self) -> Rc<RefCell<usize>> {
        self.nb_read.clone()
    }
}

impl<'de> Deserialize<'de> for NullableString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<NullableString, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(NullableStringVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

impl<'de> Visitor<'de> for NullableBytesVisitor {
    type Value = NullableBytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "kafka bytes")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        if bytes.len() < 4 {
            return Err(de::Error::custom(
                "not enough bytes to deserialize byte buf size (i32)",
            ));
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&bytes[..4]);
        let size = i32::from_be_bytes(buf);

        if size == -1 {
            *self.nb_read.borrow_mut() = 4;
            Ok(NullableBytes(None))
        } else {
            let size = size as usize;
            if bytes.len() < size + 4 {
                return Err(de::Error::custom(format!(
                    "not enough bytes to deserialize byte buf of length {} + 4",
                    size
                )));
            }

            let mut buf = vec![0u8; size];
            buf.copy_from_slice(&bytes[4..size + 4]);
            *self.nb_read.borrow_mut() = size + 4;

            Ok(NullableBytes(Some(buf)))
        }
    }
}

struct NullableStringVisitor {
    nb_read: Rc<RefCell<usize>>,
}

impl Consumed for NullableStringVisitor {
    fn consumed(&self) -> Rc<RefCell<usize>> {
        self.nb_read.clone()
    }
}

impl<'de> Visitor<'de> for NullableStringVisitor {
    type Value = NullableString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a kafka nullable string")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        if bytes.len() < 2 {
            return Err(de::Error::custom(
                "not enough bytes to deserialize nullable str size (i16)",
            ));
        }
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&bytes[..2]);
        let size = i16::from_be_bytes(buf);

        if size == -1 {
            *self.nb_read.borrow_mut() = 2;
            return Ok(NullableString(None));
        }

        let size = size as usize;
        if bytes.len() < size + 2 {
            return Err(de::Error::custom(format!(
                "not enough bytes to deserialize nullable str of length {} + 2",
                size
            )));
        }

        let mut buf = vec![0u8; size];
        buf.copy_from_slice(&bytes[2..size + 2]);
        *self.nb_read.borrow_mut() = size + 2;
        let val = String::from_utf8(buf).map_err(de::Error::custom)?;

        Ok(NullableString(Some(val)))
    }
}

impl<'de> Deserialize<'de> for Varint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Varint, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(VarintVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

struct VarintVisitor {
    nb_read: Rc<RefCell<usize>>,
}

impl Consumed for VarintVisitor {
    fn consumed(&self) -> Rc<RefCell<usize>> {
        self.nb_read.clone()
    }
}

impl<'de> Visitor<'de> for VarintVisitor {
    type Value = Varint;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a zigzag encoded variable length i32")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut rdr = std::io::Cursor::new(bytes);
        let (i, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
        *self.nb_read.borrow_mut() = nb_read;
        Ok(Varint(i))
    }
}

impl<'de> Deserialize<'de> for Varlong {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Varlong, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(VarlongVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

struct VarlongVisitor {
    nb_read: Rc<RefCell<usize>>,
}

impl Consumed for VarlongVisitor {
    fn consumed(&self) -> Rc<RefCell<usize>> {
        self.nb_read.clone()
    }
}

impl<'de> Visitor<'de> for VarlongVisitor {
    type Value = Varlong;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a zigzag encoded variable length i64")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut rdr = std::io::Cursor::new(bytes);
        let (i, nb_read) = zag_i64(&mut rdr).map_err(de::Error::custom)?;
        *self.nb_read.borrow_mut() = nb_read;
        Ok(Varlong(i))
    }
}

fn zag_i32<R: Read>(reader: &mut R) -> Result<(i32, usize)> {
    let (i, nb_read) = zag_i64(reader)?;
    if i < i64::from(i32::min_value()) || i > i64::from(i32::max_value()) {
        Err(de::Error::custom("int out of range"))
    } else {
        Ok((i as i32, nb_read))
    }
}

fn zag_i64<R: Read>(reader: &mut R) -> Result<(i64, usize)> {
    let (z, nb_read) = decode_variable(reader)?;
    Ok(if z & 0x1 == 0 {
        ((z >> 1) as i64, nb_read)
    } else {
        (!(z >> 1) as i64, nb_read)
    })
}

fn decode_variable<R: Read>(reader: &mut R) -> Result<(u64, usize)> {
    let mut i = 0u64;
    let mut buf = [0u8; 1];

    let mut j = 0;
    loop {
        if j > 9 {
            // if j * 7 > 64
            return Err(de::Error::custom(
                "overflow when decoding zigzag integer value",
            ));
        }
        reader.read_exact(&mut buf[..])?;
        i |= (u64::from(buf[0] & 0x7F)) << (j * 7);
        j += 1;
        if (buf[0] >> 7) == 0 {
            break;
        }
    }

    Ok((i, j))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use crate::types::*;
    use std::io::Cursor;

    #[test]
    fn serde_bool() {
        let v1 = true;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<bool>(&bytes, None).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn serde_integers() {
        let v1 = 13 as i8;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i8>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as i16;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i16>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as i32;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i32>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as i64;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<i64>(&bytes, None).unwrap();
        assert_eq!(v1, v2);

        let v1 = 13 as u32;
        let bytes = encode_single(&v1).unwrap();
        let v2 = decode_single::<u32>(&bytes, None).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn serde_varint_varlong() {
        let i: i32 = 3;
        let mut bytes = vec![];
        zig_i32(i, &mut bytes);
        let mut rdr = Cursor::new(bytes);
        let (j, varint_size) = zag_i32(&mut rdr).unwrap();
        assert_eq!(i, j);
        assert_eq!(1, varint_size);

        let i = Varint(3);
        let bytes = encode_single(&i).unwrap();
        let j = decode_single::<Varint>(&bytes, None).unwrap();
        assert_eq!(i, j);

        let i = Varlong(-3);
        let bytes = encode_single(&i).unwrap();
        let j = decode_single::<Varlong>(&bytes, None).unwrap();
        assert_eq!(i, j);
    }

    #[test]
    fn serde_strings() {
        let s1 = String::from("yes");
        let bytes = encode_single(&s1).unwrap();
        let s2 = decode_single::<String>(&bytes, None).unwrap();
        assert_eq!(s1, s2);

        let s1 = NullableString::from("yes");
        let bytes = encode_single(&s1).unwrap();
        let s2 = decode_single::<NullableString>(&bytes, None).unwrap();
        assert_eq!(s1, s2);

        let s1 = NullableString(None);
        let bytes = encode_single(&s1).unwrap();
        let s2 = decode_single::<NullableString>(&bytes, None).unwrap();
        assert_eq!(s1, s2);
    }

    #[test]
    fn serde_bytes() {
        let b1 = Bytes(vec![1, 2, 3]);
        let bytes = encode_single(&b1).unwrap();
        let b2 = decode_single::<Bytes>(&bytes, None).unwrap();
        assert_eq!(b1, b2);

        let b1 = NullableBytes::from(vec![1, 2, 3]);
        let bytes = encode_single(&b1).unwrap();
        let b2 = decode_single::<NullableBytes>(&bytes, None).unwrap();
        assert_eq!(b1, b2);

        let b1 = NullableBytes(None);
        let bytes = encode_single(&b1).unwrap();
        let b2 = decode_single::<NullableBytes>(&bytes, None).unwrap();
        assert_eq!(b1, b2);
    }

    #[test]
    fn ser_req() {
        let header = HeaderRequest {
            api_key: ApiKey::ApiVersions,
            api_version: 0,
            correlation_id: 42,
            client_id: NullableString(None),
        };
        let bytes = encode_req(&header).unwrap();
        assert_eq!(vec![0, 0, 0, 10, 0, 18, 0, 0, 0, 0, 0, 42, 255, 255], bytes);
    }

    #[test]
    fn de_resp() {
        let mut bytes = Cursor::new(vec![
            0, 0, 1, 12, 0, 0, 0, 42, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 7, 0, 1, 0, 0, 0, 10, 0, 2,
            0, 0, 0, 4, 0, 3, 0, 0, 0, 7, 0, 4, 0, 0, 0, 1, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 0, 4, 0,
            7, 0, 0, 0, 1, 0, 8, 0, 0, 0, 6, 0, 9, 0, 0, 0, 5, 0, 10, 0, 0, 0, 2, 0, 11, 0, 0, 0,
            3, 0, 12, 0, 0, 0, 2, 0, 13, 0, 0, 0, 2, 0, 14, 0, 0, 0, 2, 0, 15, 0, 0, 0, 2, 0, 16,
            0, 0, 0, 2, 0, 17, 0, 0, 0, 1, 0, 18, 0, 0, 0, 2, 0, 19, 0, 0, 0, 3, 0, 20, 0, 0, 0, 3,
            0, 21, 0, 0, 0, 1, 0, 22, 0, 0, 0, 1, 0, 23, 0, 0, 0, 2, 0, 24, 0, 0, 0, 1, 0, 25, 0,
            0, 0, 1, 0, 26, 0, 0, 0, 1, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 2, 0, 29, 0, 0, 0, 1, 0,
            30, 0, 0, 0, 1, 0, 31, 0, 0, 0, 1, 0, 32, 0, 0, 0, 2, 0, 33, 0, 0, 0, 1, 0, 34, 0, 0,
            0, 1, 0, 35, 0, 0, 0, 1, 0, 36, 0, 0, 0, 0, 0, 37, 0, 0, 0, 1, 0, 38, 0, 0, 0, 1, 0,
            39, 0, 0, 0, 1, 0, 40, 0, 0, 0, 1, 0, 41, 0, 0, 0, 1, 0, 42, 0, 0, 0, 1,
        ]);

        let (header, resp) =
            read_resp::<_, HeaderResponse, ApiVersionsResponse>(&mut bytes, Some(0)).unwrap();
        println!("{:?}", header);
        println!("{:?}", resp);
    }
}
