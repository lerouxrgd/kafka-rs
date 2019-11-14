mod acc;
mod req;

use std::io::prelude::*;
use std::net::TcpStream;

use kafka_protocol::codec::{self, decode_resp, encode_req, Compression, Deserializer, Serializer};
use kafka_protocol::model::*;
use kafka_protocol::types::*;

pub fn read_resp<R, T>(rdr: &mut R, version: usize) -> codec::Result<(HeaderResponse, T)>
where
    R: Read,
    T: serde::de::DeserializeOwned,
{
    let mut buf = [0u8; 4];
    rdr.read_exact(&mut buf)?;
    let size = i32::from_be_bytes(buf);
    let mut bytes = vec![0; size as usize];
    rdr.read_exact(&mut bytes)?;
    decode_resp::<T>(&bytes, version)
}

fn main() {}
