use std::ops::{Deref, DerefMut};

use crate::codec::{ser, Compression};

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
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic: i8,
    pub crc: u32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub first_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records_len: i32,
    pub records: Records,
}

impl RecordBatch {
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, serde::Serialize)]
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
    pub length: Varint,
    pub attributes: i8,
    pub timestamp_delta: Varint,
    pub offset_delta: Varint,
    pub key_length: Varint,
    pub key: Option<Vec<u8>>,
    pub value_len: Varint,
    pub value: Vec<u8>,
    pub header_len: Varint,
    pub headers: Vec<HeaderRecord>,
}

impl RecData {
    pub fn with_val(val: Vec<u8>) -> Self {
        let mut rec_data = RecData::default();
        rec_data.value_len = Varint(val.len() as i32);
        rec_data.value = val;
        rec_data.key_length = Varint(-1);
        rec_data
    }

    pub fn set_key(mut self, key: Vec<u8>) -> Self {
        self.key_length = Varint(key.len() as i32);
        self.key = Some(key);
        self
    }

    pub fn add_header(mut self, key: String, value: Vec<u8>) -> Self {
        let key_length = Varint(key.len() as i32);
        let value_length = Varint(value.len() as i32);
        self.headers.push(HeaderRecord {
            key_length,
            key,
            value,
            value_length,
        });
        *self.header_len += 1;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct HeaderRecord {
    pub key_length: Varint,
    #[serde(serialize_with = "ser::ser_raw_string")]
    pub key: String,
    pub value_length: Varint,
    #[serde(serialize_with = "serde_bytes::serialize")]
    pub value: Vec<u8>,
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
        RecordBatchBuilder { rec_batch }
    }

    pub fn add_record(mut self, ts: i64, mut rec: RecData) -> Self {
        if self.rec_batch.first_timestamp == 0 {
            self.rec_batch.first_timestamp = ts;
        }

        if ts > self.rec_batch.max_timestamp {
            self.rec_batch.max_timestamp = ts;
        }

        let ts_delta = Varint((ts - self.rec_batch.first_timestamp) as i32);
        rec.timestamp_delta = ts_delta;

        rec.offset_delta = Varint(self.rec_batch.records.len() as i32);
        self.rec_batch.records.deref_mut().push(Record::Data(rec));

        self
    }

    pub fn compression(mut self, compression: Compression) -> Self {
        let attr = &mut self.rec_batch.attributes;
        match compression {
            Compression::None => *attr &= 32760,
            Compression::Gzip => *attr = (*attr | 1) & 32761,
            Compression::Snappy => *attr = (*attr | 2) & 32762,
            Compression::Lz4 => *attr = (*attr | 3) & 32763,
            Compression::Zstd => *attr = (*attr | 4) & 32764,
            _ => (),
        }
        self
    }

    pub fn ts_type(mut self, ts_type: TimestampType) -> Self {
        let attr = &mut self.rec_batch.attributes;
        match ts_type {
            TimestampType::CreateTime => *attr &= 32759,
            TimestampType::LogAppendTime => *attr |= 8,
        }
        self
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
