#![feature(specialization)]

mod codec;
mod model;
mod types;

use std::io::prelude::*;
use std::net::TcpStream;

fn wip_requests() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9092")?;

    use crate::codec::*;
    use crate::model::*;
    use crate::types::*;

    let header = HeaderRequest {
        api_key: 18,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };
    let bytes = to_bytes(&header).unwrap();
    stream.write(&bytes)?;

    let (header, resp) =
        from_reader::<_, HeaderResponse, ApiVersionsResponse>(&mut stream).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    Ok(())
}

fn main() {
    wip_requests().unwrap();
}
