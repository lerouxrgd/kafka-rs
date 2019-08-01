use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;
use std::{fmt, io};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
};

use crate::codec::error::{Error, Result};
use crate::model::HeaderResponse;
use crate::types::{
    Batch, Bytes, HeaderRecord, NullableBytes, NullableString,
    Varint, Varlong,
};

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
    let (a, b) = decode_resp::<T>(&mut bytes, version)?;
    Ok((a, b))
}

pub fn decode_resp<'a, T>(input: &'a [u8], version: usize) -> Result<(HeaderResponse, T)>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(input, version);

    let header = HeaderResponse::deserialize(&mut deserializer)?;
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

#[derive(Debug)]
pub struct Deserializer<'de> {
    pub(crate) input: &'de [u8],
    pub(crate) identifiers: Vec<&'de str>,
    pub(crate) struct_variant: usize,
}

trait Versioned {
    fn version(&self) -> usize;
}

impl<'de, 'a> Versioned for &'a mut Deserializer<'de> {
    fn version(&self) -> usize {
        self.struct_variant
    }
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8], version: usize) -> Self {
        Deserializer {
            input,
            identifiers: vec![],
            struct_variant: version,
        }
    }

    pub fn get_rest(&self) -> &[u8] {
        self.input
    }

    /// 0 -> Record::Batch
    /// 1 -> Record::Control
    fn record_variant(&self) -> usize {
        // TODO: return Result<usize, de:Error> ?

        // RecordBatch first bytes are:
        //  base_offset: i64,
        //  batch_length: i32,
        //  partition_leader_epoch: i32,
        //  magic: i8,
        //  crc: i32,
        //  attributes: i16,
        //  ...
        // and the part of attributes we're interested in is in the first byte
        // so we end up with this formula to find the byte position:
        let byte_pos = (64 + 32 + 32 + 8 + 32 + 16) / 8;

        if self.input.len() < byte_pos {
            return 0;
        }

        ((self.input[byte_pos - 1] >> 5) & 1) as usize
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

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // TODO: not needed ?
        if self.input.len() < 1 {
            return Err(de::Error::custom("not enough bytes to deserialize u8"));
        }
        let (val, rest) = self.input.split_at(1);
        self.input = rest;
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(val);
        visitor.visit_u8(u8::from_be_bytes(bytes))
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
        println!("rest {:?}", rest);
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

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
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
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if name == "RecordBatch" {
            let curr_version = self.struct_variant;
            self.struct_variant = self.record_variant();
            let res = visitor.visit_map(StructDeserializer::new(&mut self, fields));
            self.struct_variant = curr_version;
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

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok(val)
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
        println!("youhou");
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

        impl Consumed for NullableBytesVisitor {
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

        deserializer.deserialize_bytes(NullableStringVisitor {
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
        if bytes.len() < 1 {
            return Err(de::Error::custom(
                "not enough bytes to deserialize varint (i32)",
            ));
        }
        let mut rdr = std::io::Cursor::new(bytes);
        let (i, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
        *self.nb_read.borrow_mut() = nb_read;
        Ok(Varint(i))
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

impl<'de> Deserialize<'de> for Varlong {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Varlong, D::Error>
    where
        D: de::Deserializer<'de>,
    {
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
                if bytes.len() < 1 {
                    return Err(de::Error::custom(
                        "not enough bytes to deserialize varlong (i64)",
                    ));
                }
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


impl<'de> Deserialize<'de> for Batch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Batch, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct BatchVisitor {
            nb_read: Rc<RefCell<usize>>,
        }

        impl Consumed for BatchVisitor {
            fn consumed(&self) -> Rc<RefCell<usize>> {
                self.nb_read.clone()
            }
        }

        impl<'de> Visitor<'de> for BatchVisitor {
            type Value = Batch;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a Batch")
            }

            fn visit_bytes<E>(self, bytes: &[u8]) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                // println!("input bytes: {:?}", bytes);

                if bytes.len() < 1 {
                    return Err(de::Error::custom("not enough bytes to deserialize Batch"));
                }

                let mut rdr = std::io::Cursor::new(bytes);
                let (length, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                // println!("length: {:?}", length);
                // println!("nb_read: {:?}", nb_read);
                // println!("rdr: {:?}", rdr);

                if bytes.len() < (length as usize + nb_read) {
                    return Err(de::Error::custom("not enough bytes to deserialize Batch"));
                }

                let mut buf = [0u8; 1];
                rdr.read_exact(&mut buf); // TODO: handle error
                let attributes = i8::from_be_bytes(buf);
                // println!("attributes: {:?}", attributes);

                let (timestamp_delta, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                // println!("timestamp_delta: {:?}", timestamp_delta);
                // println!("nb_read: {:?}", nb_read);

                let (offset_delta, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                // println!("offset_delta: {:?}", offset_delta);
                // println!("nb_read: {:?}", nb_read);

                let (key_length, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                // println!("key_length: {:?}", key_length);
                // println!("nb_read: {:?}", nb_read);

                let mut key: Vec<u8> = Vec::new(); 
                if key_length != -1 {
                    for _i in 0..key_length  {
                        let mut buf = [0u8; 1];
                        rdr.read_exact(&mut buf); // TODO: handle error
                        key.push(buf[0]);
                    }
                    // println!("key: {:?}", key);
                }

                let (value_len, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                // println!("value_len: {:?}", value_len);
                // println!("nb_read: {:?}", nb_read);

                let mut value: Vec<u8> = Vec::new(); 
                if value_len != -1 {
                    for _i in 0..value_len  {
                        let mut buf = [0u8; 1];
                        rdr.read_exact(&mut buf); // TODO: handle error
                        value.push(buf[0]);
                    }
                    // println!("value: {:?}", value);
                }

                let (header_len, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                // println!("header_len: {:?}", header_len);
                // println!("nb_read: {:?}", nb_read);

                let mut headers: Vec<HeaderRecord> = Vec::new(); 
                if header_len != -1 {
                    for _i in 0..header_len  {
                        let (header_key_length, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;

                        let mut header_key = String::from("");
                        if header_key_length != -1 {
                            let mut buf = vec![0u8; header_key_length as usize];
                            rdr.read_exact(&mut buf); // TODO: handle error
                            header_key = String::from_utf8(buf).map_err(de::Error::custom)?;
                        }

                        let (header_value_length, nb_read) = zag_i32(&mut rdr).map_err(de::Error::custom)?;
                        let mut header_value: Vec<u8> = Vec::new();
                        if header_value_length != -1 {
                            for _i in 0..header_value_length {
                                let mut buf = [0u8; 1];
                                rdr.read_exact(&mut buf); // TODO: handle error
                                header_value.push(buf[0]);
                            }
                        }

                        headers.push(HeaderRecord {
                            key_length: Varint(header_key_length),
                            key: header_key,
                            value_length: Varint(header_value_length),
                            value: header_value,
                        });
                    }
                    // println!("headers: {:?}", headers);
                }

                // println!("read: {:?}", length as usize + nb_read);
                *self.nb_read.borrow_mut() = length as usize + nb_read;

                Ok(Batch {
                    length: Varint(length),
                    attributes,
                    timestamp_delta: Varint(timestamp_delta),
                    offset_delta: Varint(offset_delta),
                    key_length: Varint(key_length),
                    key,
                    value_len: Varint(value_len),
                    value,
                    header_len: Varint(header_len),
                    headers,
                })
            }
        }

        deserializer.deserialize_bytes(BatchVisitor {
            nb_read: Rc::new(RefCell::new(0)),
        })

    }
}
