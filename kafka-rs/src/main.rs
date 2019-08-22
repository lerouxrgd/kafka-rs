use std::io::prelude::*;
use std::net::TcpStream;

use kafka_protocol::codec::{encode_req, read_resp, Compression, Deserializer, Serializer};
use kafka_protocol::model::*;
use kafka_protocol::types::*;

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
        api_version: 4,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };

    let req = FetchRequest::V4 {
        replica_id: -1,
        max_wait_time: 5 * 1000,
        min_bytes: 1,
        max_bytes: 15 * 1024 * 1024,
        isolation_level: 0,
        topics: vec![fetch_request::v4::Topics {
            topic: "test".into(),
            partitions: vec![fetch_request::v4::Partitions {
                partition: 0,
                fetch_offset: 0,
                partition_max_bytes: 5 * 1024 * 1024,
            }],
        }],
    };

    let bytes = encode_req(&header, &req).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, FetchResponse>(&mut stream, 4).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    if let FetchResponse::V4 { responses, .. } = resp {
        if let NullableBytes(Some(bytes)) = &responses
            .get(0)
            .unwrap()
            .partition_responses
            .get(0)
            .unwrap()
            .record_set
        {
            use serde::Deserialize;

            let mut deserializer = Deserializer::from_bytes(&bytes, 0);

            while deserializer.len() != 0 {
                let batch = RecordBatch::deserialize(&mut deserializer).unwrap();
                println!(">>>>>>>> {:?}", batch);

                match &(*batch.records)[0] {
                    Record::Data(RecData { value, .. }) => {
                        println!("{:?}", String::from_utf8(value.to_vec()))
                    }
                    _ => println!("Nothing"),
                }
            }
        }
    }

    // TODO: crc check when deserializing

    ///////////////////////////////////////////////////////////////////

    use chrono::Utc;
    use serde::Serialize;

    let mut serializer = Serializer::new();
    let rec_batch = RecordBatch::builder()
        // .compression(Compression::Snappy)
        .add_record(
            Utc::now().timestamp(),
            RecData::with_val(vec![99, 111, 117, 99, 111, 117])
                .set_key(vec![1, 2])
                .add_header("bob".into(), Some(vec![3])),
        )
        .build();
    println!("+++++++> {:?}", rec_batch);
    rec_batch.serialize(&mut serializer).unwrap();
    let bytes = (&serializer.bytes()[4..]).to_vec();

    let header = HeaderRequest {
        api_key: ApiKey::Produce,
        api_version: 3,
        correlation_id: 42,
        client_id: NullableString::from("me"),
    };

    let req = ProduceRequest::V3 {
        transactional_id: NullableString(None),
        acks: -1,
        timeout: 5 * 1000,
        topic_data: vec![produce_request::v3::TopicData {
            topic: "test".into(),
            data: vec![produce_request::v3::Data {
                partition: 0,
                record_set: NullableBytes::from(bytes),
                // record_set: NullableBytes(None),
            }],
        }],
    };

    let bytes = encode_req(&header, &req).unwrap();
    stream.write(&bytes)?;

    let (header, resp) = read_resp::<_, ProduceResponse>(&mut stream, 3).unwrap();
    println!("---> {:?}", header);
    println!("---> {:?}", resp);

    Ok(())
}

fn main() {
    wip_requests().unwrap();
}
