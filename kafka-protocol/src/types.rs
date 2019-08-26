use std::ops::{Deref, DerefMut};

use crate::codec::ser::{ser_option_bytes, ser_raw_string};
use crate::codec::Compression;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NullableString(pub Option<String>);

impl NullableString {
    pub fn from(s: &str) -> Self {
        NullableString(Some(s.to_string()))
    }
}

impl Deref for NullableString {
    type Target = Option<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Varint(pub i32);

impl Varint {
    pub const MAX_BYTES: usize = 5;

    #[allow(overflowing_literals)]
    pub fn size_of(val: i32) -> usize {
        let mut v: i32 = (val << 1) ^ (val >> 31);
        let mut bytes: i32 = 1;
        while (v & 0xffffff80) != 0 {
            bytes += 1;
            v >>= 7;
        }
        return bytes as usize;
    }

    pub fn size(&self) -> usize {
        Varint::size_of(self.0)
    }
}

impl Deref for Varint {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Varint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Varlong(pub i64);

impl Varlong {
    pub const MAX_BYTES: usize = 10;

    #[allow(overflowing_literals)]
    pub fn size_of(val: i64) -> usize {
        let mut v: i64 = (val << 1) ^ (val >> 63);
        let mut bytes: i64 = 1;
        while (v & 0xffffffffffffff80) != 0 {
            bytes += 1;
            v >>= 7;
        }
        return bytes as usize;
    }

    pub fn size(&self) -> usize {
        Varlong::size_of(self.0)
    }
}

impl Deref for Varlong {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Varlong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bytes(pub Vec<u8>);

impl Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NullableBytes(pub Option<Vec<u8>>);

impl NullableBytes {
    pub fn from(b: Vec<u8>) -> Self {
        NullableBytes(Some(b))
    }
}

impl Deref for NullableBytes {
    type Target = Option<Vec<u8>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, serde::Deserialize)]
pub struct RecordBatch {
    pub base_offset: i64,
    pub(crate) batch_length: i32,
    pub partition_leader_epoch: i32,
    pub(crate) magic: i8,
    pub(crate) crc: u32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub first_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub(crate) records_len: i32,
    pub(crate) records: Records,
}

impl RecordBatch {
    /// Size of fields [partition_leader_epoch ..records_len]
    pub(crate) const INNER_SIZE: usize = (32 + 8 + 32 + 16 + 32 + 64 + 64 + 64 + 16 + 32 + 32) / 8;

    pub fn builder() -> RecordBatchBuilder {
        RecordBatchBuilder::new()
    }

    pub fn compression(&self) -> Compression {
        Compression::from_attr(self.attributes)
    }

    pub fn timestamp_type(&self) -> TimestampType {
        match (self.attributes >> 3) & 1 {
            0 => TimestampType::CreateTime,
            _ => TimestampType::LogAppendTime,
        }
    }

    pub fn is_transactional(&self) -> bool {
        match (self.attributes >> 4) & 1 {
            0 => false,
            _ => true,
        }
    }

    pub fn is_control(&self) -> bool {
        match (self.attributes >> 5) & 1 {
            0 => false,
            _ => true,
        }
    }

    pub fn raw_size(&self) -> usize {
        let records_size: usize = self.iter().map(|rec| rec.size()).sum();
        (64 + 32) / 8 + RecordBatch::INNER_SIZE + records_size
    }

    pub fn len(&self) -> usize {
        self.records_len as usize
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = RecData> {
        self.records.0.into_iter().filter_map(|rec| {
            if let Record::Data(rec) = rec {
                Some(rec)
            } else {
                None
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &RecData> {
        self.records.0.iter().filter_map(|rec| {
            if let Record::Data(ref rec) = rec {
                Some(rec)
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Records(pub Vec<Record>);

impl Deref for Records {
    type Target = Vec<Record>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Records {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum Record {
    Data(RecData),
    Control(RecControl),
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct RecControl {
    pub version: i16,
    pub r#type: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RecData {
    pub(crate) length: Varint,
    pub attributes: i8,
    pub(crate) timestamp_delta: Varlong,
    pub(crate) offset_delta: Varint,
    pub(crate) key_length: Varint,
    pub key: Option<Vec<u8>>,
    pub(crate) value_len: Varint,
    pub value: Vec<u8>,
    pub(crate) header_len: Varint,
    pub headers: Vec<HeaderRecord>,
}

impl RecData {
    pub fn new(value: Vec<u8>) -> Self {
        let mut rec_data = RecData::default();
        rec_data.value_len = Varint(value.len() as i32);
        rec_data.value = value;
        rec_data.key_length = Varint(-1);
        rec_data
    }

    pub fn set_key(mut self, key: Vec<u8>) -> Self {
        self.key_length = Varint(key.len() as i32);
        self.key = Some(key);
        self
    }

    pub fn add_header(mut self, key: String, value: Option<Vec<u8>>) -> Self {
        let key_length = Varint(key.len() as i32);
        let value_length = if let Some(ref value) = value {
            Varint(value.len() as i32)
        } else {
            Varint(-1)
        };

        self.headers.push(HeaderRecord {
            key_length,
            key,
            value_length,
            value,
        });
        *self.header_len += 1;

        self
    }

    pub fn offset_delta(&self) -> i32 {
        *self.offset_delta
    }

    pub fn timestamp_delta(&self) -> i64 {
        *self.timestamp_delta
    }

    pub fn size(&self) -> usize {
        let mut size = 1 + self.timestamp_delta.size() + self.offset_delta.size();

        size += self.key_length.size();
        if let Some(ref key) = self.key {
            size += key.len();
        }

        size += self.value_len.size() + self.value.len();

        size += self.header_len.size();
        for header in self.headers.iter() {
            size += header.size();
        }

        size
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct HeaderRecord {
    pub(crate) key_length: Varint,
    #[serde(serialize_with = "ser_raw_string")]
    pub key: String,
    pub(crate) value_length: Varint,
    #[serde(serialize_with = "ser_option_bytes")]
    pub value: Option<Vec<u8>>,
}

impl HeaderRecord {
    pub fn size(&self) -> usize {
        let mut size = self.key_length.size() + self.key.len() + self.value_length.size();
        if let Some(ref value) = self.value {
            size += value.len();
        }
        size
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimestampType {
    CreateTime,
    LogAppendTime,
}

pub struct RecordBatchBuilder {
    rec_batch: RecordBatch,
}

impl RecordBatchBuilder {
    pub fn new() -> Self {
        let mut rec_batch = RecordBatch::default();
        rec_batch.magic = 2;
        rec_batch.producer_id = -1;
        rec_batch.producer_epoch = -1;
        rec_batch.base_sequence = -1;
        RecordBatchBuilder { rec_batch }
    }

    #[allow(overflowing_literals)]
    pub fn compression(mut self, compression: Compression) -> Self {
        let attr = &mut self.rec_batch.attributes;
        match compression {
            Compression::None => *attr &= 0xfff8,
            Compression::Gzip => *attr = (*attr | 0x0001) & 0xfff9,
            Compression::Snappy => *attr = (*attr | 0x0002) & 0xfffa,
            Compression::Lz4 => *attr = (*attr | 0x0003) & 0xfffb,
            Compression::Zstd => *attr = (*attr | 0x0004) & 0xfffc,
            _ => (),
        }
        self
    }

    pub fn add_record(mut self, ts: i64, mut rec: RecData) -> Self {
        if self.rec_batch.first_timestamp == 0 {
            self.rec_batch.first_timestamp = ts;
        }

        if ts > self.rec_batch.max_timestamp {
            self.rec_batch.max_timestamp = ts;
        }

        let ts_delta = Varlong((ts - self.rec_batch.first_timestamp) as i64);
        rec.timestamp_delta = ts_delta;

        rec.offset_delta = Varint(self.rec_batch.records.len() as i32);
        self.rec_batch.records.deref_mut().push(Record::Data(rec));

        self
    }

    pub fn raw_size(&self) -> usize {
        self.rec_batch.raw_size()
    }

    pub fn build(self) -> RecordBatch {
        let mut rec_batch = self.rec_batch;
        if rec_batch.records.len() > 0 {
            rec_batch.last_offset_delta = (rec_batch.records.len() - 1) as i32;
            rec_batch.records_len = rec_batch.records.len() as i32;
        }
        rec_batch
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum MessageSet {
    V0 {
        offset: i64,
        message_size: i32,
        message: message_set::v0::Message,
    },
    V1 {
        offset: i64,
        message_size: i32,
        message: message_set::v1::Message,
    },
}

pub mod message_set {
    pub mod v0 {
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
        )]
        pub struct Message {
            pub crc: u32,
            pub magic_byte: i8,
            pub attributes: i8,
            pub key: crate::types::NullableBytes,
            pub value: crate::types::NullableBytes,
        }
    }
    pub mod v1 {
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
        )]
        pub struct Message {
            pub crc: u32,
            pub magic_byte: i8,
            pub attributes: i8,
            pub timestamp: i64,
            pub key: crate::types::NullableBytes,
            pub value: crate::types::NullableBytes,
        }
    }
}
