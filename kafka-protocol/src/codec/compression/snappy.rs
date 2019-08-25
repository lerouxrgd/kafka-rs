use std::io::{Error, ErrorKind};

use snap::{Decoder, Encoder};

const JAVA_MAGIC: &'static [u8] = &[0x82, b'S', b'N', b'A', b'P', b'P', b'Y', 0];

macro_rules! err(
    ($($arg:tt)*) => (Err(Error::new(ErrorKind::InvalidData, format!($($arg)*))))
);

pub fn decompress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    if !src.starts_with(JAVA_MAGIC) {
        Decoder::new().decompress_vec(src).map_err(Error::from)
    } else {
        // See SnappyInputStream.java in https://github.com/xerial/snappy-java

        ensure(12, src)?;
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&src[8..12]);
        let version = i32::from_be_bytes(bytes);

        if version != 1 {
            return err!("Unsupported snappy-java codec version {:?}", version);
        }
        // &src[12..16] is the "compatible version"; ignore for now

        let mut decompressed = vec![];

        let mut i = 16;
        while i < src.len() {
            ensure(i + 4, src)?;
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&src[i..i + 4]);
            let n = i32::from_be_bytes(bytes) as usize;
            i += 4;

            ensure(i + n, src)?;
            let chunk = Decoder::new().decompress_vec(&src[i..i + n])?;
            decompressed.extend(chunk);
            i += n;
        }

        Ok(decompressed)
    }
}

pub fn compress(src: &[u8]) -> std::io::Result<Vec<u8>> {
    Encoder::new().compress_vec(src).map_err(Error::from)
}

fn ensure(size: usize, slice: &[u8]) -> std::io::Result<()> {
    if slice.len() < size {
        err!("Not enough bytes to decompress snappy")
    } else {
        Ok(())
    }
}
