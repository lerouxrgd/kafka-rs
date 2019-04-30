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

///  Numeric codes to indicate what problem occurred on the Kafka server.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i16)]
pub enum ErrorCode {
    /// The server experienced an unexpected error when processing the request.
    /// Retriable: False.
    UnknownServerError = -1,
    /// Retriable: False.
    None = 0,
    /// The requested offset is not within the range of offsets maintained by
    /// the server. Retriable: False.
    OffsetOutOfRange = 1,
    /// This message has failed its CRC checksum, exceeds the valid size, has
    /// a null key for a compacted topic, or is otherwise corrupt. Retriable:
    /// True.
    CorruptMessage = 2,
    /// This server does not host this topic-partition. Retriable: True.
    UnknownTopicOrPartition = 3,
    /// The requested fetch size is invalid. Retriable: False.
    InvalidFetchSize = 4,
    /// There is no leader for this topic-partition as we are in the middle of
    /// a leadership election. Retriable: True.
    LeaderNotAvailable = 5,
    /// This server is not the leader for that topic-partition. Retriable:
    /// True.
    NotLeaderForPartition = 6,
    /// The request timed out. Retriable: True.
    RequestTimedOut = 7,
    /// The broker is not available. Retriable: False.
    BrokerNotAvailable = 8,
    /// The replica is not available for the requested topic-partition.
    /// Retriable: False.
    ReplicaNotAvailable = 9,
    /// The request included a message larger than the max message size the
    /// server will accept. Retriable: False.
    MessageTooLarge = 10,
    /// The controller moved to another broker. Retriable: False.
    StaleControllerEpoch = 11,
    /// The metadata field of the offset request was too large. Retriable:
    /// False.
    OffsetMetadataTooLarge = 12,
    /// The server disconnected before a response was received. Retriable:
    /// True.
    NetworkException = 13,
    /// The coordinator is loading and hence can't process requests. Retriable:
    /// True.
    CoordinatorLoadInProgress = 14,
    /// The coordinator is not available. Retriable: True.
    CoordinatorNotAvailable = 15,
    /// This is not the correct coordinator. Retriable: True.
    NotCoordinator = 16,
    /// The request attempted to perform an operation on an invalid topic.
    /// Retriable: False.
    InvalidTopicException = 17,
    /// The request included message batch larger than the configured segment
    /// size on the server. Retriable: False.
    RecordListTooLarge = 18,
    /// Messages are rejected since there are fewer in-sync replicas than
    /// required. Retriable: True.
    NotEnoughReplicas = 19,
    /// Messages are written to the log, but to fewer in-sync replicas than
    /// required. Retriable: True.
    NotEnoughReplicasAfterAppend = 20,
    /// Produce request specified an invalid value for required acks.
    /// Retriable: False.
    InvalidRequiredAcks = 21,
    /// Specified group generation id is not valid. Retriable: False.
    IllegalGeneration = 22,
    /// The group member's supported protocols are incompatible with those of
    /// existing members or first group member tried to join with empty
    /// protocol type or empty protocol list. Retriable: False.
    InconsistentGroupProtocol = 23,
    /// The configured groupId is invalid. Retriable: False.
    InvalidGroupId = 24,
    /// The coordinator is not aware of this member. Retriable: False.
    UnknownMemberId = 25,
    /// The session timeout is not within the range allowed by the broker (as
    /// configured by group.min.session.timeout.ms and group.max.session.
    /// timeout.ms). Retriable: False.
    InvalidSessionTimeout = 26,
    /// The group is rebalancing, so a rejoin is needed. Retriable: False.
    RebalanceInProgress = 27,
    /// The committing offset data size is not valid. Retriable: False.
    InvalidCommitOffsetSize = 28,
    /// Not authorized to access topics: [Topic authorization failed.]
    /// Retriable: False.
    TopicAuthorizationFailed = 29,
    /// Not authorized to access group: Group authorization failed. Retriable:
    /// False.
    GroupAuthorizationFailed = 30,
    /// Cluster authorization failed. Retriable: False.
    ClusterAuthorizationFailed = 31,
    /// The timestamp of the message is out of acceptable range. Retriable:
    /// False.
    InvalidTimestamp = 32,
    /// The broker does not support the requested SASL mechanism. Retriable:
    /// False.
    UnsupportedSaslMechanism = 33,
    /// Request is not valid given the current SASL state. Retriable: False.
    IllegalSaslState = 34,
    /// The version of API is not supported. Retriable: False.
    UnsupportedVersion = 35,
    /// Topic with this name already exists. Retriable: False.
    TopicAlreadyExists = 36,
    /// Number of partitions is below 1. Retriable: False.
    InvalidPartitions = 37,
    /// Replication factor is below 1 or larger than the number of available
    /// brokers. Retriable: False.
    InvalidReplicationFactor = 38,
    /// Replica assignment is invalid. Retriable: False.
    InvalidReplicaAssignment = 39,
    /// Configuration is invalid. Retriable: False.
    InvalidConfig = 40,
    /// This is not the correct controller for this cluster. Retriable: True.
    NotController = 41,
    /// This most likely occurs because of a request being malformed by the
    /// client library or the message was sent to an incompatible broker. See
    /// the broker logs for more details. Retriable: False.
    InvalidRequest = 42,
    /// The message format version on the broker does not support the request.
    /// Retriable: False.
    UnsupportedForMessageFormat = 43,
    /// Request parameters do not satisfy the configured policy. Retriable:
    /// False.
    PolicyViolation = 44,
    /// The broker received an out of order sequence number. Retriable: False.
    OutOfOrderSequenceNumber = 45,
    /// The broker received a duplicate sequence number. Retriable: False.
    DuplicateSequenceNumber = 46,
    /// Producer attempted an operation with an old epoch. Either there is a
    /// newer producer with the same transactionalId, or the producer's
    /// transaction has been expired by the broker. Retriable: False.
    InvalidProducerEpoch = 47,
    /// The producer attempted a transactional operation in an invalid state.
    /// Retriable: False.
    InvalidTxnState = 48,
    /// The producer attempted to use a producer id which is not currently
    /// assigned to its transactional id. Retriable: False.
    InvalidProducerIdMapping = 49,
    /// The transaction timeout is larger than the maximum value allowed by
    /// the broker (as configured by transaction.max.timeout.ms). Retriable:
    /// False.
    InvalidTransactionTimeout = 50,
    /// The producer attempted to update a transaction while another
    /// concurrent operation on the same transaction was ongoing. Retriable:
    /// False.
    ConcurrentTransactions = 51,
    /// Indicates that the transaction coordinator sending a WriteTxnMarker is
    /// no longer the current coordinator for a given producer. Retriable:
    /// False.
    TransactionCoordinatorFenced = 52,
    /// Transactional Id authorization failed. Retriable: False.
    TransactionalIdAuthorizationFailed = 53,
    /// Security features are disabled. Retriable: False.
    SecurityDisabled = 54,
    /// The broker did not attempt to execute this operation. This may happen
    /// for batched RPCs where some operations in the batch failed, causing
    /// the broker to respond without trying the rest. Retriable: False.
    OperationNotAttempted = 55,
    /// Disk error when trying to access log file on the disk. Retriable: True.
    KafkaStorageError = 56,
    /// The user-specified log directory is not found in the broker config.
    /// Retriable: False.
    LogDirNotFound = 57,
    /// SASL Authentication failed. Retriable: False.
    SaslAuthenticationFailed = 58,
    /// This exception is raised by the broker if it could not locate the
    /// producer metadata associated with the producerId in question. This
    /// could happen if, for instance, the producer's records were deleted
    /// because their retention time had elapsed. Once the last records of the
    /// producerId are removed, the producer's metadata is removed from the
    /// broker, and future appends by the producer will return this exception.
    /// Retriable: False.
    UnknownProducerId = 59,
    /// A partition reassignment is in progress. Retriable: False.
    ReassignmentInProgress = 60,
    /// Delegation Token feature is not enabled. Retriable: False.
    DelegationTokenAuthDisabled = 61,
    /// Delegation Token is not found on server. Retriable: False.
    DelegationTokenNotFound = 62,
    /// Specified Principal is not valid Owner/Renewer. Retriable: False.
    DelegationTokenOwnerMismatch = 63,
    /// Delegation Token requests are not allowed on PLAINTEXT/1-way SSL
    /// channels and on delegation token authenticated channels. Retriable:
    /// False.
    DelegationTokenRequestNotAllowed = 64,
    /// Delegation Token authorization failed. Retriable: False.
    DelegationTokenAuthorizationFailed = 65,
    /// Delegation Token is expired. Retriable: False.
    DelegationTokenExpired = 66,
    /// Supplied principalType is not supported. Retriable: False.
    InvalidPrincipalType = 67,
    /// The group is not empty. Retriable: False.
    NonEmptyGroup = 68,
    /// The group id does not exist. Retriable: False.
    GroupIdNotFound = 69,
    /// The fetch session ID was not found. Retriable: True.
    FetchSessionIdNotFound = 70,
    /// The fetch session epoch is invalid. Retriable: True.
    InvalidFetchSessionEpoch = 71,
    /// There is no listener on the leader broker that matches the listener on
    /// which metadata request was processed. Retriable: True.
    ListenerNotFound = 72,
    /// Topic deletion is disabled. Retriable: False.
    TopicDeletionDisabled = 73,
    /// The leader epoch in the request is older than the epoch on the broker
    /// Retriable: True.
    FencedLeaderEpoch = 74,
    /// The leader epoch in the request is newer than the epoch on the broker
    /// Retriable: True.
    UnknownLeaderEpoch = 75,
    /// The requesting client does not support the compression type of given
    /// partition. Retriable: False.
    UnsupportedCompressionType = 76,
    /// Broker epoch has changed Retriable: False.
    StaleBrokerEpoch = 77,
    /// The leader high watermark has not caught up from a recent leader
    /// election so the offsets cannot be guaranteed to be monotonically
    /// increasing Retriable: True.
    OffsetNotAvailable = 78,
    /// The group member needs to have a valid member id before actually
    /// entering a consumer group Retriable: False.
    MemberIdRequired = 79,
    /// The preferred leader was not available Retriable: True.
    PreferredLeaderNotAvailable = 80,
    /// Consumer group The consumer group has reached its max size. already
    /// has the configured maximum number of members. Retriable: False.
    GroupMaxSizeReached = 81,
}

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
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProduceRequest {
    V0 {
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v0::TopicData>,
    },
    V1 {
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v1::TopicData>,
    },
    V2 {
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v2::TopicData>,
    },
    V3 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v3::TopicData>,
    },
    V4 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v4::TopicData>,
    },
    V5 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v5::TopicData>,
    },
    V6 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v6::TopicData>,
    },
    V7 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The number of acknowledgments the producer requires the leader to have
        /// received before considering a request complete. Allowed values: 0 for
        /// no acknowledgments, 1 for only the leader and -1 for the full ISR.
        acks: i16,
        /// The time to await a response in ms.
        timeout: i32,
        /// null
        topic_data: Vec<produce_request::v7::TopicData>,
    },
}

pub mod produce_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicData {
            /// Name of topic
            pub topic: String,
            /// null
            pub data: Vec<Data>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            /// Topic partition id
            pub partition: i32,
            /// null
            pub record_set: crate::types::Records,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProduceResponse {
    V0 {
        /// null
        responses: Vec<produce_response::v0::Responses>,
    },
    V1 {
        /// null
        responses: Vec<produce_response::v1::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V2 {
        /// null
        responses: Vec<produce_response::v2::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V3 {
        /// null
        responses: Vec<produce_response::v3::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V4 {
        /// null
        responses: Vec<produce_response::v4::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V5 {
        /// null
        responses: Vec<produce_response::v5::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V6 {
        /// null
        responses: Vec<produce_response::v6::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V7 {
        /// null
        responses: Vec<produce_response::v7::Responses>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
}

pub mod produce_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
            /// The timestamp returned by broker after appending the messages. If
            /// CreateTime is used for the topic, the timestamp will be -1. If
            /// LogAppendTime is used for the topic, the timestamp will be the broker
            /// local time when the messages are appended.
            pub log_append_time: i64,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
            /// The timestamp returned by broker after appending the messages. If
            /// CreateTime is used for the topic, the timestamp will be -1. If
            /// LogAppendTime is used for the topic, the timestamp will be the broker
            /// local time when the messages are appended.
            pub log_append_time: i64,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
            /// The timestamp returned by broker after appending the messages. If
            /// CreateTime is used for the topic, the timestamp will be -1. If
            /// LogAppendTime is used for the topic, the timestamp will be the broker
            /// local time when the messages are appended.
            pub log_append_time: i64,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
            /// The timestamp returned by broker after appending the messages. If
            /// CreateTime is used for the topic, the timestamp will be -1. If
            /// LogAppendTime is used for the topic, the timestamp will be the broker
            /// local time when the messages are appended.
            pub log_append_time: i64,
            /// The start offset of the log at the time this produce response was
            /// created
            pub log_start_offset: i64,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
            /// The timestamp returned by broker after appending the messages. If
            /// CreateTime is used for the topic, the timestamp will be -1. If
            /// LogAppendTime is used for the topic, the timestamp will be the broker
            /// local time when the messages are appended.
            pub log_append_time: i64,
            /// The start offset of the log at the time this produce response was
            /// created
            pub log_start_offset: i64,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// null
            pub base_offset: i64,
            /// The timestamp returned by broker after appending the messages. If
            /// CreateTime is used for the topic, the timestamp will be -1. If
            /// LogAppendTime is used for the topic, the timestamp will be the broker
            /// local time when the messages are appended.
            pub log_append_time: i64,
            /// The start offset of the log at the time this produce response was
            /// created
            pub log_start_offset: i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FetchRequest {
    V0 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v0::Topics>,
    },
    V1 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v1::Topics>,
    },
    V2 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v2::Topics>,
    },
    V3 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v3::Topics>,
    },
    V4 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v4::Topics>,
    },
    V5 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v5::Topics>,
    },
    V6 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v6::Topics>,
    },
    V7 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// The fetch session ID
        session_id: i32,
        /// The fetch session epoch
        session_epoch: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v7::Topics>,
        /// Topics to remove from the fetch session.
        forgotten_topics_data: Vec<fetch_request::v7::ForgottenTopicsData>,
    },
    V8 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// The fetch session ID
        session_id: i32,
        /// The fetch session epoch
        session_epoch: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v8::Topics>,
        /// Topics to remove from the fetch session.
        forgotten_topics_data: Vec<fetch_request::v8::ForgottenTopicsData>,
    },
    V9 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// The fetch session ID
        session_id: i32,
        /// The fetch session epoch
        session_epoch: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v9::Topics>,
        /// Topics to remove from the fetch session.
        forgotten_topics_data: Vec<fetch_request::v9::ForgottenTopicsData>,
    },
    V10 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Maximum time in ms to wait for the response.
        max_wait_time: i32,
        /// Minimum bytes to accumulate in the response.
        min_bytes: i32,
        /// Maximum bytes to accumulate in the response. Note that this is not an
        /// absolute maximum, if the first message in the first non-empty
        /// partition of the fetch is larger than this value, the message will
        /// still be returned to ensure that progress can be made.
        max_bytes: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// The fetch session ID
        session_id: i32,
        /// The fetch session epoch
        session_epoch: i32,
        /// Topics to fetch in the order provided.
        topics: Vec<fetch_request::v10::Topics>,
        /// Topics to remove from the fetch session.
        forgotten_topics_data: Vec<fetch_request::v10::ForgottenTopicsData>,
    },
}

pub mod fetch_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Earliest available offset of the follower replica. The field is only
            /// used when request is sent by follower.
            pub log_start_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Earliest available offset of the follower replica. The field is only
            /// used when request is sent by follower.
            pub log_start_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ForgottenTopicsData {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Earliest available offset of the follower replica. The field is only
            /// used when request is sent by follower.
            pub log_start_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v8 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ForgottenTopicsData {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Earliest available offset of the follower replica. The field is only
            /// used when request is sent by follower.
            pub log_start_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v9 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ForgottenTopicsData {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The current leader epoch, if provided, is used to fence consumers/
            /// replicas with old metadata. If the epoch provided by the client is
            /// larger than the current epoch known to the broker, then the
            /// UNKNOWN_LEADER_EPOCH error code will be returned. If the provided
            /// epoch is smaller, then the FENCED_LEADER_EPOCH error code will be
            /// returned.
            pub current_leader_epoch: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Earliest available offset of the follower replica. The field is only
            /// used when request is sent by follower.
            pub log_start_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
    pub mod v10 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ForgottenTopicsData {
            /// Name of topic
            pub topic: String,
            /// Partitions to remove from the fetch session.
            pub partitions: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The current leader epoch, if provided, is used to fence consumers/
            /// replicas with old metadata. If the epoch provided by the client is
            /// larger than the current epoch known to the broker, then the
            /// UNKNOWN_LEADER_EPOCH error code will be returned. If the provided
            /// epoch is smaller, then the FENCED_LEADER_EPOCH error code will be
            /// returned.
            pub current_leader_epoch: i32,
            /// Message offset.
            pub fetch_offset: i64,
            /// Earliest available offset of the follower replica. The field is only
            /// used when request is sent by follower.
            pub log_start_offset: i64,
            /// Maximum bytes to fetch.
            pub partition_max_bytes: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FetchResponse {
    V0 {
        /// null
        responses: Vec<fetch_response::v0::Responses>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        responses: Vec<fetch_response::v1::Responses>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        responses: Vec<fetch_response::v2::Responses>,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        responses: Vec<fetch_response::v3::Responses>,
    },
    V4 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        responses: Vec<fetch_response::v4::Responses>,
    },
    V5 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        responses: Vec<fetch_response::v5::Responses>,
    },
    V6 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        responses: Vec<fetch_response::v6::Responses>,
    },
    V7 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The fetch session ID
        session_id: i32,
        /// null
        responses: Vec<fetch_response::v7::Responses>,
    },
    V8 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The fetch session ID
        session_id: i32,
        /// null
        responses: Vec<fetch_response::v8::Responses>,
    },
    V9 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The fetch session ID
        session_id: i32,
        /// null
        responses: Vec<fetch_response::v9::Responses>,
    },
    V10 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The fetch session ID
        session_id: i32,
        /// null
        responses: Vec<fetch_response::v10::Responses>,
    },
}

pub mod fetch_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// Earliest available offset.
            pub log_start_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// Earliest available offset.
            pub log_start_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// Earliest available offset.
            pub log_start_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
    pub mod v8 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// Earliest available offset.
            pub log_start_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
    pub mod v9 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// Earliest available offset.
            pub log_start_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
    pub mod v10 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// null
            pub partition_header: PartitionHeader,
            /// null
            pub record_set: crate::types::Records,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionHeader {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// Last committed offset.
            pub high_watermark: i64,
            /// The last stable offset (or LSO) of the partition. This is the last
            /// offset such that the state of all transactional records prior to this
            /// offset have been decided (ABORTED or COMMITTED)
            pub last_stable_offset: i64,
            /// Earliest available offset.
            pub log_start_offset: i64,
            /// null
            pub aborted_transactions: Vec<AbortedTransactions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct AbortedTransactions {
            /// The producer id associated with the aborted transactions
            pub producer_id: i64,
            /// The first offset in the aborted transaction
            pub first_offset: i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ListOffsetsRequest {
    V0 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Topics to list offsets.
        topics: Vec<list_offsets_request::v0::Topics>,
    },
    V1 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// Topics to list offsets.
        topics: Vec<list_offsets_request::v1::Topics>,
    },
    V2 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to list offsets.
        topics: Vec<list_offsets_request::v2::Topics>,
    },
    V3 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to list offsets.
        topics: Vec<list_offsets_request::v3::Topics>,
    },
    V4 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to list offsets.
        topics: Vec<list_offsets_request::v4::Topics>,
    },
    V5 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// This setting controls the visibility of transactional records. Using
        /// READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With
        /// READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED
        /// transactional records are visible. To be more concrete, READ_COMMITTED
        /// returns all data from offsets smaller than the current LSO (last
        /// stable offset), and enables the inclusion of the list of aborted
        /// transactions in the result, which allows consumers to discard ABORTED
        /// transactional records
        isolation_level: i8,
        /// Topics to list offsets.
        topics: Vec<list_offsets_request::v5::Topics>,
    },
}

pub mod list_offsets_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to list offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The target timestamp for the partition.
            pub timestamp: i64,
            /// Maximum offsets to return.
            pub max_num_offsets: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to list offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The target timestamp for the partition.
            pub timestamp: i64,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to list offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The target timestamp for the partition.
            pub timestamp: i64,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to list offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The target timestamp for the partition.
            pub timestamp: i64,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to list offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The current leader epoch, if provided, is used to fence consumers/
            /// replicas with old metadata. If the epoch provided by the client is
            /// larger than the current epoch known to the broker, then the
            /// UNKNOWN_LEADER_EPOCH error code will be returned. If the provided
            /// epoch is smaller, then the FENCED_LEADER_EPOCH error code will be
            /// returned.
            pub current_leader_epoch: i32,
            /// The target timestamp for the partition.
            pub timestamp: i64,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to list offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The current leader epoch, if provided, is used to fence consumers/
            /// replicas with old metadata. If the epoch provided by the client is
            /// larger than the current epoch known to the broker, then the
            /// UNKNOWN_LEADER_EPOCH error code will be returned. If the provided
            /// epoch is smaller, then the FENCED_LEADER_EPOCH error code will be
            /// returned.
            pub current_leader_epoch: i32,
            /// The target timestamp for the partition.
            pub timestamp: i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ListOffsetsResponse {
    V0 {
        /// The listed offsets by topic
        responses: Vec<list_offsets_response::v0::Responses>,
    },
    V1 {
        /// The listed offsets by topic
        responses: Vec<list_offsets_response::v1::Responses>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// The listed offsets by topic
        responses: Vec<list_offsets_response::v2::Responses>,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// The listed offsets by topic
        responses: Vec<list_offsets_response::v3::Responses>,
    },
    V4 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// The listed offsets by topic
        responses: Vec<list_offsets_response::v4::Responses>,
    },
    V5 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// The listed offsets by topic
        responses: Vec<list_offsets_response::v5::Responses>,
    },
}

pub mod list_offsets_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// The listed offsets by partition
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// A list of offsets.
            pub offsets: Vec<i64>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// The listed offsets by partition
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// The timestamp associated with the returned offset
            pub timestamp: i64,
            /// The offset found
            pub offset: i64,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// The listed offsets by partition
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// The timestamp associated with the returned offset
            pub timestamp: i64,
            /// The offset found
            pub offset: i64,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// The listed offsets by partition
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// The timestamp associated with the returned offset
            pub timestamp: i64,
            /// The offset found
            pub offset: i64,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// The listed offsets by partition
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// The timestamp associated with the returned offset
            pub timestamp: i64,
            /// The offset found
            pub offset: i64,
            /// The leader epoch
            pub leader_epoch: i32,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// The listed offsets by partition
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
            /// The timestamp associated with the returned offset
            pub timestamp: i64,
            /// The offset found
            pub offset: i64,
            /// The leader epoch
            pub leader_epoch: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MetadataRequest {
    V0 {
        /// An array of topics to fetch metadata for. If no topics are specified
        /// fetch metadata for all topics.
        topics: Vec<String>,
    },
    V1 {
        /// An array of topics to fetch metadata for. If the topics array is null
        /// fetch metadata for all topics.
        topics: Vec<String>,
    },
    V2 {
        /// An array of topics to fetch metadata for. If the topics array is null
        /// fetch metadata for all topics.
        topics: Vec<String>,
    },
    V3 {
        /// An array of topics to fetch metadata for. If the topics array is null
        /// fetch metadata for all topics.
        topics: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LeaderAndIsrRequest {
    V0 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<leader_and_isr_request::v0::PartitionStates>,
        /// Live leaders
        live_leaders: Vec<leader_and_isr_request::v0::LiveLeaders>,
    },
    V1 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<leader_and_isr_request::v1::PartitionStates>,
        /// Live leaders
        live_leaders: Vec<leader_and_isr_request::v1::LiveLeaders>,
    },
    V2 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// The broker epoch
        broker_epoch: i64,
        /// Topic states
        topic_states: Vec<leader_and_isr_request::v2::TopicStates>,
        /// Live leaders
        live_leaders: Vec<leader_and_isr_request::v2::LiveLeaders>,
    },
}

pub mod leader_and_isr_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveLeaders {
            /// The broker id
            pub id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
            /// Whether the replica should have existed on the broker or not
            pub is_new: bool,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveLeaders {
            /// The broker id
            pub id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicStates {
            /// Name of topic
            pub topic: String,
            /// Partition states
            pub partition_states: Vec<PartitionStates>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveLeaders {
            /// The broker id
            pub id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
            /// Whether the replica should have existed on the broker or not
            pub is_new: bool,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LeaderAndIsrResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// Response for the requests partitions
        partitions: Vec<leader_and_isr_response::v0::Partitions>,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// Response for the requests partitions
        partitions: Vec<leader_and_isr_response::v1::Partitions>,
    },
    V2 {
        /// Response error code
        error_code: i16,
        /// Response for the requests partitions
        partitions: Vec<leader_and_isr_response::v2::Partitions>,
    },
}

pub mod leader_and_isr_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StopReplicaRequest {
    V0 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Boolean which indicates if replica's partitions must be deleted.
        delete_partitions: bool,
        /// The partitions
        partitions: Vec<stop_replica_request::v0::Partitions>,
    },
    V1 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// The broker epoch
        broker_epoch: i64,
        /// Boolean which indicates if replica's partitions must be deleted.
        delete_partitions: bool,
        /// The partitions
        partitions: Vec<stop_replica_request::v1::Partitions>,
    },
}

pub mod stop_replica_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// The partition ids of a topic
            pub partition_ids: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StopReplicaResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// Response for the requests partitions
        partitions: Vec<stop_replica_response::v0::Partitions>,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// Response for the requests partitions
        partitions: Vec<stop_replica_response::v1::Partitions>,
    },
}

pub mod stop_replica_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UpdateMetadataRequest {
    V0 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<update_metadata_request::v0::PartitionStates>,
        /// Live broekrs
        live_brokers: Vec<update_metadata_request::v0::LiveBrokers>,
    },
    V1 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<update_metadata_request::v1::PartitionStates>,
        /// Live broekrs
        live_brokers: Vec<update_metadata_request::v1::LiveBrokers>,
    },
    V2 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<update_metadata_request::v2::PartitionStates>,
        /// Live broekrs
        live_brokers: Vec<update_metadata_request::v2::LiveBrokers>,
    },
    V3 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<update_metadata_request::v3::PartitionStates>,
        /// Live broekrs
        live_brokers: Vec<update_metadata_request::v3::LiveBrokers>,
    },
    V4 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// Partition states
        partition_states: Vec<update_metadata_request::v4::PartitionStates>,
        /// Live broekrs
        live_brokers: Vec<update_metadata_request::v4::LiveBrokers>,
    },
    V5 {
        /// The controller id
        controller_id: i32,
        /// The controller epoch
        controller_epoch: i32,
        /// The broker epoch
        broker_epoch: i64,
        /// Topic states
        topic_states: Vec<update_metadata_request::v5::TopicStates>,
        /// Live broekrs
        live_brokers: Vec<update_metadata_request::v5::LiveBrokers>,
    },
}

pub mod update_metadata_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveBrokers {
            /// The broker id
            pub id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveBrokers {
            /// The broker id
            pub id: i32,
            /// The endpoints
            pub end_points: Vec<EndPoints>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct EndPoints {
            /// The port on which the broker accepts requests.
            pub port: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The security protocol type.
            pub security_protocol_type: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveBrokers {
            /// The broker id
            pub id: i32,
            /// The endpoints
            pub end_points: Vec<EndPoints>,
            /// The rack
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct EndPoints {
            /// The port on which the broker accepts requests.
            pub port: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The security protocol type.
            pub security_protocol_type: i16,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveBrokers {
            /// The broker id
            pub id: i32,
            /// The endpoints
            pub end_points: Vec<EndPoints>,
            /// The rack
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct EndPoints {
            /// The port on which the broker accepts requests.
            pub port: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The listener name.
            pub listener_name: String,
            /// The security protocol type.
            pub security_protocol_type: i16,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
            /// The offline replica ids
            pub offline_replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveBrokers {
            /// The broker id
            pub id: i32,
            /// The endpoints
            pub end_points: Vec<EndPoints>,
            /// The rack
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct EndPoints {
            /// The port on which the broker accepts requests.
            pub port: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The listener name.
            pub listener_name: String,
            /// The security protocol type.
            pub security_protocol_type: i16,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicStates {
            /// Name of topic
            pub topic: String,
            /// Partition states
            pub partition_states: Vec<PartitionStates>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LiveBrokers {
            /// The broker id
            pub id: i32,
            /// The endpoints
            pub end_points: Vec<EndPoints>,
            /// The rack
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionStates {
            /// Topic partition id
            pub partition: i32,
            /// The controller epoch
            pub controller_epoch: i32,
            /// The broker id for the leader.
            pub leader: i32,
            /// The leader epoch.
            pub leader_epoch: i32,
            /// The in sync replica ids.
            pub isr: Vec<i32>,
            /// The ZK version.
            pub zk_version: i32,
            /// The replica ids.
            pub replicas: Vec<i32>,
            /// The offline replica ids
            pub offline_replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct EndPoints {
            /// The port on which the broker accepts requests.
            pub port: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The listener name.
            pub listener_name: String,
            /// The security protocol type.
            pub security_protocol_type: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UpdateMetadataResponse {
    V0 {
        /// Response error code
        error_code: i16,
    },
    V1 {
        /// Response error code
        error_code: i16,
    },
    V2 {
        /// Response error code
        error_code: i16,
    },
    V3 {
        /// Response error code
        error_code: i16,
    },
    V4 {
        /// Response error code
        error_code: i16,
    },
    V5 {
        /// Response error code
        error_code: i16,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ControlledShutdownRequest {
    V0 {
        /// The id of the broker for which controlled shutdown has been requested.
        broker_id: i32,
    },
    V1 {
        /// The id of the broker for which controlled shutdown has been requested.
        broker_id: i32,
    },
    V2 {
        /// The id of the broker for which controlled shutdown has been requested.
        broker_id: i32,
        /// The broker epoch
        broker_epoch: i64,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ControlledShutdownResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// The partitions that the broker still leads.
        partitions_remaining: Vec<controlled_shutdown_response::v0::PartitionsRemaining>,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// The partitions that the broker still leads.
        partitions_remaining: Vec<controlled_shutdown_response::v1::PartitionsRemaining>,
    },
    V2 {
        /// Response error code
        error_code: i16,
        /// The partitions that the broker still leads.
        partitions_remaining: Vec<controlled_shutdown_response::v2::PartitionsRemaining>,
    },
}

pub mod controlled_shutdown_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionsRemaining {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionsRemaining {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionsRemaining {
            /// Name of topic
            pub topic: String,
            /// Topic partition id
            pub partition: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetCommitRequest {
    V0 {
        /// The unique group identifier
        group_id: String,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v0::Topics>,
    },
    V1 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v1::Topics>,
    },
    V2 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Time period in ms to retain the offset.
        retention_time: i64,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v2::Topics>,
    },
    V3 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Time period in ms to retain the offset.
        retention_time: i64,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v3::Topics>,
    },
    V4 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Time period in ms to retain the offset.
        retention_time: i64,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v4::Topics>,
    },
    V5 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v5::Topics>,
    },
    V6 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Topics to commit offsets
        topics: Vec<offset_commit_request::v6::Topics>,
    },
}

pub mod offset_commit_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Timestamp of the commit
            pub timestamp: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// The leader epoch, if provided is derived from the last consumed record.
            /// This is used by the consumer to check for log truncation and to ensure
            /// partition metadata is up to date following a group rebalance.
            pub leader_epoch: i32,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetCommitResponse {
    V0 {
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v0::Responses>,
    },
    V1 {
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v1::Responses>,
    },
    V2 {
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v2::Responses>,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v3::Responses>,
    },
    V4 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v4::Responses>,
    },
    V5 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v5::Responses>,
    },
    V6 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed partitions
        responses: Vec<offset_commit_response::v6::Responses>,
    },
}

pub mod offset_commit_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses for committed partitions
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetFetchRequest {
    V0 {
        /// The unique group identifier
        group_id: String,
        /// Topics to fetch offsets.
        topics: Vec<offset_fetch_request::v0::Topics>,
    },
    V1 {
        /// The unique group identifier
        group_id: String,
        /// Topics to fetch offsets.
        topics: Vec<offset_fetch_request::v1::Topics>,
    },
    V2 {
        /// The unique group identifier
        group_id: String,
        /// Topics to fetch offsets. If the topic array is null fetch offsets for
        /// all topics.
        topics: Vec<offset_fetch_request::v2::Topics>,
    },
    V3 {
        /// The unique group identifier
        group_id: String,
        /// Topics to fetch offsets. If the topic array is null fetch offsets for
        /// all topics.
        topics: Vec<offset_fetch_request::v3::Topics>,
    },
    V4 {
        /// The unique group identifier
        group_id: String,
        /// Topics to fetch offsets. If the topic array is null fetch offsets for
        /// all topics.
        topics: Vec<offset_fetch_request::v4::Topics>,
    },
    V5 {
        /// The unique group identifier
        group_id: String,
        /// Topics to fetch offsets. If the topic array is null fetch offsets for
        /// all topics.
        topics: Vec<offset_fetch_request::v5::Topics>,
    },
}

pub mod offset_fetch_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to fetch offsets.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetFetchResponse {
    V0 {
        /// Responses by topic for fetched offsets
        responses: Vec<offset_fetch_response::v0::Responses>,
    },
    V1 {
        /// Responses by topic for fetched offsets
        responses: Vec<offset_fetch_response::v1::Responses>,
    },
    V2 {
        /// Responses by topic for fetched offsets
        responses: Vec<offset_fetch_response::v2::Responses>,
        /// Response error code
        error_code: i16,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for fetched offsets
        responses: Vec<offset_fetch_response::v3::Responses>,
        /// Response error code
        error_code: i16,
    },
    V4 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for fetched offsets
        responses: Vec<offset_fetch_response::v4::Responses>,
        /// Response error code
        error_code: i16,
    },
    V5 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for fetched offsets
        responses: Vec<offset_fetch_response::v5::Responses>,
        /// Response error code
        error_code: i16,
    },
}

pub mod offset_fetch_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for fetched offsets
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for fetched offsets
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for fetched offsets
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for fetched offsets
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for fetched offsets
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for fetched offsets
            pub partition_responses: Vec<PartitionResponses>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResponses {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// The leader epoch, if provided is derived from the last consumed record.
            /// This is used by the consumer to check for log truncation and to ensure
            /// partition metadata is up to date following a group rebalance.
            pub leader_epoch: i32,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FindCoordinatorRequest {
    V0 {
        /// The unique group identifier
        group_id: String,
    },
    V1 {
        /// Id to use for finding the coordinator (for groups, this is the groupId,
        /// for transactional producers, this is the transactional id)
        coordinator_key: String,
        /// The type of coordinator to find (0 = group, 1 = transaction)
        coordinator_type: i8,
    },
    V2 {
        /// Id to use for finding the coordinator (for groups, this is the groupId,
        /// for transactional producers, this is the transactional id)
        coordinator_key: String,
        /// The type of coordinator to find (0 = group, 1 = transaction)
        coordinator_type: i8,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FindCoordinatorResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// Host and port information for the coordinator for a consumer group.
        coordinator: find_coordinator_response::v0::Coordinator,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// Response error message
        error_message: crate::types::NullableString,
        /// Host and port information for the coordinator
        coordinator: find_coordinator_response::v1::Coordinator,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// Response error message
        error_message: crate::types::NullableString,
        /// Host and port information for the coordinator
        coordinator: find_coordinator_response::v2::Coordinator,
    },
}

pub mod find_coordinator_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Coordinator {
            /// The broker id.
            pub node_id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Coordinator {
            /// The broker id.
            pub node_id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Coordinator {
            /// The broker id.
            pub node_id: i32,
            /// The hostname of the broker.
            pub host: String,
            /// The port on which the broker accepts requests.
            pub port: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum JoinGroupRequest {
    V0 {
        /// The unique group identifier
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in ms.
        session_timeout: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Unique name for class of protocols implemented by group
        protocol_type: String,
        /// List of protocols that the member supports
        group_protocols: Vec<join_group_request::v0::GroupProtocols>,
    },
    V1 {
        /// The unique group identifier
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in ms.
        session_timeout: i32,
        /// The maximum time that the coordinator will wait for each member to
        /// rejoin when rebalancing the group
        rebalance_timeout: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Unique name for class of protocols implemented by group
        protocol_type: String,
        /// List of protocols that the member supports
        group_protocols: Vec<join_group_request::v1::GroupProtocols>,
    },
    V2 {
        /// The unique group identifier
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in ms.
        session_timeout: i32,
        /// The maximum time that the coordinator will wait for each member to
        /// rejoin when rebalancing the group
        rebalance_timeout: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Unique name for class of protocols implemented by group
        protocol_type: String,
        /// List of protocols that the member supports
        group_protocols: Vec<join_group_request::v2::GroupProtocols>,
    },
    V3 {
        /// The unique group identifier
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in ms.
        session_timeout: i32,
        /// The maximum time that the coordinator will wait for each member to
        /// rejoin when rebalancing the group
        rebalance_timeout: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Unique name for class of protocols implemented by group
        protocol_type: String,
        /// List of protocols that the member supports
        group_protocols: Vec<join_group_request::v3::GroupProtocols>,
    },
    V4 {
        /// The unique group identifier
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in ms.
        session_timeout: i32,
        /// The maximum time that the coordinator will wait for each member to
        /// rejoin when rebalancing the group
        rebalance_timeout: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// Unique name for class of protocols implemented by group
        protocol_type: String,
        /// List of protocols that the member supports
        group_protocols: Vec<join_group_request::v4::GroupProtocols>,
    },
}

pub mod join_group_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupProtocols {
            /// null
            pub protocol_name: String,
            /// null
            pub protocol_metadata: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupProtocols {
            /// null
            pub protocol_name: String,
            /// null
            pub protocol_metadata: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupProtocols {
            /// null
            pub protocol_name: String,
            /// null
            pub protocol_metadata: crate::types::Bytes,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupProtocols {
            /// null
            pub protocol_name: String,
            /// null
            pub protocol_metadata: crate::types::Bytes,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupProtocols {
            /// null
            pub protocol_name: String,
            /// null
            pub protocol_metadata: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum JoinGroupResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// The generation of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator
        group_protocol: String,
        /// The leader of the group
        leader_id: String,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        members: Vec<join_group_response::v0::Members>,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// The generation of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator
        group_protocol: String,
        /// The leader of the group
        leader_id: String,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        members: Vec<join_group_response::v1::Members>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The generation of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator
        group_protocol: String,
        /// The leader of the group
        leader_id: String,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        members: Vec<join_group_response::v2::Members>,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The generation of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator
        group_protocol: String,
        /// The leader of the group
        leader_id: String,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        members: Vec<join_group_response::v3::Members>,
    },
    V4 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The generation of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator
        group_protocol: String,
        /// The leader of the group
        leader_id: String,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        members: Vec<join_group_response::v4::Members>,
    },
}

pub mod join_group_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_metadata: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_metadata: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_metadata: crate::types::Bytes,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_metadata: crate::types::Bytes,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_metadata: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HeartbeatRequest {
    V0 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
    },
    V1 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
    },
    V2 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HeartbeatResponse {
    V0 {
        /// Response error code
        error_code: i16,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LeaveGroupRequest {
    V0 {
        /// The ID of the group to leave.
        group_id: String,
        /// The member ID to remove from the group.
        member_id: String,
    },
    V1 {
        /// The ID of the group to leave.
        group_id: String,
        /// The member ID to remove from the group.
        member_id: String,
    },
    V2 {
        /// The ID of the group to leave.
        group_id: String,
        /// The member ID to remove from the group.
        member_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LeaveGroupResponse {
    V0 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
    },
    V1 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SyncGroupRequest {
    V0 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        group_assignment: Vec<sync_group_request::v0::GroupAssignment>,
    },
    V1 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        group_assignment: Vec<sync_group_request::v1::GroupAssignment>,
    },
    V2 {
        /// The unique group identifier
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member id assigned by the group coordinator or null if joining for
        /// the first time.
        member_id: String,
        /// null
        group_assignment: Vec<sync_group_request::v2::GroupAssignment>,
    },
}

pub mod sync_group_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupAssignment {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupAssignment {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupAssignment {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// null
            pub member_assignment: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SyncGroupResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// null
        member_assignment: crate::types::Bytes,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// null
        member_assignment: crate::types::Bytes,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// null
        member_assignment: crate::types::Bytes,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeGroupsRequest {
    V0 {
        /// List of groupIds to request metadata for (an empty groupId array will
        /// return empty group metadata)
        group_ids: Vec<String>,
    },
    V1 {
        /// List of groupIds to request metadata for (an empty groupId array will
        /// return empty group metadata)
        group_ids: Vec<String>,
    },
    V2 {
        /// List of groupIds to request metadata for (an empty groupId array will
        /// return empty group metadata)
        group_ids: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeGroupsResponse {
    V0 {
        /// null
        groups: Vec<describe_groups_response::v0::Groups>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        groups: Vec<describe_groups_response::v1::Groups>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        groups: Vec<describe_groups_response::v2::Groups>,
    },
}

pub mod describe_groups_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// Response error code
            pub error_code: i16,
            /// The unique group identifier
            pub group_id: String,
            /// The current state of the group (one of: Dead, Stable,
            /// CompletingRebalance, PreparingRebalance, or empty if there is no
            /// active group)
            pub state: String,
            /// The current group protocol type (will be empty if there is no active
            /// group)
            pub protocol_type: String,
            /// The current group protocol (only provided if the group is Stable)
            pub protocol: String,
            /// Current group members (only provided if the group is not Dead)
            pub members: Vec<Members>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// The client id used in the member's latest join group request
            pub client_id: String,
            /// The client host used in the request session corresponding to the
            /// member's join group.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use (will
            /// only be present if the group is stable)
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader (will only be
            /// present if the group is stable)
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// Response error code
            pub error_code: i16,
            /// The unique group identifier
            pub group_id: String,
            /// The current state of the group (one of: Dead, Stable,
            /// CompletingRebalance, PreparingRebalance, or empty if there is no
            /// active group)
            pub state: String,
            /// The current group protocol type (will be empty if there is no active
            /// group)
            pub protocol_type: String,
            /// The current group protocol (only provided if the group is Stable)
            pub protocol: String,
            /// Current group members (only provided if the group is not Dead)
            pub members: Vec<Members>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// The client id used in the member's latest join group request
            pub client_id: String,
            /// The client host used in the request session corresponding to the
            /// member's join group.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use (will
            /// only be present if the group is stable)
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader (will only be
            /// present if the group is stable)
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// Response error code
            pub error_code: i16,
            /// The unique group identifier
            pub group_id: String,
            /// The current state of the group (one of: Dead, Stable,
            /// CompletingRebalance, PreparingRebalance, or empty if there is no
            /// active group)
            pub state: String,
            /// The current group protocol type (will be empty if there is no active
            /// group)
            pub protocol_type: String,
            /// The current group protocol (only provided if the group is Stable)
            pub protocol: String,
            /// Current group members (only provided if the group is not Dead)
            pub members: Vec<Members>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member id assigned by the group coordinator or null if joining for
            /// the first time.
            pub member_id: String,
            /// The client id used in the member's latest join group request
            pub client_id: String,
            /// The client host used in the request session corresponding to the
            /// member's join group.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use (will
            /// only be present if the group is stable)
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader (will only be
            /// present if the group is stable)
            pub member_assignment: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ListGroupsRequest {
    V0 {},
    V1 {},
    V2 {},
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ListGroupsResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// null
        groups: Vec<list_groups_response::v0::Groups>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// null
        groups: Vec<list_groups_response::v1::Groups>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// null
        groups: Vec<list_groups_response::v2::Groups>,
    },
}

pub mod list_groups_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The unique group identifier
            pub group_id: String,
            /// null
            pub protocol_type: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The unique group identifier
            pub group_id: String,
            /// null
            pub protocol_type: String,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The unique group identifier
            pub group_id: String,
            /// null
            pub protocol_type: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SaslHandshakeRequest {
    V0 {
        /// SASL Mechanism chosen by the client.
        mechanism: String,
    },
    V1 {
        /// SASL Mechanism chosen by the client.
        mechanism: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SaslHandshakeResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// Array of mechanisms enabled in the server.
        enabled_mechanisms: Vec<String>,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// Array of mechanisms enabled in the server.
        enabled_mechanisms: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ApiVersionsRequest {
    V0 {},
    V1 {},
    V2 {},
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ApiVersionsResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// API versions supported by the broker.
        api_versions: Vec<api_versions_response::v0::ApiVersions>,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// API versions supported by the broker.
        api_versions: Vec<api_versions_response::v1::ApiVersions>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V2 {
        /// Response error code
        error_code: i16,
        /// API versions supported by the broker.
        api_versions: Vec<api_versions_response::v2::ApiVersions>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
}

pub mod api_versions_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ApiVersions {
            /// API key.
            pub api_key: i16,
            /// Minimum supported version.
            pub min_version: i16,
            /// Maximum supported version.
            pub max_version: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ApiVersions {
            /// API key.
            pub api_key: i16,
            /// Minimum supported version.
            pub min_version: i16,
            /// Maximum supported version.
            pub max_version: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ApiVersions {
            /// API key.
            pub api_key: i16,
            /// Minimum supported version.
            pub min_version: i16,
            /// Maximum supported version.
            pub max_version: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsRequest {
    V0 {
        /// An array of single topic creation requests. Can not have multiple
        /// entries for the same topic.
        create_topic_requests: Vec<create_topics_request::v0::CreateTopicRequests>,
        /// The time in ms to wait for a topic to be completely created on the
        /// controller node. Values <= 0 will trigger topic creation and return
        /// immediately
        timeout: i32,
    },
    V1 {
        /// An array of single topic creation requests. Can not have multiple
        /// entries for the same topic.
        create_topic_requests: Vec<create_topics_request::v1::CreateTopicRequests>,
        /// The time in ms to wait for a topic to be completely created on the
        /// controller node. Values <= 0 will trigger topic creation and return
        /// immediately
        timeout: i32,
        /// If this is true, the request will be validated, but the topic won't be
        /// created.
        validate_only: bool,
    },
    V2 {
        /// An array of single topic creation requests. Can not have multiple
        /// entries for the same topic.
        create_topic_requests: Vec<create_topics_request::v2::CreateTopicRequests>,
        /// The time in ms to wait for a topic to be completely created on the
        /// controller node. Values <= 0 will trigger topic creation and return
        /// immediately
        timeout: i32,
        /// If this is true, the request will be validated, but the topic won't be
        /// created.
        validate_only: bool,
    },
    V3 {
        /// An array of single topic creation requests. Can not have multiple
        /// entries for the same topic.
        create_topic_requests: Vec<create_topics_request::v3::CreateTopicRequests>,
        /// The time in ms to wait for a topic to be completely created on the
        /// controller node. Values <= 0 will trigger topic creation and return
        /// immediately
        timeout: i32,
        /// If this is true, the request will be validated, but the topic won't be
        /// created.
        validate_only: bool,
    },
}

pub mod create_topics_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreateTopicRequests {
            /// Name of topic
            pub topic: String,
            /// Number of partitions to be created. -1 indicates unset.
            pub num_partitions: i32,
            /// Replication factor for the topic. -1 indicates unset.
            pub replication_factor: i16,
            /// Replica assignment among kafka brokers for this topic partitions. If
            /// this is set num_partitions and replication_factor must be unset.
            pub replica_assignment: Vec<ReplicaAssignment>,
            /// Topic level configuration for topic to be set.
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaAssignment {
            /// Topic partition id
            pub partition: i32,
            /// The set of all nodes that should host this partition. The first
            /// replica in the list is the preferred leader.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// Configuration name
            pub config_name: String,
            /// Configuration value
            pub config_value: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreateTopicRequests {
            /// Name of topic
            pub topic: String,
            /// Number of partitions to be created. -1 indicates unset.
            pub num_partitions: i32,
            /// Replication factor for the topic. -1 indicates unset.
            pub replication_factor: i16,
            /// Replica assignment among kafka brokers for this topic partitions. If
            /// this is set num_partitions and replication_factor must be unset.
            pub replica_assignment: Vec<ReplicaAssignment>,
            /// Topic level configuration for topic to be set.
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaAssignment {
            /// Topic partition id
            pub partition: i32,
            /// The set of all nodes that should host this partition. The first
            /// replica in the list is the preferred leader.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// Configuration name
            pub config_name: String,
            /// Configuration value
            pub config_value: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreateTopicRequests {
            /// Name of topic
            pub topic: String,
            /// Number of partitions to be created. -1 indicates unset.
            pub num_partitions: i32,
            /// Replication factor for the topic. -1 indicates unset.
            pub replication_factor: i16,
            /// Replica assignment among kafka brokers for this topic partitions. If
            /// this is set num_partitions and replication_factor must be unset.
            pub replica_assignment: Vec<ReplicaAssignment>,
            /// Topic level configuration for topic to be set.
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaAssignment {
            /// Topic partition id
            pub partition: i32,
            /// The set of all nodes that should host this partition. The first
            /// replica in the list is the preferred leader.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// Configuration name
            pub config_name: String,
            /// Configuration value
            pub config_value: crate::types::NullableString,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreateTopicRequests {
            /// Name of topic
            pub topic: String,
            /// Number of partitions to be created. -1 indicates unset.
            pub num_partitions: i32,
            /// Replication factor for the topic. -1 indicates unset.
            pub replication_factor: i16,
            /// Replica assignment among kafka brokers for this topic partitions. If
            /// this is set num_partitions and replication_factor must be unset.
            pub replica_assignment: Vec<ReplicaAssignment>,
            /// Topic level configuration for topic to be set.
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaAssignment {
            /// Topic partition id
            pub partition: i32,
            /// The set of all nodes that should host this partition. The first
            /// replica in the list is the preferred leader.
            pub replicas: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// Configuration name
            pub config_name: String,
            /// Configuration value
            pub config_value: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsResponse {
    V0 {
        /// An array of per topic error codes.
        topic_errors: Vec<create_topics_response::v0::TopicErrors>,
    },
    V1 {
        /// An array of per topic errors.
        topic_errors: Vec<create_topics_response::v1::TopicErrors>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per topic errors.
        topic_errors: Vec<create_topics_response::v2::TopicErrors>,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per topic errors.
        topic_errors: Vec<create_topics_response::v3::TopicErrors>,
    },
}

pub mod create_topics_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrors {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrors {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrors {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrors {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteTopicsRequest {
    V0 {
        /// An array of topics to be deleted.
        topics: Vec<String>,
        /// The time in ms to wait for a topic to be completely deleted on the
        /// controller node. Values <= 0 will trigger topic deletion and return
        /// immediately
        timeout: i32,
    },
    V1 {
        /// An array of topics to be deleted.
        topics: Vec<String>,
        /// The time in ms to wait for a topic to be completely deleted on the
        /// controller node. Values <= 0 will trigger topic deletion and return
        /// immediately
        timeout: i32,
    },
    V2 {
        /// An array of topics to be deleted.
        topics: Vec<String>,
        /// The time in ms to wait for a topic to be completely deleted on the
        /// controller node. Values <= 0 will trigger topic deletion and return
        /// immediately
        timeout: i32,
    },
    V3 {
        /// An array of topics to be deleted.
        topics: Vec<String>,
        /// The time in ms to wait for a topic to be completely deleted on the
        /// controller node. Values <= 0 will trigger topic deletion and return
        /// immediately
        timeout: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteTopicsResponse {
    V0 {
        /// An array of per topic error codes.
        topic_error_codes: Vec<delete_topics_response::v0::TopicErrorCodes>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per topic error codes.
        topic_error_codes: Vec<delete_topics_response::v1::TopicErrorCodes>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per topic error codes.
        topic_error_codes: Vec<delete_topics_response::v2::TopicErrorCodes>,
    },
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per topic error codes.
        topic_error_codes: Vec<delete_topics_response::v3::TopicErrorCodes>,
    },
}

pub mod delete_topics_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrorCodes {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrorCodes {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrorCodes {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrorCodes {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteRecordsRequest {
    V0 {
        /// null
        topics: Vec<delete_records_request::v0::Topics>,
        /// The maximum time to await a response in ms.
        timeout: i32,
    },
    V1 {
        /// null
        topics: Vec<delete_records_request::v1::Topics>,
        /// The maximum time to await a response in ms.
        timeout: i32,
    },
}

pub mod delete_records_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The offset before which the messages will be deleted. -1 means high-
            /// watermark for the partition.
            pub offset: i64,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The offset before which the messages will be deleted. -1 means high-
            /// watermark for the partition.
            pub offset: i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteRecordsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        topics: Vec<delete_records_response::v0::Topics>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        topics: Vec<delete_records_response::v1::Topics>,
    },
}

pub mod delete_records_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Smallest available offset of all live replicas
            pub low_watermark: i64,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Smallest available offset of all live replicas
            pub low_watermark: i64,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InitProducerIdRequest {
    V0 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The time in ms to wait for before aborting idle transactions sent by
        /// this producer.
        transaction_timeout_ms: i32,
    },
    V1 {
        /// The transactional id or null if the producer is not transactional
        transactional_id: crate::types::NullableString,
        /// The time in ms to wait for before aborting idle transactions sent by
        /// this producer.
        transaction_timeout_ms: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InitProducerIdResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetForLeaderEpochRequest {
    V0 {
        /// An array of topics to get epochs for
        topics: Vec<offset_for_leader_epoch_request::v0::Topics>,
    },
    V1 {
        /// An array of topics to get epochs for
        topics: Vec<offset_for_leader_epoch_request::v1::Topics>,
    },
    V2 {
        /// An array of topics to get epochs for
        topics: Vec<offset_for_leader_epoch_request::v2::Topics>,
    },
}

pub mod offset_for_leader_epoch_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// An array of partitions to get epochs for
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The epoch to lookup an offset for.
            pub leader_epoch: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// An array of partitions to get epochs for
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The epoch to lookup an offset for.
            pub leader_epoch: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// An array of partitions to get epochs for
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The current leader epoch, if provided, is used to fence consumers/
            /// replicas with old metadata. If the epoch provided by the client is
            /// larger than the current epoch known to the broker, then the
            /// UNKNOWN_LEADER_EPOCH error code will be returned. If the provided
            /// epoch is smaller, then the FENCED_LEADER_EPOCH error code will be
            /// returned.
            pub current_leader_epoch: i32,
            /// The epoch to lookup an offset for.
            pub leader_epoch: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetForLeaderEpochResponse {
    V0 {
        /// An array of topics for which we have leader offsets for some requested
        /// partition leader epoch
        topics: Vec<offset_for_leader_epoch_response::v0::Topics>,
    },
    V1 {
        /// An array of topics for which we have leader offsets for some requested
        /// partition leader epoch
        topics: Vec<offset_for_leader_epoch_response::v1::Topics>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of topics for which we have leader offsets for some requested
        /// partition leader epoch
        topics: Vec<offset_for_leader_epoch_response::v2::Topics>,
    },
}

pub mod offset_for_leader_epoch_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// An array of offsets by partition
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Response error code
            pub error_code: i16,
            /// Topic partition id
            pub partition: i32,
            /// The end offset
            pub end_offset: i64,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// An array of offsets by partition
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Response error code
            pub error_code: i16,
            /// Topic partition id
            pub partition: i32,
            /// The leader epoch
            pub leader_epoch: i32,
            /// The end offset
            pub end_offset: i64,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// An array of offsets by partition
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Response error code
            pub error_code: i16,
            /// Topic partition id
            pub partition: i32,
            /// The leader epoch
            pub leader_epoch: i32,
            /// The end offset
            pub end_offset: i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddPartitionsToTxnRequest {
    V0 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// The partitions to add to the transaction.
        topics: Vec<add_partitions_to_txn_request::v0::Topics>,
    },
    V1 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// The partitions to add to the transaction.
        topics: Vec<add_partitions_to_txn_request::v1::Topics>,
    },
}

pub mod add_partitions_to_txn_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<i32>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddPartitionsToTxnResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        errors: Vec<add_partitions_to_txn_response::v0::Errors>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        errors: Vec<add_partitions_to_txn_response::v1::Errors>,
    },
}

pub mod add_partitions_to_txn_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Errors {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_errors: Vec<PartitionErrors>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionErrors {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Errors {
            /// Name of topic
            pub topic: String,
            /// null
            pub partition_errors: Vec<PartitionErrors>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionErrors {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddOffsetsToTxnRequest {
    V0 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// The unique group identifier
        group_id: String,
    },
    V1 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// The unique group identifier
        group_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddOffsetsToTxnResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EndTxnRequest {
    V0 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// The result of the transaction (0 = ABORT, 1 = COMMIT)
        transaction_result: bool,
    },
    V1 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// The result of the transaction (0 = ABORT, 1 = COMMIT)
        transaction_result: bool,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EndTxnResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WriteTxnMarkersRequest {
    V0 {
        /// The transaction markers to be written.
        transaction_markers: Vec<write_txn_markers_request::v0::TransactionMarkers>,
    },
}

pub mod write_txn_markers_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TransactionMarkers {
            /// Current producer id in use by the transactional id.
            pub producer_id: i64,
            /// Current epoch associated with the producer id.
            pub producer_epoch: i16,
            /// The result of the transaction to write to the partitions (false =
            /// ABORT, true = COMMIT)
            pub transaction_result: bool,
            /// The partitions to write markers for.
            pub topics: Vec<Topics>,
            /// Epoch associated with the transaction state partition hosted by this
            /// transaction coordinator
            pub coordinator_epoch: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WriteTxnMarkersResponse {
    V0 {
        /// Errors per partition from writing markers.
        transaction_markers: Vec<write_txn_markers_response::v0::TransactionMarkers>,
    },
}

pub mod write_txn_markers_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TransactionMarkers {
            /// Current producer id in use by the transactional id.
            pub producer_id: i64,
            /// Errors per partition from writing markers.
            pub topics: Vec<Topics>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TxnOffsetCommitRequest {
    V0 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// The unique group identifier
        group_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// Topics to commit offsets
        topics: Vec<txn_offset_commit_request::v0::Topics>,
    },
    V1 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// The unique group identifier
        group_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// Topics to commit offsets
        topics: Vec<txn_offset_commit_request::v1::Topics>,
    },
    V2 {
        /// The transactional id corresponding to the transaction.
        transactional_id: String,
        /// The unique group identifier
        group_id: String,
        /// Current producer id in use by the transactional id.
        producer_id: i64,
        /// Current epoch associated with the producer id.
        producer_epoch: i16,
        /// Topics to commit offsets
        topics: Vec<txn_offset_commit_request::v2::Topics>,
    },
}

pub mod txn_offset_commit_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Partitions to commit offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Message offset to be committed
            pub offset: i64,
            /// The leader epoch, if provided is derived from the last consumed record.
            /// This is used by the consumer to check for log truncation and to ensure
            /// partition metadata is up to date following a group rebalance.
            pub leader_epoch: i32,
            /// Any associated metadata the client wants to keep.
            pub metadata: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TxnOffsetCommitResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed offsets
        topics: Vec<txn_offset_commit_response::v0::Topics>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed offsets
        topics: Vec<txn_offset_commit_response::v1::Topics>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Responses by topic for committed offsets
        topics: Vec<txn_offset_commit_response::v2::Topics>,
    },
}

pub mod txn_offset_commit_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for committed offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for committed offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// Responses by partition for committed offsets
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeAclsRequest {
    V0 {
        /// The resource type
        resource_type: i8,
        /// The resource name filter
        resource_name: crate::types::NullableString,
        /// The ACL principal filter
        principal: crate::types::NullableString,
        /// The ACL host filter
        host: crate::types::NullableString,
        /// The ACL operation
        operation: i8,
        /// The ACL permission type
        permission_type: i8,
    },
    V1 {
        /// The resource type
        resource_type: i8,
        /// The resource name filter
        resource_name: crate::types::NullableString,
        /// The resource pattern type filter
        resource_pattern_type_filter: i8,
        /// The ACL principal filter
        principal: crate::types::NullableString,
        /// The ACL host filter
        host: crate::types::NullableString,
        /// The ACL operation
        operation: i8,
        /// The ACL permission type
        permission_type: i8,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeAclsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// Response error message
        error_message: crate::types::NullableString,
        /// The resources and their associated ACLs.
        resources: Vec<describe_acls_response::v0::Resources>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// Response error message
        error_message: crate::types::NullableString,
        /// The resources and their associated ACLs.
        resources: Vec<describe_acls_response::v1::Resources>,
    },
}

pub mod describe_acls_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// The resource type
            pub resource_type: i8,
            /// The resource name
            pub resource_name: String,
            /// null
            pub acls: Vec<Acls>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Acls {
            /// The ACL principal
            pub principal: String,
            /// The ACL host
            pub host: String,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// The resource type
            pub resource_type: i8,
            /// The resource name
            pub resource_name: String,
            /// The resource pattern type
            pub resource_pattten_type: i8,
            /// null
            pub acls: Vec<Acls>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Acls {
            /// The ACL principal
            pub principal: String,
            /// The ACL host
            pub host: String,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateAclsRequest {
    V0 {
        /// null
        creations: Vec<create_acls_request::v0::Creations>,
    },
    V1 {
        /// null
        creations: Vec<create_acls_request::v1::Creations>,
    },
}

pub mod create_acls_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Creations {
            /// The resource type
            pub resource_type: i8,
            /// The resource name
            pub resource_name: String,
            /// The ACL principal
            pub principal: String,
            /// The ACL host
            pub host: String,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Creations {
            /// The resource type
            pub resource_type: i8,
            /// The resource name
            pub resource_name: String,
            /// The resource pattern type
            pub resource_pattten_type: i8,
            /// The ACL principal
            pub principal: String,
            /// The ACL host
            pub host: String,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateAclsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        creation_responses: Vec<create_acls_response::v0::CreationResponses>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        creation_responses: Vec<create_acls_response::v1::CreationResponses>,
    },
}

pub mod create_acls_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreationResponses {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct CreationResponses {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteAclsRequest {
    V0 {
        /// null
        filters: Vec<delete_acls_request::v0::Filters>,
    },
    V1 {
        /// null
        filters: Vec<delete_acls_request::v1::Filters>,
    },
}

pub mod delete_acls_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Filters {
            /// The resource type
            pub resource_type: i8,
            /// The resource name filter
            pub resource_name: crate::types::NullableString,
            /// The ACL principal filter
            pub principal: crate::types::NullableString,
            /// The ACL host filter
            pub host: crate::types::NullableString,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Filters {
            /// The resource type
            pub resource_type: i8,
            /// The resource name filter
            pub resource_name: crate::types::NullableString,
            /// The resource pattern type filter
            pub resource_pattern_type_filter: i8,
            /// The ACL principal filter
            pub principal: crate::types::NullableString,
            /// The ACL host filter
            pub host: crate::types::NullableString,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteAclsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        filter_responses: Vec<delete_acls_response::v0::FilterResponses>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        filter_responses: Vec<delete_acls_response::v1::FilterResponses>,
    },
}

pub mod delete_acls_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct FilterResponses {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// The matching ACLs
            pub matching_acls: Vec<MatchingAcls>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct MatchingAcls {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// The resource type
            pub resource_type: i8,
            /// The resource name
            pub resource_name: String,
            /// The ACL principal
            pub principal: String,
            /// The ACL host
            pub host: String,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct FilterResponses {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// The matching ACLs
            pub matching_acls: Vec<MatchingAcls>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct MatchingAcls {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// The resource type
            pub resource_type: i8,
            /// The resource name
            pub resource_name: String,
            /// The resource pattern type
            pub resource_pattten_type: i8,
            /// The ACL principal
            pub principal: String,
            /// The ACL host
            pub host: String,
            /// The ACL operation
            pub operation: i8,
            /// The ACL permission type
            pub permission_type: i8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeConfigsRequest {
    V0 {
        /// An array of config resources to be returned.
        resources: Vec<describe_configs_request::v0::Resources>,
    },
    V1 {
        /// An array of config resources to be returned.
        resources: Vec<describe_configs_request::v1::Resources>,
        /// null
        include_synonyms: bool,
    },
    V2 {
        /// An array of config resources to be returned.
        resources: Vec<describe_configs_request::v2::Resources>,
        /// null
        include_synonyms: bool,
    },
}

pub mod describe_configs_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_names: Vec<String>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_names: Vec<String>,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_names: Vec<String>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeConfigsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        resources: Vec<describe_configs_response::v0::Resources>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        resources: Vec<describe_configs_response::v1::Resources>,
    },
    V2 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        resources: Vec<describe_configs_response::v2::Resources>,
    },
}

pub mod describe_configs_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// null
            pub config_name: String,
            /// null
            pub config_value: crate::types::NullableString,
            /// null
            pub read_only: bool,
            /// null
            pub is_default: bool,
            /// null
            pub is_sensitive: bool,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// null
            pub config_name: String,
            /// null
            pub config_value: crate::types::NullableString,
            /// null
            pub read_only: bool,
            /// null
            pub config_source: i8,
            /// null
            pub is_sensitive: bool,
            /// null
            pub config_synonyms: Vec<ConfigSynonyms>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigSynonyms {
            /// null
            pub config_name: String,
            /// null
            pub config_value: crate::types::NullableString,
            /// null
            pub config_source: i8,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// null
            pub config_name: String,
            /// null
            pub config_value: crate::types::NullableString,
            /// null
            pub read_only: bool,
            /// null
            pub config_source: i8,
            /// null
            pub is_sensitive: bool,
            /// null
            pub config_synonyms: Vec<ConfigSynonyms>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigSynonyms {
            /// null
            pub config_name: String,
            /// null
            pub config_value: crate::types::NullableString,
            /// null
            pub config_source: i8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterConfigsRequest {
    V0 {
        /// An array of resources to update with the provided configs.
        resources: Vec<alter_configs_request::v0::Resources>,
        /// null
        validate_only: bool,
    },
    V1 {
        /// An array of resources to update with the provided configs.
        resources: Vec<alter_configs_request::v1::Resources>,
        /// null
        validate_only: bool,
    },
}

pub mod alter_configs_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// Configuration name
            pub config_name: String,
            /// Configuration value
            pub config_value: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
            /// null
            pub config_entries: Vec<ConfigEntries>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ConfigEntries {
            /// Configuration name
            pub config_name: String,
            /// Configuration value
            pub config_value: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterConfigsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        resources: Vec<alter_configs_response::v0::Resources>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        resources: Vec<alter_configs_response::v1::Resources>,
    },
}

pub mod alter_configs_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
            /// null
            pub resource_type: i8,
            /// null
            pub resource_name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterReplicaLogDirsRequest {
    V0 {
        /// null
        log_dirs: Vec<alter_replica_log_dirs_request::v0::LogDirs>,
    },
    V1 {
        /// null
        log_dirs: Vec<alter_replica_log_dirs_request::v1::LogDirs>,
    },
}

pub mod alter_replica_log_dirs_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LogDirs {
            /// The absolute log directory path.
            pub log_dir: String,
            /// null
            pub topics: Vec<Topics>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// List of partition ids of the topic.
            pub partitions: Vec<i32>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LogDirs {
            /// The absolute log directory path.
            pub log_dir: String,
            /// null
            pub topics: Vec<Topics>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// List of partition ids of the topic.
            pub partitions: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterReplicaLogDirsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        topics: Vec<alter_replica_log_dirs_response::v0::Topics>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        topics: Vec<alter_replica_log_dirs_response::v1::Topics>,
    },
}

pub mod alter_replica_log_dirs_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeLogDirsRequest {
    V0 {
        /// null
        topics: Vec<describe_log_dirs_request::v0::Topics>,
    },
    V1 {
        /// null
        topics: Vec<describe_log_dirs_request::v1::Topics>,
    },
}

pub mod describe_log_dirs_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// List of partition ids of the topic.
            pub partitions: Vec<i32>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// List of partition ids of the topic.
            pub partitions: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeLogDirsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        log_dirs: Vec<describe_log_dirs_response::v0::LogDirs>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// null
        log_dirs: Vec<describe_log_dirs_response::v1::LogDirs>,
    },
}

pub mod describe_log_dirs_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LogDirs {
            /// Response error code
            pub error_code: i16,
            /// The absolute log directory path.
            pub log_dir: String,
            /// null
            pub topics: Vec<Topics>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The size of the log segments of the partition in bytes.
            pub size: i64,
            /// The lag of the log's LEO w.r.t. partition's HW (if it is the current
            /// log for the partition) or current replica's LEO (if it is the future
            /// log for the partition)
            pub offset_lag: i64,
            /// True if this log is created by AlterReplicaLogDirsRequest and will
            /// replace the current log of the replica in the future.
            pub is_future: bool,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct LogDirs {
            /// Response error code
            pub error_code: i16,
            /// The absolute log directory path.
            pub log_dir: String,
            /// null
            pub topics: Vec<Topics>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// Name of topic
            pub topic: String,
            /// null
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// Topic partition id
            pub partition: i32,
            /// The size of the log segments of the partition in bytes.
            pub size: i64,
            /// The lag of the log's LEO w.r.t. partition's HW (if it is the current
            /// log for the partition) or current replica's LEO (if it is the future
            /// log for the partition)
            pub offset_lag: i64,
            /// True if this log is created by AlterReplicaLogDirsRequest and will
            /// replace the current log of the replica in the future.
            pub is_future: bool,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SaslAuthenticateRequest {
    V0 {
        /// SASL authentication bytes from client as defined by the SASL mechanism.
        sasl_auth_bytes: crate::types::Bytes,
    },
    V1 {
        /// SASL authentication bytes from client as defined by the SASL mechanism.
        sasl_auth_bytes: crate::types::Bytes,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SaslAuthenticateResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// Response error message
        error_message: crate::types::NullableString,
        /// SASL authentication bytes from server as defined by the SASL mechanism.
        sasl_auth_bytes: crate::types::Bytes,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// Response error message
        error_message: crate::types::NullableString,
        /// SASL authentication bytes from server as defined by the SASL mechanism.
        sasl_auth_bytes: crate::types::Bytes,
        /// Number of milliseconds after which only re-authentication over the
        /// existing connection to create a new session can occur.
        session_lifetime_ms: i64,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreatePartitionsRequest {
    V0 {
        /// List of topic and the corresponding new partitions.
        topic_partitions: Vec<create_partitions_request::v0::TopicPartitions>,
        /// The time in ms to wait for the partitions to be created.
        timeout: i32,
        /// If true then validate the request, but don't actually increase the
        /// number of partitions.
        validate_only: bool,
    },
    V1 {
        /// List of topic and the corresponding new partitions.
        topic_partitions: Vec<create_partitions_request::v1::TopicPartitions>,
        /// The time in ms to wait for the partitions to be created.
        timeout: i32,
        /// If true then validate the request, but don't actually increase the
        /// number of partitions.
        validate_only: bool,
    },
}

pub mod create_partitions_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicPartitions {
            /// Name of topic
            pub topic: String,
            /// null
            pub new_partitions: NewPartitions,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct NewPartitions {
            /// The new partition count.
            pub count: i32,
            /// The assigned brokers.
            pub assignment: Vec<Vec<i32>>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicPartitions {
            /// Name of topic
            pub topic: String,
            /// null
            pub new_partitions: NewPartitions,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct NewPartitions {
            /// The new partition count.
            pub count: i32,
            /// The assigned brokers.
            pub assignment: Vec<Vec<i32>>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreatePartitionsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Per topic results for the create partitions request
        topic_errors: Vec<create_partitions_response::v0::TopicErrors>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Per topic results for the create partitions request
        topic_errors: Vec<create_partitions_response::v1::TopicErrors>,
    },
}

pub mod create_partitions_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrors {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicErrors {
            /// Name of topic
            pub topic: String,
            /// Response error code
            pub error_code: i16,
            /// Response error message
            pub error_message: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateDelegationTokenRequest {
    V0 {
        /// An array of token renewers. Renewer is an Kafka PrincipalType and name
        /// string, who is allowed to renew this token before the max lifetime
        /// expires.
        renewers: Vec<create_delegation_token_request::v0::Renewers>,
        /// Max lifetime period for token in milli seconds. if value is -1, then
        /// max lifetime  will default to a server side config value.
        max_life_time: i64,
    },
    V1 {
        /// An array of token renewers. Renewer is an Kafka PrincipalType and name
        /// string, who is allowed to renew this token before the max lifetime
        /// expires.
        renewers: Vec<create_delegation_token_request::v1::Renewers>,
        /// Max lifetime period for token in milli seconds. if value is -1, then
        /// max lifetime  will default to a server side config value.
        max_life_time: i64,
    },
}

pub mod create_delegation_token_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Renewers {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Renewers {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateDelegationTokenResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// token owner.
        owner: create_delegation_token_response::v0::Owner,
        /// timestamp (in msec) when this token was generated.
        issue_timestamp: i64,
        /// timestamp (in msec) at which this token expires.
        expiry_timestamp: i64,
        /// max life time of this token.
        max_timestamp: i64,
        /// UUID to ensure uniqueness.
        token_id: String,
        /// HMAC of the delegation token.
        hmac: crate::types::Bytes,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// token owner.
        owner: create_delegation_token_response::v1::Owner,
        /// timestamp (in msec) when this token was generated.
        issue_timestamp: i64,
        /// timestamp (in msec) at which this token expires.
        expiry_timestamp: i64,
        /// max life time of this token.
        max_timestamp: i64,
        /// UUID to ensure uniqueness.
        token_id: String,
        /// HMAC of the delegation token.
        hmac: crate::types::Bytes,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
}

pub mod create_delegation_token_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Owner {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Owner {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RenewDelegationTokenRequest {
    V0 {
        /// HMAC of the delegation token to be renewed.
        hmac: crate::types::Bytes,
        /// Renew time period in milli seconds.
        renew_time_period: i64,
    },
    V1 {
        /// HMAC of the delegation token to be renewed.
        hmac: crate::types::Bytes,
        /// Renew time period in milli seconds.
        renew_time_period: i64,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RenewDelegationTokenResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// timestamp (in msec) at which this token expires.
        expiry_timestamp: i64,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// timestamp (in msec) at which this token expires.
        expiry_timestamp: i64,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExpireDelegationTokenRequest {
    V0 {
        /// HMAC of the delegation token to be expired.
        hmac: crate::types::Bytes,
        /// expiry time period in milli seconds.
        expiry_time_period: i64,
    },
    V1 {
        /// HMAC of the delegation token to be expired.
        hmac: crate::types::Bytes,
        /// expiry time period in milli seconds.
        expiry_time_period: i64,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExpireDelegationTokenResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// timestamp (in msec) at which this token expires.
        expiry_timestamp: i64,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// timestamp (in msec) at which this token expires.
        expiry_timestamp: i64,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeDelegationTokenRequest {
    V0 {
        /// An array of token owners.
        owners: Vec<describe_delegation_token_request::v0::Owners>,
    },
    V1 {
        /// An array of token owners.
        owners: Vec<describe_delegation_token_request::v1::Owners>,
    },
}

pub mod describe_delegation_token_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Owners {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Owners {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeDelegationTokenResponse {
    V0 {
        /// Response error code
        error_code: i16,
        /// null
        token_details: Vec<describe_delegation_token_response::v0::TokenDetails>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
    V1 {
        /// Response error code
        error_code: i16,
        /// null
        token_details: Vec<describe_delegation_token_response::v1::TokenDetails>,
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
    },
}

pub mod describe_delegation_token_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TokenDetails {
            /// token owner.
            pub owner: Owner,
            /// timestamp (in msec) when this token was generated.
            pub issue_timestamp: i64,
            /// timestamp (in msec) at which this token expires.
            pub expiry_timestamp: i64,
            /// max life time of this token.
            pub max_timestamp: i64,
            /// UUID to ensure uniqueness.
            pub token_id: String,
            /// HMAC of the delegation token to be expired.
            pub hmac: crate::types::Bytes,
            /// An array of token renewers. Renewer is an Kafka PrincipalType and name
            /// string, who is allowed to renew this token before the max lifetime
            /// expires.
            pub renewers: Vec<Renewers>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Owner {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Renewers {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TokenDetails {
            /// token owner.
            pub owner: Owner,
            /// timestamp (in msec) when this token was generated.
            pub issue_timestamp: i64,
            /// timestamp (in msec) at which this token expires.
            pub expiry_timestamp: i64,
            /// max life time of this token.
            pub max_timestamp: i64,
            /// UUID to ensure uniqueness.
            pub token_id: String,
            /// HMAC of the delegation token to be expired.
            pub hmac: crate::types::Bytes,
            /// An array of token renewers. Renewer is an Kafka PrincipalType and name
            /// string, who is allowed to renew this token before the max lifetime
            /// expires.
            pub renewers: Vec<Renewers>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Owner {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Renewers {
            /// principalType of the Kafka principal
            pub principal_type: String,
            /// name of the Kafka principal
            pub name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteGroupsRequest {
    V0 {
        /// An array of groups to be deleted.
        groups: Vec<String>,
    },
    V1 {
        /// An array of groups to be deleted.
        groups: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteGroupsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per group error codes.
        group_error_codes: Vec<delete_groups_response::v0::GroupErrorCodes>,
    },
    V1 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of per group error codes.
        group_error_codes: Vec<delete_groups_response::v1::GroupErrorCodes>,
    },
}

pub mod delete_groups_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupErrorCodes {
            /// The unique group identifier
            pub group_id: String,
            /// Response error code
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct GroupErrorCodes {
            /// The unique group identifier
            pub group_id: String,
            /// Response error code
            pub error_code: i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ElectPreferredLeadersRequest {
    V0 {
        /// The topic partitions to elect the preferred leader of.
        topic_partitions: Vec<elect_preferred_leaders_request::v0::TopicPartitions>,
        /// The time in ms to wait for the election to complete.
        timeout_ms: i32,
    },
}

pub mod elect_preferred_leaders_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TopicPartitions {
            /// The name of a topic.
            pub topic: String,
            /// The partitions of this topic whose preferred leader should be elected
            pub partition_id: Vec<i32>,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ElectPreferredLeadersResponse {
    V0 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        replica_election_results: Vec<elect_preferred_leaders_response::v0::ReplicaElectionResults>,
    },
}

pub mod elect_preferred_leaders_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ReplicaElectionResults {
            /// The topic name
            pub topic: String,
            /// The results for each partition
            pub partition_result: Vec<PartitionResult>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PartitionResult {
            /// The partition id
            pub partition_id: i32,
            /// The result error, or zero if there was no error.
            pub error_code: i16,
            /// The result message, or null if there was no error.
            pub error_message: crate::types::NullableString,
        }
    }
}
