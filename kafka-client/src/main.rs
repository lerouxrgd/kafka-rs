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
    let bytes = encode_req::<HeaderRequest>(&header, None).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, ApiVersionsResponse>(&mut stream, Some(0)).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    let header = HeaderRequest {
        api_key: ApiKey::Metadata,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };
    let req = MetadataRequest::V0 {
        topics: vec!["test".to_owned()],
    };
    let bytes = encode_req(&header, Some(&req)).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, MetadataResponse>(&mut stream, Some(0)).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    Ok(())
}

fn main() {
    wip_requests().unwrap();
}
