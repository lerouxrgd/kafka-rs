use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Varint(pub i32);

impl Deref for Varint {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Varlong(pub i64);

impl Deref for Varlong {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(pub Vec<u8>);

impl Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
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
    pub records: Vec<Record>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum Record {
    Batch(Batch),
    Control(Control),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct Batch {
    pub length: Varint,
    pub attributes: i8,
    pub timestamp_delta: Varint,
    pub offset_delta: Varint,
    pub key_length: Varint,
    pub key: Vec<u8>,
    pub value_len: Varint,
    pub value: Vec<u8>,
    pub header_len: Varint,
    pub headers: Vec<HeaderRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct HeaderRecord {
    pub key_length: Varint,
    pub key: String,
    pub value_length: Varint,
    pub value: Vec<u8>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Control {
    pub version: i16,
    pub r#type: i16,
}

impl RecordBatch {
    pub fn compression(&self) -> Compression {
        match self.attributes & 7 {
            0 => Compression::NoCompression,
            1 => Compression::Gzip,
            2 => Compression::Snappy,
            3 => Compression::Lz4,
            4 => Compression::Zstd,
            _ => Compression::Unknown,
        }
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

    pub fn is_control_batch(&self) -> bool {
        match (self.attributes >> 5) & 1 {
            0 => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compression {
    NoCompression,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimestampType {
    CreateTime,
    LogAppendTime,
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
