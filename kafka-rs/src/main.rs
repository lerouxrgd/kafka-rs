use std::io::prelude::*;
use std::net::TcpStream;

use kafka_protocol::codec::{encode_req, read_resp};
use kafka_protocol::model::*;
use kafka_protocol::types::NullableString;

fn wip_requests() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9092")?;

    let header = HeaderRequest {
        api_key: ApiKey::ApiVersions,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };
    let bytes = encode_req(&header, &ApiVersionsRequest::V0 {}).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, ApiVersionsResponse>(&mut stream, 0).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    use kafka_protocol::model::metadata_request::v0::Topics;

    let header = HeaderRequest {
        api_key: ApiKey::Metadata,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };

    let req = MetadataRequest::V0 {
        topics: vec![Topics {
            name: "test".to_owned(),
        }],
    };

    let bytes = encode_req(&header, &req).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, MetadataResponse>(&mut stream, 0).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    Ok(())
}

fn main() {
    wip_requests().unwrap();
}
