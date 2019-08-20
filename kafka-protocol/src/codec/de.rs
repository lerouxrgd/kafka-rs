use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;
use std::{fmt, io};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};

use crate::codec::compression::Compression;
use crate::codec::error::{Error, Result};
use crate::model::HeaderResponse;
use crate::types::*;

pub fn read_resp<R, T>(rdr: &mut R, version: usize) -> Result<(HeaderResponse, T)>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut buf = [0u8; 4];
    rdr.read_exact(&mut buf)?;
    let size = i32::from_be_bytes(buf);
    let mut bytes = vec![0; size as usize];
    rdr.read_exact(&mut bytes)?;
    decode_resp::<T>(&bytes, version)
}

pub fn decode_resp<'a, T>(input: &'a [u8], version: usize) -> Result<(HeaderResponse, T)>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(input, version);

    let header = HeaderResponse::deserialize(&mut deserializer)?;
    let resp = T::deserialize(&mut deserializer)?;

    if deserializer.len() == 0 {
        Ok((header, resp))
    } else {
        Err(de::Error::custom(format!(
            "{} bytes remaining",
            deserializer.len()
        )))
    }
}

#[derive(Debug)]
pub struct Deserializer<'b, 'de: 'b> {
    input: Rc<RefCell<&'b [u8]>>,
    identifiers: Vec<&'de str>,
    struct_variant: usize,
    record_attributes: Option<Attributes>,
}

impl<'b, 'de> Deserializer<'b, 'de> {
    pub fn from_bytes(input: &'de [u8], version: usize) -> Self {
        Deserializer {
            input: Rc::new(RefCell::new(input)),
            identifiers: vec![],
            struct_variant: version,
            record_attributes: None,
        }
    }

    pub fn len(&self) -> usize {
        self.input.borrow().len()
    }

    fn peek_attributes(&mut self) -> Result<Attributes> {
        // pub struct RecordBatch {
        //     pub base_offset: i64,
        //     pub batch_length: i32,
        //     pub partition_leader_epoch: i32,
        //     pub magic: i8,
        //     pub crc: u32,
        //     pub attributes: i16,
        //     pub last_offset_delta: i32,
        //     pub first_timestamp: i64,
        //     pub max_timestamp: i64,
        //     pub producer_id: i64,
        //     pub producer_epoch: i16,
        //     pub base_sequence: i32,
        //     pub records_len: i32,
        //     pub records: Records,
        // }

        // Find `batch_length` first byte position and read it from current raw input
        let batch_len_pos = (64 + 32) / 8;
        ensure(batch_len_pos, "batch_length", *self.input.borrow())?;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.input.borrow()[batch_len_pos - 4..batch_len_pos]);
        let records_size = (i32::from_be_bytes(bytes)
            - (32 + 8 + 32 + 16 + 32 + 64 + 64 + 64 + 16 + 32 + 32) / 8)
            as usize;

        // Find `attributes` first byte position and read it from current raw input
        let attr_pos = (8 * batch_len_pos + 32 + 8 + 32 + 16) / 8;
        ensure(attr_pos, "attributes", *self.input.borrow())?;
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.input.borrow()[attr_pos - 2..attr_pos]);
        let attributes = i16::from_be_bytes(bytes);

        // Find `records_len` first byte position and read it from current raw input
        let rec_len_pos = (8 * attr_pos + 32 + 64 + 64 + 64 + 16 + 32 + 32) / 8;
        ensure(rec_len_pos, "records_len", *self.input.borrow())?;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.input.borrow()[rec_len_pos - 4..rec_len_pos]);
        let records_len = i32::from_be_bytes(bytes);

        let is_control = ((attributes >> 5) & 1) == 1; // attributes bit 5 == 1
        let compression = Compression::from_attr(attributes);

        Ok(Attributes {
            is_control,
            compression,
            records_len,
            records_size,
        })
    }
}

#[derive(Debug)]
struct Attributes {
    compression: Compression,
    is_control: bool,
    records_len: i32,
    records_size: usize,
}

trait DeserializerExt<'b> {
    fn record_attributes(&self) -> &Attributes {
        unimplemented!()
    }

    fn input(&self) -> Rc<RefCell<&'b [u8]>> {
        unimplemented!()
    }
}

impl<'b, 'de, D> DeserializerExt<'b> for D where D: de::Deserializer<'de> {}

impl<'b, 'de> DeserializerExt<'b> for &mut Deserializer<'b, 'de> {
    fn record_attributes(&self) -> &Attributes {
        self.record_attributes
            .as_ref()
            .expect("Attributes haven't been set")
    }

    fn input(&self) -> Rc<RefCell<&'b [u8]>> {
        self.input.clone()
    }
}

impl<'a, 'b, 'de> de::Deserializer<'de> for &'a mut Deserializer<'b, 'de> {
    type Error = Error;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        ensure(1, "bool", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(1);
        *self.input.borrow_mut() = rest;
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
        ensure(1, "i8", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(1);
        *self.input.borrow_mut() = rest;
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(val);
        visitor.visit_i8(i8::from_be_bytes(bytes))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        ensure(2, "i16", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(2);
        *self.input.borrow_mut() = rest;
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(val);
        visitor.visit_i16(i16::from_be_bytes(bytes))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        ensure(4, "i32", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(4);
        *self.input.borrow_mut() = rest;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(val);
        visitor.visit_i32(i32::from_be_bytes(bytes))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        ensure(8, "i64", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(8);
        *self.input.borrow_mut() = rest;
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
        ensure(4, "u32", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(4);
        *self.input.borrow_mut() = rest;
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
        ensure(2, "string size (i16)", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(2);
        *self.input.borrow_mut() = rest;

        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(val);
        let size = i16::from_be_bytes(bytes) as usize;

        ensure(size, "string", *self.input.borrow())?;
        let (val, rest) = self.input.borrow().split_at(size);
        *self.input.borrow_mut() = rest;

        let val = String::from_utf8(val.to_vec())?;
        visitor.visit_string(val)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let c = visitor.consumed();
        let val = visitor.visit_bytes(*self.input.borrow());
        let (_, rest) = self.input.borrow().split_at(*c.borrow());
        *self.input.borrow_mut() = rest;
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

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
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
        if self.record_attributes.is_some() {
            let len = self.record_attributes().records_len;
            visitor.visit_seq(SeqDeserializer::new(&mut self, len))
        } else {
            ensure(4, "seq size (i32)", *self.input.borrow())?;
            let (val, rest) = self.input.borrow().split_at(4);
            *self.input.borrow_mut() = rest;
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(val);
            let len = i32::from_be_bytes(bytes);

            visitor.visit_seq(SeqDeserializer::new(&mut self, len))
        }
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
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if name == "RecordBatch" {
            self.record_attributes = Some(self.peek_attributes()?);
            let res = visitor.visit_map(StructDeserializer::new(&mut self, fields));
            self.record_attributes = None;
            res
        } else {
            visitor.visit_map(StructDeserializer::new(&mut self, fields))
        }
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
        let variant = variants
            .get(self.struct_variant)
            .ok_or_else::<Error, _>(|| {
                de::Error::custom(format!(
                    "no variant {} within {:?}",
                    self.struct_variant, variants
                ))
            })?;

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

struct SeqDeserializer<'a, 'b, 'de: 'a + 'b> {
    de: &'a mut Deserializer<'b, 'de>,
    len: i32,
}

impl<'a, 'b, 'de> SeqDeserializer<'a, 'b, 'de> {
    fn new(de: &'a mut Deserializer<'b, 'de>, len: i32) -> Self {
        SeqDeserializer { de, len }
    }
}

impl<'a, 'b, 'de> SeqAccess<'de> for SeqDeserializer<'a, 'b, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct StructDeserializer<'a, 'b, 'de: 'a + 'b> {
    de: &'a mut Deserializer<'b, 'de>,
    fields: &'static [&'static str],
    i: usize,
}

impl<'a, 'b, 'de> StructDeserializer<'a, 'b, 'de> {
    fn new(de: &'a mut Deserializer<'b, 'de>, fields: &'static [&'static str]) -> Self {
        StructDeserializer { de, fields, i: 0 }
    }
}

impl<'a, 'b, 'de> MapAccess<'de> for StructDeserializer<'a, 'b, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.i < self.fields.len() {
            self.de.identifiers.push(self.fields[self.i]);
            self.i += 1;
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

#[derive(Debug)]
struct Enum<'a, 'b, 'de: 'a + 'b> {
    de: &'a mut Deserializer<'b, 'de>,
    variant: &'static str,
}

impl<'a, 'b, 'de> Enum<'a, 'b, 'de> {
    fn new(de: &'a mut Deserializer<'b, 'de>, variant: &'static str) -> Self {
        Enum { de, variant }
    }
}

impl<'a, 'b, 'de> EnumAccess<'de> for Enum<'a, 'b, 'de> {
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

impl<'a, 'b, 'de> VariantAccess<'de> for Enum<'a, 'b, 'de> {
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

trait VisitorExt {
    fn consumed(&self) -> Rc<RefCell<usize>> {
        unimplemented!()
    }
}

impl<'de, V: Visitor<'de>> VisitorExt for V {}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Bytes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct BytesVisitor {
            nb_read: Rc<RefCell<usize>>,
        }

        impl VisitorExt for BytesVisitor {
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
                ensure(4, "byte buf size (i32)", bytes).map_err(de::Error::custom)?;
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[..4]);
                let size = i32::from_be_bytes(buf) as usize;

                ensure(size + 4, "byte buf", bytes).map_err(de::Error::custom)?;
                let mut buf = vec![0u8; size];
                buf.copy_from_slice(&bytes[4..size + 4]);
                *self.nb_read.borrow_mut() = size + 4;

                Ok(Bytes(buf))
            }
        }

        deserializer.deserialize_bytes(BytesVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

impl<'de> Deserialize<'de> for NullableBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<NullableBytes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct NullableBytesVisitor {
            nb_read: Rc<RefCell<usize>>,
        }

        impl VisitorExt for NullableBytesVisitor {
            fn consumed(&self) -> Rc<RefCell<usize>> {
                self.nb_read.clone()
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
                ensure(4, "byte buf size (i32)", bytes).map_err(de::Error::custom)?;
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[..4]);
                let size = i32::from_be_bytes(buf);

                if size == -1 {
                    *self.nb_read.borrow_mut() = 4;
                    Ok(NullableBytes(None))
                } else {
                    let size = size as usize;

                    ensure(size + 4, "byte buf", bytes).map_err(de::Error::custom)?;
                    let mut buf = vec![0u8; size];
                    buf.copy_from_slice(&bytes[4..size + 4]);
                    *self.nb_read.borrow_mut() = size + 4;

                    Ok(NullableBytes(Some(buf)))
                }
            }
        }

        deserializer.deserialize_bytes(NullableBytesVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

impl<'de> Deserialize<'de> for NullableString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<NullableString, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct NullableStringVisitor {
            nb_read: Rc<RefCell<usize>>,
        }

        impl VisitorExt for NullableStringVisitor {
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
                ensure(2, "nullable str size (i16)", bytes).map_err(de::Error::custom)?;
                let mut buf = [0u8; 2];
                buf.copy_from_slice(&bytes[..2]);
                let size = i16::from_be_bytes(buf);

                if size == -1 {
                    *self.nb_read.borrow_mut() = 2;
                    return Ok(NullableString(None));
                }

                let size = size as usize;

                ensure(size + 2, "nullable str", bytes).map_err(de::Error::custom)?;
                let mut buf = vec![0u8; size];
                buf.copy_from_slice(&bytes[2..size + 2]);
                *self.nb_read.borrow_mut() = size + 2;
                let val = String::from_utf8(buf).map_err(de::Error::custom)?;

                Ok(NullableString(Some(val)))
            }
        }

        deserializer.deserialize_bytes(NullableStringVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

impl<'de> Deserialize<'de> for Varint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Varint, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct VarintVisitor {
            nb_read: Rc<RefCell<usize>>,
        }

        impl VisitorExt for VarintVisitor {
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
                ensure(1, "varint (i32)", bytes).map_err(de::Error::custom)?;
                let mut rdr = std::io::Cursor::new(bytes);
                let (i, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                *self.nb_read.borrow_mut() = nb_read;
                Ok(Varint(i))
            }
        }

        deserializer.deserialize_bytes(VarintVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

impl<'de> Deserialize<'de> for Varlong {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Varlong, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct VarlongVisitor {
            nb_read: Rc<RefCell<usize>>,
        }

        impl VisitorExt for VarlongVisitor {
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
                ensure(1, "varlong (i64)", bytes).map_err(de::Error::custom)?;
                let mut rdr = std::io::Cursor::new(bytes);
                let (i, nb_read) = zag_i64(&mut rdr).map_err(de::Error::custom)?;
                *self.nb_read.borrow_mut() = nb_read;
                Ok(Varlong(i))
            }
        }

        deserializer.deserialize_bytes(VarlongVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })
    }
}

pub(crate) fn zag_i32<R: Read>(reader: &mut R) -> Result<(i32, usize)> {
    let (i, nb_read) = zag_i64(reader)?;
    if i < i64::from(i32::min_value()) || i > i64::from(i32::max_value()) {
        Err(de::Error::custom("int out of range"))
    } else {
        Ok((i as i32, nb_read))
    }
}

pub(crate) fn zag_i64<R: Read>(reader: &mut R) -> Result<(i64, usize)> {
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

impl<'de> Deserialize<'de> for Records {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Records, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct RecordsVisitor;

        impl<'de> Visitor<'de> for RecordsVisitor {
            type Value = Records;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "potentially compressed records")
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Records, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut records = vec![];
                while let Some(record) = seq.next_element()? {
                    records.push(record);
                }
                Ok(Records(records))
            }
        }

        let compression = &deserializer.record_attributes().compression;

        if let Compression::None = compression {
            return Ok(deserializer.deserialize_seq(RecordsVisitor)?);
        }

        let size = deserializer.record_attributes().records_size;
        let input = deserializer.input();
        let bytes = *input.borrow();

        let decompressed;
        match compression {
            #[cfg(feature = "gzip")]
            Compression::Gzip => {
                use crate::codec::compression::gzip;
                decompressed = gzip::decompress(&bytes[..size]).map_err(de::Error::custom)?;
            }

            #[cfg(feature = "snappy")]
            Compression::Snappy => {
                use crate::codec::compression::snappy;
                decompressed = snappy::decompress(&bytes[..size]).map_err(de::Error::custom)?;
            }

            #[cfg(feature = "lz4")]
            Compression::Lz4 => {
                use crate::codec::compression::lz4;
                decompressed = lz4::decompress(&bytes[..size]).map_err(de::Error::custom)?;
            }

            #[cfg(feature = "zstd")]
            Compression::Zstd => {
                use crate::codec::compression::zstd;
                decompressed = zstd::decompress(&bytes[..size]).map_err(de::Error::custom)?;
            }

            _ => {
                return Err(de::Error::custom(format!(
                    "Unsupported compression format: {:?}",
                    compression
                )));
            }
        }

        *input.borrow_mut() = &decompressed;
        let res = Ok(deserializer.deserialize_seq(RecordsVisitor)?);
        *input.borrow_mut() = &bytes[size..];

        res
    }
}

struct WrapBytes<'a> {
    underlying: &'a mut Vec<u8>,
    consumed: usize,
}

impl<'de, 'a> DeserializeSeed<'de> for WrapBytes<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct WrapBytesVisitor<'a> {
            underlying: &'a mut Vec<u8>,
            nb_read: Rc<RefCell<usize>>,
        };

        impl<'a> VisitorExt for WrapBytesVisitor<'a> {
            fn consumed(&self) -> Rc<RefCell<usize>> {
                self.nb_read.clone()
            }
        }

        impl<'de, 'a> Visitor<'de> for WrapBytesVisitor<'a> {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Vec<u8>")
            }

            fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.underlying
                    .write(&bytes[..*self.nb_read.borrow()])
                    .map_err(de::Error::custom)?;
                Ok(())
            }
        }

        deserializer.deserialize_bytes(WrapBytesVisitor {
            underlying: self.underlying,
            nb_read: Rc::new(RefCell::new(self.consumed)),
        })
    }
}

impl<'de> Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Record, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.record_attributes().is_control {
            Ok(Record::Control(RecControl::deserialize(deserializer)?))
        } else {
            Ok(Record::Data(RecData::deserialize(deserializer)?))
        }
    }
}

impl<'de> Deserialize<'de> for RecData {
    fn deserialize<D>(deserializer: D) -> std::result::Result<RecData, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct RecDataVisitor;

        impl<'de> Visitor<'de> for RecDataVisitor {
            type Value = RecData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "kafka batch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let length: Option<Varint>;
                let attributes: Option<i8>;
                let timestamp_delta: Option<Varlong>;
                let offset_delta: Option<Varint>;
                let key_length: Option<Varint>;
                let key: Option<Option<Vec<u8>>>;
                let value_len: Option<Varint>;
                let value: Option<Vec<u8>>;
                let header_len: Option<Varint>;
                let headers: Option<Vec<HeaderRecord>>;

                length = map.next_value::<Varint>().map(Some)?;
                attributes = map.next_value::<i8>().map(Some)?;
                timestamp_delta = map.next_value::<Varlong>().map(Some)?;
                offset_delta = map.next_value::<Varint>().map(Some)?;

                key_length = map.next_value::<Varint>().map(Some)?;
                if **key_length.as_ref().unwrap() > -1 {
                    let mut buf = vec![];
                    let _ = map.next_value_seed(WrapBytes {
                        underlying: &mut buf,
                        consumed: **key_length.as_ref().unwrap() as usize,
                    });
                    key = Some(Some(buf));
                } else {
                    key = Some(None);
                }

                value_len = map.next_value::<Varint>().map(Some)?;
                let mut buf = vec![];
                let _ = map.next_value_seed(WrapBytes {
                    underlying: &mut buf,
                    consumed: **value_len.as_ref().unwrap() as usize,
                });
                value = Some(buf);

                header_len = map.next_value::<Varint>().map(Some)?;
                let mut buf = vec![];
                for _ in 0..**header_len.as_ref().unwrap() {
                    buf.push(map.next_value::<HeaderRecord>()?);
                }
                headers = Some(buf);

                let batch = RecData {
                    length: length.unwrap(),
                    attributes: attributes.unwrap(),
                    timestamp_delta: timestamp_delta.unwrap(),
                    offset_delta: offset_delta.unwrap(),
                    key_length: key_length.unwrap(),
                    key: key.unwrap(),
                    value_len: value_len.unwrap(),
                    value: value.unwrap(),
                    header_len: header_len.unwrap(),
                    headers: headers.unwrap(),
                };

                Ok(batch)
            }
        }

        const NAME: &'static str = "RecData";
        const FIELDS: &'static [&'static str] = &[
            "length",
            "attributes",
            "timestamp_delta",
            "offset_delta",
            "key_length",
            "key",
            "value_len",
            "value",
            "header_len",
            "headers",
        ];

        deserializer.deserialize_struct(NAME, FIELDS, RecDataVisitor)
    }
}

impl<'de> Deserialize<'de> for HeaderRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<HeaderRecord, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct HeaderRecordVisitor;

        impl<'de> Visitor<'de> for HeaderRecordVisitor {
            type Value = HeaderRecord;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "kafka record header")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let key_length: Option<Varint>;
                let key: Option<String>;
                let value_length: Option<Varint>;
                let value: Option<Option<Vec<u8>>>;

                key_length = map.next_value::<Varint>().map(Some)?;
                let mut buf = vec![];
                let _ = map.next_value_seed(WrapBytes {
                    underlying: &mut buf,
                    consumed: **key_length.as_ref().unwrap() as usize,
                });
                key = Some(String::from_utf8(buf).map_err(de::Error::custom)?);

                value_length = map.next_value::<Varint>().map(Some)?;
                if **value_length.as_ref().unwrap() > -1 {
                    let mut buf = vec![];
                    let _ = map.next_value_seed(WrapBytes {
                        underlying: &mut buf,
                        consumed: **value_length.as_ref().unwrap() as usize,
                    });

                    value = Some(Some(buf));
                } else {
                    value = Some(None);
                }

                Ok(HeaderRecord {
                    key_length: key_length.unwrap(),
                    key: key.unwrap(),
                    value_length: value_length.unwrap(),
                    value: value.unwrap(),
                })
            }
        }

        const NAME: &'static str = "HeaderRecord";
        const FIELDS: &'static [&'static str] = &["key_length", "key", "value_length", "value"];

        deserializer.deserialize_struct(NAME, FIELDS, HeaderRecordVisitor)
    }
}

fn ensure(size: usize, what: &str, slice: &[u8]) -> Result<()> {
    if slice.len() < size {
        Err(de::Error::custom(format!(
            "Not enough bytes ({:?}) to deserialize {:?} of size {:?}",
            slice.len(),
            what,
            size
        )))
    } else {
        Ok(())
    }
}
