use std::io::prelude::*;
use std::net::TcpStream;

use kafka_protocol::codec::*;
use kafka_protocol::model::*;
use kafka_protocol::types::*;

fn wip_requests() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9092")?;

    let header = HeaderRequest {
        api_key: ApiKey::ApiVersions,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };
    let bytes = encode_req(&header).unwrap();
    stream.write(&bytes)?;

    let (header, resp) =
        read_resp::<_, HeaderResponse, ApiVersionsResponse>(&mut stream, Some(0)).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    Ok(())
}

fn main() {
    wip_requests().unwrap();
}
