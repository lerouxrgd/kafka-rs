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
    ElectPreferredLeaders = 43,
    IncrementalAlterConfigs = 44,
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ApiVersionsRequest {
    V0 {},
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MetadataRequest {
    V0 {
        topics: Vec<metadata_request::v0::Topics>,
    },
}

pub mod metadata_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MetadataResponse {
    V0 {
        brokers: Vec<metadata_response::v0::Brokers>,
        topics: Vec<metadata_response::v0::Topics>,
    },
}

pub mod metadata_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            pub node_id: i32,
            pub host: String,
            pub port: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub error_code: i16,
            pub name: String,
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            pub error_code: i16,
            pub partition_index: i32,
            pub leader_id: i32,
            pub replica_nodes: Vec<i32>,
            pub isr_nodes: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsRequest {
    V0 {
        topics: Vec<create_topics_request::v0::Topics>,
        timeout_ms: i32,
    },
}

pub mod create_topics_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub name: String,
            pub num_partitions: i32,
            pub replication_factor: i16,
            pub assignments: Vec<Assignments>,
            pub configs: Vec<Configs>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            pub partition_index: i32,
            pub broker_ids: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Configs {
            pub name: String,
            pub value: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsResponse {
    V0 {
        topics: Vec<create_topics_response::v0::Topics>,
    },
}

pub mod create_topics_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub name: String,
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FetchRequest {
    V0 {
        replica_id: i32,
        max_wait_time: i32,
        min_bytes: i32,
        topics: Vec<fetch_request::v0::Topics>,
    },
    V1 {
        replica_id: i32,
        max_wait_time: i32,
        min_bytes: i32,
        topics: Vec<fetch_request::v1::Topics>,
    },
    V2 {
        replica_id: i32,
        max_wait_time: i32,
        min_bytes: i32,
        topics: Vec<fetch_request::v2::Topics>,
    },
    V3 {
        replica_id: i32,
        max_wait_time: i32,
        min_bytes: i32,
        max_bytes: i32,
        topics: Vec<fetch_request::v3::Topics>,
    },
    V4 {
        replica_id: i32,
        max_wait_time: i32,
        min_bytes: i32,
        max_bytes: i32,
        isolation_level: i8,
        topics: Vec<fetch_request::v4::Topics>,
    },
    // V5 {
    //     replica_id: i32,
    //     max_wait_time: i32,
    //     min_bytes: i32,
    //     max_bytes: i32,
    //     isolation_level: i8,
    //     topics: Vec<fetch_request::v5::Topics>,
    // },
    // V6 {
    //     replica_id: i32,
    //     max_wait_time: i32,
    //     min_bytes: i32,
    //     max_bytes: i32,
    //     isolation_level: i8,
    //     topics: Vec<fetch_request::v6::Topics>,
    // },
    // V7 {
    //     replica_id: i32,
    //     max_wait_time: i32,
    //     min_bytes: i32,
    //     max_bytes: i32,
    //     isolation_level: i8,
    //     session_id: i32,
    //     session_epoch: i32,
    //     topics: Vec<fetch_request::v7::Topics>,
    //     forgotten_topics_data: Vec<fetch_request::v7::ForgottenTopicsData>,
    // },
}

pub mod fetch_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub topic: String,
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            pub partition: i32,
            pub fetch_offset: i64,
            pub partition_max_bytes: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub topic: String,
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            pub partition: i32,
            pub fetch_offset: i64,
            pub partition_max_bytes: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub topic: String,
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            pub partition: i32,
            pub fetch_offset: i64,
            pub partition_max_bytes: i32,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub topic: String,
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            pub partition: i32,
            pub fetch_offset: i64,
            pub partition_max_bytes: i32,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            pub topic: String,
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            pub partition: i32,
            pub fetch_offset: i64,
            pub partition_max_bytes: i32,
        }
    }
    // pub mod v5 {
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Topics {
    //         pub topic: String,
    //         pub partitions: Vec<Partitions>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Partitions {
    //         pub partition: i32,
    //         pub fetch_offset: i64,
    //         pub log_start_offset: i64,
    //         pub partition_max_bytes: i32,
    //     }
    // }
    // pub mod v6 {
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Topics {
    //         pub topic: String,
    //         pub partitions: Vec<Partitions>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Partitions {
    //         pub partition: i32,
    //         pub fetch_offset: i64,
    //         pub log_start_offset: i64,
    //         pub partition_max_bytes: i32,
    //     }
    // }
    // pub mod v7 {
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Topics {
    //         pub topic: String,
    //         pub partitions: Vec<Partitions>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct ForgottenTopicsData {
    //         pub topic: String,
    //         pub partitions: Vec<i32>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Partitions {
    //         pub partition: i32,
    //         pub fetch_offset: i64,
    //         pub log_start_offset: i64,
    //         pub partition_max_bytes: i32,
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FetchResponse {
    V0 {
        responses: Vec<fetch_response::v0::Responses>,
    },
    V1 {
        throttle_time_ms: i32,
        responses: Vec<fetch_response::v1::Responses>,
    },
    V2 {
        throttle_time_ms: i32,
        responses: Vec<fetch_response::v2::Responses>,
    },
    V3 {
        throttle_time_ms: i32,
        responses: Vec<fetch_response::v3::Responses>,
    },
    V4 {
        throttle_time_ms: i32,
        responses: Vec<fetch_response::v4::Responses>,
    },
    // V5 {
    //     throttle_time_ms: i32,
    //     responses: Vec<fetch_response::v5::Responses>,
    // },
    // V6 {
    //     throttle_time_ms: i32,
    //     responses: Vec<fetch_response::v6::Responses>,
    // },
    // V7 {
    //     throttle_time_ms: i32,
    //     error_code: i16,
    //     session_id: i32,
    //     responses: Vec<fetch_response::v7::Responses>,
    // },
}

pub mod fetch_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            pub topic: String,
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            pub partition_header: PartitionHeader,
            pub record_set: crate::types::NullableBytes,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            pub partition: i32,
            pub error_code: i16,
            pub high_watermark: i64,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            pub topic: String,
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            pub partition_header: PartitionHeader,
            pub record_set: crate::types::NullableBytes,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            pub partition: i32,
            pub error_code: i16,
            pub high_watermark: i64,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            pub topic: String,
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            pub partition_header: PartitionHeader,
            pub record_set: crate::types::NullableBytes,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            pub partition: i32,
            pub error_code: i16,
            pub high_watermark: i64,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            pub topic: String,
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            pub partition_header: PartitionHeader,
            pub record_set: crate::types::NullableBytes,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            pub partition: i32,
            pub error_code: i16,
            pub high_watermark: i64,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            pub topic: String,
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            pub partition_header: PartitionHeader,
            pub record_set: crate::types::NullableBytes,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            pub partition: i32,
            pub error_code: i16,
            pub high_watermark: i64,
            pub last_stable_offset: i64,
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            pub producer_id: i64,
            pub first_offset: i64,
        }
    }
    // pub mod v5 {
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Responses {
    //         pub topic: String,
    //         pub partition_responses: Vec<PartitionResponses>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct PartitionResponses {
    //         pub partition_header: PartitionHeader,
    //         pub record_set: crate::types::NullableBytes,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct PartitionHeader {
    //         pub partition: i32,
    //         pub error_code: i16,
    //         pub high_watermark: i64,
    //         pub last_stable_offset: i64,
    //         pub log_start_offset: i64,
    //         pub aborted_transactions: Vec<AbortedTransactions>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct AbortedTransactions {
    //         pub producer_id: i64,
    //         pub first_offset: i64,
    //     }
    // }
    // pub mod v6 {
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Responses {
    //         pub topic: String,
    //         pub partition_responses: Vec<PartitionResponses>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct PartitionResponses {
    //         pub partition_header: PartitionHeader,
    //         pub record_set: crate::types::NullableBytes,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct PartitionHeader {
    //         pub partition: i32,
    //         pub error_code: i16,
    //         pub high_watermark: i64,
    //         pub last_stable_offset: i64,
    //         pub log_start_offset: i64,
    //         pub aborted_transactions: Vec<AbortedTransactions>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct AbortedTransactions {
    //         pub producer_id: i64,
    //         pub first_offset: i64,
    //     }
    // }
    // pub mod v7 {
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct Responses {
    //         pub topic: String,
    //         pub partition_responses: Vec<PartitionResponses>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct PartitionResponses {
    //         pub partition_header: PartitionHeader,
    //         pub record_set: crate::types::NullableBytes,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct PartitionHeader {
    //         pub partition: i32,
    //         pub error_code: i16,
    //         pub high_watermark: i64,
    //         pub last_stable_offset: i64,
    //         pub log_start_offset: i64,
    //         pub aborted_transactions: Vec<AbortedTransactions>,
    //     }
    //     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    //     pub struct AbortedTransactions {
    //         pub producer_id: i64,
    //         pub first_offset: i64,
    //     }
    // }
}
