use std::io::prelude::*;

use arrayvec::ArrayVec;
use serde::ser::{self, Serialize};

use crate::codec::compression::Compression;
use crate::codec::error::{Error, Result};
use crate::model::HeaderRequest;
use crate::types::*;

pub fn encode_req<T: Serialize>(header: &HeaderRequest, val: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    header.serialize(&mut serializer)?;
    val.serialize(&mut serializer)?;
    Ok(serializer.bytes())
}

pub struct Serializer {
    pub(crate) buf: Vec<u8>,
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

    fn serialize_u8(self, val: u8) -> Result<()> {
        // TODO: not needed ?
        self.buf.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u16(self, _val: u16) -> Result<()> {
        unimplemented!()
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
        Ok(())
    }

    fn serialize_some<T>(self, val: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(&mut *self)?;
        Ok(())
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

    fn serialize_newtype_struct<T>(self, _name: &'static str, val: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        // TODO: not needed ?
        val.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        val: &T,
    ) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            None => Ok(self),
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
        let mut buf = ArrayVec::<[u8; Varint::MAX_BYTES]>::new();
        zig_i32(self.0, &mut buf).map_err(ser::Error::custom)?;
        serializer.serialize_bytes(&buf)
    }
}

impl Serialize for Varlong {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut buf = ArrayVec::<[u8; Varlong::MAX_BYTES]>::new();
        zig_i64(self.0, &mut buf).map_err(ser::Error::custom)?;
        serializer.serialize_bytes(&buf)
    }
}

pub(crate) fn zig_i32(n: i32, buf: impl Write) -> std::io::Result<usize> {
    zig_i64(n as i64, buf)
}

pub(crate) fn zig_i64(n: i64, buf: impl Write) -> std::io::Result<usize> {
    encode_variable(((n << 1) ^ (n >> 63)) as u64, buf)
}

fn encode_variable(mut z: u64, mut buf: impl Write) -> std::io::Result<usize> {
    loop {
        if z <= 0x7F {
            return buf.write(&[(z & 0x7F) as u8]);
        } else {
            buf.write(&[(0x80 | (z & 0x7F)) as u8])?;
            z >>= 7;
        }
    }
}

impl Serialize for RecData {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use ser::Error;

        // TODO: alternative to reversing everthing like push_front or something...
        let mut s = Serializer { buf: vec![] };

        for header in self.headers.iter() {
            header.serialize(&mut s).map_err(Error::custom)?;
        }
        self.header_len.serialize(&mut s).map_err(Error::custom)?;

        serde_bytes::Serialize::serialize(&self.value, &mut s).map_err(Error::custom)?;
        self.value_len.serialize(&mut s).map_err(Error::custom)?;

        if *self.key_length > -1 {
            if self.key.is_none() {
                return Err(Error::custom(
                    "RecData `key_length` is > -1 but `key` is None",
                ));
            }
            serde_bytes::Serialize::serialize(&self.key.as_ref().unwrap(), &mut s)
                .map_err(Error::custom)?;
            self.key.serialize(&mut s).map_err(Error::custom)?;
        }
        self.key_length.serialize(&mut s).map_err(Error::custom)?;

        self.offset_delta.serialize(&mut s).map_err(Error::custom)?;
        self.timestamp_delta
            .serialize(&mut s)
            .map_err(Error::custom)?;
        self.attributes.serialize(&mut s).map_err(Error::custom)?;

        let length = Varint(s.buf.len() as i32);
        length.serialize(&mut s).map_err(Error::custom)?;

        let mut bytes = s.buf;
        bytes.reverse();
        serializer.serialize_bytes(&bytes)
    }
}

impl Serialize for RecordBatch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use ser::Error;

        let mut s = Serializer { buf: vec![] };

        for record in self.records.iter() {
            record.serialize(&mut s).map_err(Error::custom)?;
        }

        match self.compression() {
            Compression::None => (),

            #[cfg(feature = "gzip")]
            Compression::Gzip => {
                use crate::codec::compression::gzip;
                s.buf = gzip::compress(&s.buf).map_err(Error::custom)?;
            }

            _ => unimplemented!(), // TODO: other compression formats
        }

        self.records_len.serialize(&mut s).map_err(Error::custom)?;

        self.base_sequence
            .serialize(&mut s)
            .map_err(Error::custom)?;
        self.producer_epoch
            .serialize(&mut s)
            .map_err(Error::custom)?;
        self.producer_id.serialize(&mut s).map_err(Error::custom)?;

        self.max_timestamp
            .serialize(&mut s)
            .map_err(Error::custom)?;
        self.first_timestamp
            .serialize(&mut s)
            .map_err(Error::custom)?;
        self.last_offset_delta
            .serialize(&mut s)
            .map_err(Error::custom)?;
        self.attributes.serialize(&mut s).map_err(Error::custom)?;

        // TODO: calculate crc
        self.crc.serialize(&mut s).map_err(Error::custom)?;

        self.magic.serialize(&mut s).map_err(Error::custom)?;
        self.partition_leader_epoch
            .serialize(&mut s)
            .map_err(Error::custom)?;

        let batch_length = s.buf.len() as i32;
        batch_length.serialize(&mut s).map_err(Error::custom)?;

        self.base_offset.serialize(&mut s).map_err(Error::custom)?;

        let mut bytes = s.buf;
        bytes.reverse();
        serializer.serialize_bytes(&bytes)
    }
}

pub(crate) fn ser_raw_string<S>(
    string: &String,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_bytes(string.as_bytes())
}

pub(crate) fn ser_option_bytes<S>(
    bytes: &Option<Vec<u8>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    if let Some(bytes) = bytes {
        serializer.serialize_bytes(bytes)
    } else {
        serializer.serialize_none()
    }
}
