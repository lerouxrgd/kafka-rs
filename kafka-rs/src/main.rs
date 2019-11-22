mod acc;
mod req;

use std::io::prelude::*;

use async_std::{io::BufReader, net::TcpStream, prelude::*, task};
use kafka_protocol::{
    codec::{self, decode_resp, encode_req, Compression, Deserializer, Serializer},
    model::*,
    types::*,
};

pub async fn read_resp<T>(
    stream: &mut TcpStream,
    version: usize,
) -> codec::Result<(HeaderResponse, T)>
where
    T: serde::de::DeserializeOwned,
{
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await?;
    let size = i32::from_be_bytes(buf);
    let mut bytes = vec![0; size as usize];
    stream.read_exact(&mut bytes).await?;
    decode_resp::<T>(&bytes, version)
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn main_async() -> Result<()> {
    let addr = "127.0.0.1:9092";
    let mut stream = TcpStream::connect(addr).await?;

    let header = HeaderRequest {
        api_key: ApiKey::ApiVersions,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };
    let bytes = encode_req(&header, &ApiVersionsRequest::V0 {})?;

    stream.write_all(&bytes).await?;

    let (header, resp) = read_resp::<ApiVersionsResponse>(&mut stream, 0).await?;
    println!("---> {:?}", header);
    println!("---> {:?}", resp);
    Ok(())
}

fn main() -> Result<()> {
    task::block_on(main_async())
}
