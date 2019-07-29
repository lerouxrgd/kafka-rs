use std::io::prelude::*;
use std::net::TcpStream;

use kafka_protocol::codec::{encode_req, read_resp, Deserializer};
use kafka_protocol::model::*;
use kafka_protocol::types::{NullableBytes, NullableString, RecordBatch, Record, Batch};

// TODO: handle decoding empty response (server error) which leads to SO on the client

fn wip_requests() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9092")?;

    // let header = HeaderRequest {
    //     api_key: ApiKey::ApiVersions,
    //     api_version: 0,
    //     correlation_id: 42,
    //     client_id: NullableString::from("me"),
    // };
    // let bytes = encode_req(&header, &ApiVersionsRequest::V0 {}).unwrap();
    // stream.write(&bytes)?;

    // let (header, resp) = read_resp::<_, ApiVersionsResponse>(&mut stream, 0).unwrap();
    // // println!("---> {:?}", header);
    // // println!("---> {:?}", resp);

    ///////////////////////////////////////////////////////////////////

    // let header = HeaderRequest {
    //     api_key: ApiKey::Metadata,
    //     api_version: 0,
    //     correlation_id: 42,
    //     client_id: NullableString::from("me"),
    // };

    // let req = MetadataRequest::V0 {
    //     topics: vec![metadata_request::v0::Topics {
    //         name: "test".to_owned(),
    //     }],
    // };

    // let bytes = encode_req(&header, &req).unwrap();
    // stream.write(&bytes)?;

    // let (header, resp) = read_resp::<_, MetadataResponse>(&mut stream, 0).unwrap();
    // // println!("---> {:?}", header);
    // // println!("---> {:?}", resp);

    ///////////////////////////////////////////////////////////////////

    let header = HeaderRequest {
        api_key: ApiKey::Fetch,
        api_version: 7,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };

    let req = FetchRequest::V7 {
        replica_id: -1,
        max_wait_time: 5 * 1000,
        min_bytes: 1,
        max_bytes: 5 * 1024 * 1024,
        isolation_level: 0,
        session_id: 0,
        session_epoch: 0,

        topics: vec![fetch_request::v7::Topics {
            topic: "test".into(),
            partitions: vec![fetch_request::v7::Partitions {
                partition: 0,
                fetch_offset: 0,
                log_start_offset: 0,
                partition_max_bytes: 5 * 1024 * 1024,
            }],
        }],
        forgotten_topics_data: vec![fetch_request::v7::ForgottenTopicsData {
            topic: "".into(),
            partitions: vec![0],
        }],
    };

    let bytes = encode_req(&header, &req).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, FetchResponse>(&mut stream, 7).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    if let FetchResponse::V7 { throttle_time_ms, error_code, session_id, responses } = resp {
        if let NullableBytes(bytes) = &responses
            .get(0)
            .unwrap()
            .partition_responses
            .get(0)
            .unwrap()
            .record_set
        {
            use serde::Deserialize;
            println!("stream len={:?}", bytes.as_ref().unwrap().len() * 8);
            println!("stream={:?}\n", bytes.as_ref().unwrap());
            let mut deserializer = Deserializer::from_bytes(bytes.as_ref().unwrap(), 0);
            let resp = RecordBatch::deserialize(&mut deserializer).unwrap();
            println!(">>>>>>>> {:?}", resp);

            match &resp.records[0] {
                Record::Batch(Batch { length, attributes, value, timestamp_delta, offset_delta, key_length, key, value_len, header_len, headers }) => println!("{:?}", String::from_utf8(value.to_vec())),
                _ => println!("Nothing")
            }

            let rest = deserializer.get_rest();
            let mut deserializer = Deserializer::from_bytes(rest, 0);
            let resp = RecordBatch::deserialize(&mut deserializer).unwrap();
            println!("\n>>>>>>>> {:?}", resp);

            match &resp.records[0] {
                Record::Batch(Batch { length, attributes, value, timestamp_delta, offset_delta, key_length, key, value_len, header_len, headers }) => println!("{:?}", String::from_utf8(value.to_vec())),
                _ => println!("Nothing")
            }
        }
    }
    // TODO: crc check
    // TODO: check that recordBatch.batch_length matches actual record length
    // TODO: Take into account RecordBatch compression information in attributes to deser records

    Ok(())
}

fn main() {
    wip_requests().unwrap();
}
