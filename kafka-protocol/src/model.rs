// All this will be generated by `spec-parser`

///  Numeric codes used to specify request types.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i16)]
pub enum ApiKey {
    Produce = 0,
    Fetch = 1,
    ListOffsets = 2,
    Metadata = 3,
    LeaderAndIsr = 4,
    StopReplica = 5,
    UpdateMetadata = 6,
    ControlledShutdown = 7,
    OffsetCommit = 8,
    OffsetFetch = 9,
    FindCoordinator = 10,
    JoinGroup = 11,
    Heartbeat = 12,
    LeaveGroup = 13,
    SyncGroup = 14,
    DescribeGroups = 15,
    ListGroups = 16,
    SaslHandshake = 17,
    ApiVersions = 18,
    CreateTopics = 19,
    DeleteTopics = 20,
    DeleteRecords = 21,
    InitProducerId = 22,
    OffsetForLeaderEpoch = 23,
    AddPartitionsToTxn = 24,
    AddOffsetsToTxn = 25,
    EndTxn = 26,
    WriteTxnMarkers = 27,
    TxnOffsetCommit = 28,
    DescribeAcls = 29,
    CreateAcls = 30,
    DeleteAcls = 31,
    DescribeConfigs = 32,
    AlterConfigs = 33,
    AlterReplicaLogDirs = 34,
    DescribeLogDirs = 35,
    SaslAuthenticate = 36,
    CreatePartitions = 37,
    CreateDelegationToken = 38,
    RenewDelegationToken = 39,
    ExpireDelegationToken = 40,
    DescribeDelegationToken = 41,
    DeleteGroups = 42,
}

#[derive(Debug, serde::Serialize)]
pub struct HeaderRequest {
    pub api_key: ApiKey,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: crate::types::NullableString,
}

#[derive(Debug, serde::Deserialize)]
pub struct HeaderResponse {
    pub correlation: i32,
}

#[derive(Debug, serde::Deserialize)]
pub enum ApiVersionsResponse {
    V0 {
        error_code: i16,
        api_versions: Vec<api_versions_response::v0::ApiVersion>,
    },
}

pub mod api_versions_response {
    pub mod v0 {
        #[derive(Debug, serde::Deserialize)]
        pub struct ApiVersion {
            pub api_key: crate::model::ApiKey,
            pub min_version: i16,
            pub max_version: i16,
        }
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsRequest {
    V0 {
        create_topic_requests: Vec<create_topic_request::v0::CreateTopicsRequests>,
        timeout: i32,
    },
}

pub mod create_topic_request {
    pub mod v0 {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreateTopicsRequests {
            pub topic: String,
            pub num_partitions: i32,
            pub replication_factor: i16,
            pub replica_assignment: Vec<ReplicaAssignment>,
            pub config_entries: Vec<ConfigEntries>,
        }

        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaAssignment {
            pub partition: i32,
            pub replicas: Vec<i32>,
        }

        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            pub config_name: String,
            pub config_value: crate::types::NullableString,
        }
    }
}
