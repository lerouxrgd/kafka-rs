use kafka_protocol::codec::{decode_single, encode_single};
use kafka_protocol::model::{create_topics_request::v0::*, CreateTopicsRequest};
use kafka_protocol::types::*;

#[test]
fn complex_req() {
    let val1 = CreateTopicsRequest::V0 {
        create_topic_requests: vec![CreateTopicRequests {
            topic: "topic".to_owned(),
            num_partitions: 32,
            replication_factor: 16,
            replica_assignment: vec![ReplicaAssignment {
                partition: 12,
                replicas: vec![1],
            }],
            config_entries: vec![ConfigEntries {
                config_name: "default".to_owned(),
                config_value: NullableString(None),
            }],
        }],
        timeout: 0,
    };

    let bytes = encode_single(&val1).unwrap();
    let val2 = decode_single::<CreateTopicsRequest>(&bytes, Some(0)).unwrap();
    assert_eq!(val1, val2);
}
