#[cfg(feature = "gzip")]
pub mod gzip;

#[cfg(feature = "snappy")]
pub mod snappy;

#[cfg(feature = "lz4")]
pub mod lz4;

#[cfg(feature = "zstd")]
pub mod zstd;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compression {
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
    Unknown,
}

impl Compression {
    pub fn from_attr(attributes: i16) -> Self {
        match attributes & 7 {
            0 => Compression::None,
            1 => Compression::Gzip,
            2 => Compression::Snappy,
            3 => Compression::Lz4,
            4 => Compression::Zstd,
            _ => Compression::Unknown,
        }
    }
}
