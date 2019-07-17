//! Generated from: https://kafka.apache.org/23/protocol.html

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    /// Retriable: No.
    UnknownServerError = -1,
    /// Retriable: No.
    None = 0,
    /// The requested offset is not within the range of offsets maintained by
    /// the server. Retriable: No.
    OffsetOutOfRange = 1,
    /// This message has failed its CRC checksum, exceeds the valid size, has
    /// a null key for a compacted topic, or is otherwise corrupt. Retriable:
    /// Yes.
    CorruptMessage = 2,
    /// This server does not host this topic-partition. Retriable: Yes.
    UnknownTopicOrPartition = 3,
    /// The requested fetch size is invalid. Retriable: No.
    InvalidFetchSize = 4,
    /// There is no leader for this topic-partition as we are in the middle of
    /// a leadership election. Retriable: Yes.
    LeaderNotAvailable = 5,
    /// This server is not the leader for that topic-partition. Retriable: Yes.
    NotLeaderForPartition = 6,
    /// The request timed out. Retriable: Yes.
    RequestTimedOut = 7,
    /// The broker is not available. Retriable: No.
    BrokerNotAvailable = 8,
    /// The replica is not available for the requested topic-partition.
    /// Retriable: No.
    ReplicaNotAvailable = 9,
    /// The request included a message larger than the max message size the
    /// server will accept. Retriable: No.
    MessageTooLarge = 10,
    /// The controller moved to another broker. Retriable: No.
    StaleControllerEpoch = 11,
    /// The metadata field of the offset request was too large. Retriable: No.
    OffsetMetadataTooLarge = 12,
    /// The server disconnected before a response was received. Retriable: Yes.
    NetworkException = 13,
    /// The coordinator is loading and hence can't process requests. Retriable:
    /// Yes.
    CoordinatorLoadInProgress = 14,
    /// The coordinator is not available. Retriable: Yes.
    CoordinatorNotAvailable = 15,
    /// This is not the correct coordinator. Retriable: Yes.
    NotCoordinator = 16,
    /// The request attempted to perform an operation on an invalid topic.
    /// Retriable: No.
    InvalidTopicException = 17,
    /// The request included message batch larger than the configured segment
    /// size on the server. Retriable: No.
    RecordListTooLarge = 18,
    /// Messages are rejected since there are fewer in-sync replicas than
    /// required. Retriable: Yes.
    NotEnoughReplicas = 19,
    /// Messages are written to the log, but to fewer in-sync replicas than
    /// required. Retriable: Yes.
    NotEnoughReplicasAfterAppend = 20,
    /// Produce request specified an invalid value for required acks.
    /// Retriable: No.
    InvalidRequiredAcks = 21,
    /// Specified group generation id is not valid. Retriable: No.
    IllegalGeneration = 22,
    /// The group member's supported protocols are incompatible with those of
    /// existing members or first group member tried to join with empty
    /// protocol type or empty protocol list. Retriable: No.
    InconsistentGroupProtocol = 23,
    /// The configured groupId is invalid. Retriable: No.
    InvalidGroupId = 24,
    /// The coordinator is not aware of this member. Retriable: No.
    UnknownMemberId = 25,
    /// The session timeout is not within the range allowed by the broker (as
    /// configured by group.min.session.timeout.ms and group.max.session.
    /// timeout.ms). Retriable: No.
    InvalidSessionTimeout = 26,
    /// The group is rebalancing, so a rejoin is needed. Retriable: No.
    RebalanceInProgress = 27,
    /// The committing offset data size is not valid. Retriable: No.
    InvalidCommitOffsetSize = 28,
    /// Not authorized to access topics: [Topic authorization failed.]
    /// Retriable: No.
    TopicAuthorizationFailed = 29,
    /// Not authorized to access group: Group authorization failed. Retriable:
    /// No.
    GroupAuthorizationFailed = 30,
    /// Cluster authorization failed. Retriable: No.
    ClusterAuthorizationFailed = 31,
    /// The timestamp of the message is out of acceptable range. Retriable: No.
    InvalidTimestamp = 32,
    /// The broker does not support the requested SASL mechanism. Retriable:
    /// No.
    UnsupportedSaslMechanism = 33,
    /// Request is not valid given the current SASL state. Retriable: No.
    IllegalSaslState = 34,
    /// The version of API is not supported. Retriable: No.
    UnsupportedVersion = 35,
    /// Topic with this name already exists. Retriable: No.
    TopicAlreadyExists = 36,
    /// Number of partitions is below 1. Retriable: No.
    InvalidPartitions = 37,
    /// Replication factor is below 1 or larger than the number of available
    /// brokers. Retriable: No.
    InvalidReplicationFactor = 38,
    /// Replica assignment is invalid. Retriable: No.
    InvalidReplicaAssignment = 39,
    /// Configuration is invalid. Retriable: No.
    InvalidConfig = 40,
    /// This is not the correct controller for this cluster. Retriable: Yes.
    NotController = 41,
    /// This most likely occurs because of a request being malformed by the
    /// client library or the message was sent to an incompatible broker. See
    /// the broker logs for more details. Retriable: No.
    InvalidRequest = 42,
    /// The message format version on the broker does not support the request.
    /// Retriable: No.
    UnsupportedForMessageFormat = 43,
    /// Request parameters do not satisfy the configured policy. Retriable: No.
    PolicyViolation = 44,
    /// The broker received an out of order sequence number. Retriable: No.
    OutOfOrderSequenceNumber = 45,
    /// The broker received a duplicate sequence number. Retriable: No.
    DuplicateSequenceNumber = 46,
    /// Producer attempted an operation with an old epoch. Either there is a
    /// newer producer with the same transactionalId, or the producer's
    /// transaction has been expired by the broker. Retriable: No.
    InvalidProducerEpoch = 47,
    /// The producer attempted a transactional operation in an invalid state.
    /// Retriable: No.
    InvalidTxnState = 48,
    /// The producer attempted to use a producer id which is not currently
    /// assigned to its transactional id. Retriable: No.
    InvalidProducerIdMapping = 49,
    /// The transaction timeout is larger than the maximum value allowed by
    /// the broker (as configured by transaction.max.timeout.ms). Retriable:
    /// No.
    InvalidTransactionTimeout = 50,
    /// The producer attempted to update a transaction while another
    /// concurrent operation on the same transaction was ongoing. Retriable:
    /// No.
    ConcurrentTransactions = 51,
    /// Indicates that the transaction coordinator sending a WriteTxnMarker is
    /// no longer the current coordinator for a given producer. Retriable: No.
    TransactionCoordinatorFenced = 52,
    /// Transactional Id authorization failed. Retriable: No.
    TransactionalIdAuthorizationFailed = 53,
    /// Security features are disabled. Retriable: No.
    SecurityDisabled = 54,
    /// The broker did not attempt to execute this operation. This may happen
    /// for batched RPCs where some operations in the batch failed, causing
    /// the broker to respond without trying the rest. Retriable: No.
    OperationNotAttempted = 55,
    /// Disk error when trying to access log file on the disk. Retriable: Yes.
    KafkaStorageError = 56,
    /// The user-specified log directory is not found in the broker config.
    /// Retriable: No.
    LogDirNotFound = 57,
    /// SASL Authentication failed. Retriable: No.
    SaslAuthenticationFailed = 58,
    /// This exception is raised by the broker if it could not locate the
    /// producer metadata associated with the producerId in question. This
    /// could happen if, for instance, the producer's records were deleted
    /// because their retention time had elapsed. Once the last records of the
    /// producerId are removed, the producer's metadata is removed from the
    /// broker, and future appends by the producer will return this exception.
    /// Retriable: No.
    UnknownProducerId = 59,
    /// A partition reassignment is in progress. Retriable: No.
    ReassignmentInProgress = 60,
    /// Delegation Token feature is not enabled. Retriable: No.
    DelegationTokenAuthDisabled = 61,
    /// Delegation Token is not found on server. Retriable: No.
    DelegationTokenNotFound = 62,
    /// Specified Principal is not valid Owner/Renewer. Retriable: No.
    DelegationTokenOwnerMismatch = 63,
    /// Delegation Token requests are not allowed on PLAINTEXT/1-way SSL
    /// channels and on delegation token authenticated channels. Retriable: No.
    DelegationTokenRequestNotAllowed = 64,
    /// Delegation Token authorization failed. Retriable: No.
    DelegationTokenAuthorizationFailed = 65,
    /// Delegation Token is expired. Retriable: No.
    DelegationTokenExpired = 66,
    /// Supplied principalType is not supported. Retriable: No.
    InvalidPrincipalType = 67,
    /// The group is not empty. Retriable: No.
    NonEmptyGroup = 68,
    /// The group id does not exist. Retriable: No.
    GroupIdNotFound = 69,
    /// The fetch session ID was not found. Retriable: Yes.
    FetchSessionIdNotFound = 70,
    /// The fetch session epoch is invalid. Retriable: Yes.
    InvalidFetchSessionEpoch = 71,
    /// There is no listener on the leader broker that matches the listener on
    /// which metadata request was processed. Retriable: Yes.
    ListenerNotFound = 72,
    /// Topic deletion is disabled. Retriable: No.
    TopicDeletionDisabled = 73,
    /// The leader epoch in the request is older than the epoch on the broker
    /// Retriable: Yes.
    FencedLeaderEpoch = 74,
    /// The leader epoch in the request is newer than the epoch on the broker
    /// Retriable: Yes.
    UnknownLeaderEpoch = 75,
    /// The requesting client does not support the compression type of given
    /// partition. Retriable: No.
    UnsupportedCompressionType = 76,
    /// Broker epoch has changed Retriable: No.
    StaleBrokerEpoch = 77,
    /// The leader high watermark has not caught up from a recent leader
    /// election so the offsets cannot be guaranteed to be monotonically
    /// increasing Retriable: Yes.
    OffsetNotAvailable = 78,
    /// The group member needs to have a valid member id before actually
    /// entering a consumer group Retriable: No.
    MemberIdRequired = 79,
    /// The preferred leader was not available Retriable: Yes.
    PreferredLeaderNotAvailable = 80,
    /// Consumer group The consumer group has reached its max size. already
    /// has the configured maximum number of members. Retriable: No.
    GroupMaxSizeReached = 81,
    /// The broker rejected this static consumer since another consumer with
    /// the same group.instance.id has registered with a different member.id.
    /// Retriable: No.
    FencedInstanceId = 82,
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
    IncrementalAlterConfigs = 44,
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
    V11 {
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
        topics: Vec<fetch_request::v11::Topics>,
        /// Topics to remove from the fetch session.
        forgotten_topics_data: Vec<fetch_request::v11::ForgottenTopicsData>,
        /// The consumer's rack id
        rack_id: String,
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
    pub mod v11 {
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
    V11 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// Response error code
        error_code: i16,
        /// The fetch session ID
        session_id: i32,
        /// null
        responses: Vec<fetch_response::v11::Responses>,
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
    pub mod v11 {
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
            /// The ID of the replica that the consumer should prefer.
            pub preferred_read_replica: i32,
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
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v0::Topics>,
    },
    V1 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v1::Topics>,
    },
    V2 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v2::Topics>,
    },
    V3 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v3::Topics>,
    },
    V4 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v4::Topics>,
        /// If this is true, the broker may auto-create topics that we requested
        /// which do not already exist, if it is configured to do so.
        allow_auto_topic_creation: bool,
    },
    V5 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v5::Topics>,
        /// If this is true, the broker may auto-create topics that we requested
        /// which do not already exist, if it is configured to do so.
        allow_auto_topic_creation: bool,
    },
    V6 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v6::Topics>,
        /// If this is true, the broker may auto-create topics that we requested
        /// which do not already exist, if it is configured to do so.
        allow_auto_topic_creation: bool,
    },
    V7 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v7::Topics>,
        /// If this is true, the broker may auto-create topics that we requested
        /// which do not already exist, if it is configured to do so.
        allow_auto_topic_creation: bool,
    },
    V8 {
        /// The topics to fetch metadata for.
        topics: Vec<metadata_request::v8::Topics>,
        /// If this is true, the broker may auto-create topics that we requested
        /// which do not already exist, if it is configured to do so.
        allow_auto_topic_creation: bool,
        /// Whether to include cluster authorized operations.
        include_cluster_authorized_operations: bool,
        /// Whether to include topic authorized operations.
        include_topic_authorized_operations: bool,
    },
}

pub mod metadata_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
    pub mod v8 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MetadataResponse {
    V0 {
        /// Each broker in the response.
        brokers: Vec<metadata_response::v0::Brokers>,
        /// Each topic in the response.
        topics: Vec<metadata_response::v0::Topics>,
    },
    V1 {
        /// Each broker in the response.
        brokers: Vec<metadata_response::v1::Brokers>,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v1::Topics>,
    },
    V2 {
        /// Each broker in the response.
        brokers: Vec<metadata_response::v2::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v2::Topics>,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each broker in the response.
        brokers: Vec<metadata_response::v3::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v3::Topics>,
    },
    V4 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each broker in the response.
        brokers: Vec<metadata_response::v4::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v4::Topics>,
    },
    V5 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each broker in the response.
        brokers: Vec<metadata_response::v5::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v5::Topics>,
    },
    V6 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each broker in the response.
        brokers: Vec<metadata_response::v6::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v6::Topics>,
    },
    V7 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each broker in the response.
        brokers: Vec<metadata_response::v7::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v7::Topics>,
    },
    V8 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each broker in the response.
        brokers: Vec<metadata_response::v8::Brokers>,
        /// The cluster ID that responding broker belongs to.
        cluster_id: crate::types::NullableString,
        /// The ID of the controller broker.
        controller_id: i32,
        /// Each topic in the response.
        topics: Vec<metadata_response::v8::Topics>,
        /// 32-bit bitfield to represent authorized operations for this cluster.
        cluster_authorized_operations: i32,
    },
}

pub mod metadata_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
            /// The set of offline replicas of this partition.
            pub offline_replicas: Vec<i32>,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
            /// The set of offline replicas of this partition.
            pub offline_replicas: Vec<i32>,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The leader epoch of this partition.
            pub leader_epoch: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
            /// The set of offline replicas of this partition.
            pub offline_replicas: Vec<i32>,
        }
    }
    pub mod v8 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Brokers {
            /// The broker ID.
            pub node_id: i32,
            /// The broker hostname.
            pub host: String,
            /// The broker port.
            pub port: i32,
            /// The rack of the broker, or null if it has not been assigned to a rack.
            pub rack: crate::types::NullableString,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The topic name.
            pub name: String,
            /// True if the topic is internal.
            pub is_internal: bool,
            /// Each partition in the topic.
            pub partitions: Vec<Partitions>,
            /// 32-bit bitfield to represent authorized operations for this topic.
            pub topic_authorized_operations: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition error, or 0 if there was no error.
            pub error_code: i16,
            /// The partition index.
            pub partition_index: i32,
            /// The ID of the leader broker.
            pub leader_id: i32,
            /// The leader epoch of this partition.
            pub leader_epoch: i32,
            /// The set of all nodes that host this partition.
            pub replica_nodes: Vec<i32>,
            /// The set of nodes that are in sync with the leader for this partition.
            pub isr_nodes: Vec<i32>,
            /// The set of offline replicas of this partition.
            pub offline_replicas: Vec<i32>,
        }
    }
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
        /// The broker epoch.
        broker_epoch: i64,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ControlledShutdownResponse {
    V0 {
        /// The top-level error code.
        error_code: i16,
        /// The partitions that the broker still leads.
        remaining_partitions: Vec<controlled_shutdown_response::v0::RemainingPartitions>,
    },
    V1 {
        /// The top-level error code.
        error_code: i16,
        /// The partitions that the broker still leads.
        remaining_partitions: Vec<controlled_shutdown_response::v1::RemainingPartitions>,
    },
    V2 {
        /// The top-level error code.
        error_code: i16,
        /// The partitions that the broker still leads.
        remaining_partitions: Vec<controlled_shutdown_response::v2::RemainingPartitions>,
    },
}

pub mod controlled_shutdown_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct RemainingPartitions {
            /// The name of the topic.
            pub topic_name: String,
            /// The index of the partition.
            pub partition_index: i32,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct RemainingPartitions {
            /// The name of the topic.
            pub topic_name: String,
            /// The index of the partition.
            pub partition_index: i32,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct RemainingPartitions {
            /// The name of the topic.
            pub topic_name: String,
            /// The index of the partition.
            pub partition_index: i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetCommitRequest {
    V0 {
        /// The unique group identifier.
        group_id: String,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v0::Topics>,
    },
    V1 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v1::Topics>,
    },
    V2 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The time period in ms to retain the offset.
        retention_time_ms: i64,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v2::Topics>,
    },
    V3 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The time period in ms to retain the offset.
        retention_time_ms: i64,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v3::Topics>,
    },
    V4 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The time period in ms to retain the offset.
        retention_time_ms: i64,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v4::Topics>,
    },
    V5 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v5::Topics>,
    },
    V6 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v6::Topics>,
    },
    V7 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID assigned by the group coordinator.
        member_id: String,
        /// The unique identifier of the consumer instance provided by end user.
        group_instance_id: crate::types::NullableString,
        /// The topics to commit offsets for.
        topics: Vec<offset_commit_request::v7::Topics>,
    },
}

pub mod offset_commit_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// The timestamp of the commit.
            pub commit_timestamp: i64,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// The leader epoch of this partition.
            pub committed_leader_epoch: i32,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// Each partition to commit offsets for.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The message offset to be committed.
            pub committed_offset: i64,
            /// The leader epoch of this partition.
            pub committed_leader_epoch: i32,
            /// Any associated metadata the client wants to keep.
            pub committed_metadata: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OffsetCommitResponse {
    V0 {
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v0::Topics>,
    },
    V1 {
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v1::Topics>,
    },
    V2 {
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v2::Topics>,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v3::Topics>,
    },
    V4 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v4::Topics>,
    },
    V5 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v5::Topics>,
    },
    V6 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v6::Topics>,
    },
    V7 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The responses for each topic.
        topics: Vec<offset_commit_response::v7::Topics>,
    },
}

pub mod offset_commit_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v6 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v7 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The responses for each partition in the topic.
            pub partitions: Vec<Partitions>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Partitions {
            /// The partition index.
            pub partition_index: i32,
            /// The error code, or 0 if there was no error.
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
        /// The coordinator key.
        key: String,
    },
    V1 {
        /// The coordinator key.
        key: String,
        /// The coordinator key type.  (Group, transaction, etc.
        key_type: i8,
    },
    V2 {
        /// The coordinator key.
        key: String,
        /// The coordinator key type.  (Group, transaction, etc.
        key_type: i8,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FindCoordinatorResponse {
    V0 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The node id.
        node_id: i32,
        /// The host name.
        host: String,
        /// The port.
        port: i32,
    },
    V1 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The error message, or null if there was no error.
        error_message: crate::types::NullableString,
        /// The node id.
        node_id: i32,
        /// The host name.
        host: String,
        /// The port.
        port: i32,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The error message, or null if there was no error.
        error_message: crate::types::NullableString,
        /// The node id.
        node_id: i32,
        /// The host name.
        host: String,
        /// The port.
        port: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum JoinGroupRequest {
    V0 {
        /// The group identifier.
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in milliseconds.
        session_timeout_ms: i32,
        /// The member id assigned by the group coordinator.
        member_id: String,
        /// The unique name the for class of protocols implemented by the group we
        /// want to join.
        protocol_type: String,
        /// The list of protocols that the member supports.
        protocols: Vec<join_group_request::v0::Protocols>,
    },
    V1 {
        /// The group identifier.
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in milliseconds.
        session_timeout_ms: i32,
        /// The maximum time in milliseconds that the coordinator will wait for
        /// each member to rejoin when rebalancing the group.
        rebalance_timeout_ms: i32,
        /// The member id assigned by the group coordinator.
        member_id: String,
        /// The unique name the for class of protocols implemented by the group we
        /// want to join.
        protocol_type: String,
        /// The list of protocols that the member supports.
        protocols: Vec<join_group_request::v1::Protocols>,
    },
    V2 {
        /// The group identifier.
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in milliseconds.
        session_timeout_ms: i32,
        /// The maximum time in milliseconds that the coordinator will wait for
        /// each member to rejoin when rebalancing the group.
        rebalance_timeout_ms: i32,
        /// The member id assigned by the group coordinator.
        member_id: String,
        /// The unique name the for class of protocols implemented by the group we
        /// want to join.
        protocol_type: String,
        /// The list of protocols that the member supports.
        protocols: Vec<join_group_request::v2::Protocols>,
    },
    V3 {
        /// The group identifier.
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in milliseconds.
        session_timeout_ms: i32,
        /// The maximum time in milliseconds that the coordinator will wait for
        /// each member to rejoin when rebalancing the group.
        rebalance_timeout_ms: i32,
        /// The member id assigned by the group coordinator.
        member_id: String,
        /// The unique name the for class of protocols implemented by the group we
        /// want to join.
        protocol_type: String,
        /// The list of protocols that the member supports.
        protocols: Vec<join_group_request::v3::Protocols>,
    },
    V4 {
        /// The group identifier.
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in milliseconds.
        session_timeout_ms: i32,
        /// The maximum time in milliseconds that the coordinator will wait for
        /// each member to rejoin when rebalancing the group.
        rebalance_timeout_ms: i32,
        /// The member id assigned by the group coordinator.
        member_id: String,
        /// The unique name the for class of protocols implemented by the group we
        /// want to join.
        protocol_type: String,
        /// The list of protocols that the member supports.
        protocols: Vec<join_group_request::v4::Protocols>,
    },
    V5 {
        /// The group identifier.
        group_id: String,
        /// The coordinator considers the consumer dead if it receives no
        /// heartbeat after this timeout in milliseconds.
        session_timeout_ms: i32,
        /// The maximum time in milliseconds that the coordinator will wait for
        /// each member to rejoin when rebalancing the group.
        rebalance_timeout_ms: i32,
        /// The member id assigned by the group coordinator.
        member_id: String,
        /// The unique identifier of the consumer instance provided by end user.
        group_instance_id: crate::types::NullableString,
        /// The unique name the for class of protocols implemented by the group we
        /// want to join.
        protocol_type: String,
        /// The list of protocols that the member supports.
        protocols: Vec<join_group_request::v5::Protocols>,
    },
}

pub mod join_group_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Protocols {
            /// The protocol name.
            pub name: String,
            /// The protocol metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Protocols {
            /// The protocol name.
            pub name: String,
            /// The protocol metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Protocols {
            /// The protocol name.
            pub name: String,
            /// The protocol metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Protocols {
            /// The protocol name.
            pub name: String,
            /// The protocol metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Protocols {
            /// The protocol name.
            pub name: String,
            /// The protocol metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Protocols {
            /// The protocol name.
            pub name: String,
            /// The protocol metadata.
            pub metadata: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum JoinGroupResponse {
    V0 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The generation ID of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator.
        protocol_name: String,
        /// The leader of the group.
        leader: String,
        /// The group member ID.
        member_id: String,

        members: Vec<join_group_response::v0::Members>,
    },
    V1 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The generation ID of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator.
        protocol_name: String,
        /// The leader of the group.
        leader: String,
        /// The group member ID.
        member_id: String,

        members: Vec<join_group_response::v1::Members>,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The generation ID of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator.
        protocol_name: String,
        /// The leader of the group.
        leader: String,
        /// The group member ID.
        member_id: String,

        members: Vec<join_group_response::v2::Members>,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The generation ID of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator.
        protocol_name: String,
        /// The leader of the group.
        leader: String,
        /// The group member ID.
        member_id: String,

        members: Vec<join_group_response::v3::Members>,
    },
    V4 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The generation ID of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator.
        protocol_name: String,
        /// The leader of the group.
        leader: String,
        /// The group member ID.
        member_id: String,

        members: Vec<join_group_response::v4::Members>,
    },
    V5 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The generation ID of the group.
        generation_id: i32,
        /// The group protocol selected by the coordinator.
        protocol_name: String,
        /// The leader of the group.
        leader: String,
        /// The group member ID.
        member_id: String,

        members: Vec<join_group_response::v5::Members>,
    },
}

pub mod join_group_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The group member ID.
            pub member_id: String,
            /// The group member metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The group member ID.
            pub member_id: String,
            /// The group member metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The group member ID.
            pub member_id: String,
            /// The group member metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The group member ID.
            pub member_id: String,
            /// The group member metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v4 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The group member ID.
            pub member_id: String,
            /// The group member metadata.
            pub metadata: crate::types::Bytes,
        }
    }
    pub mod v5 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The group member ID.
            pub member_id: String,
            /// The unique identifier of the consumer instance provided by end user.
            pub group_instance_id: crate::types::NullableString,
            /// The group member metadata.
            pub metadata: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HeartbeatRequest {
    V0 {
        /// The group id.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID.
        member_id: String,
    },
    V1 {
        /// The group id.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID.
        member_id: String,
    },
    V2 {
        /// The group id.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID.
        member_id: String,
    },
    V3 {
        /// The group id.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The member ID.
        member_id: String,
        /// The unique identifier of the consumer instance provided by end user.
        group_instance_id: crate::types::NullableString,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HeartbeatResponse {
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
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
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
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The ID of the member to assign.
        member_id: String,
        /// Each assignment.
        assignments: Vec<sync_group_request::v0::Assignments>,
    },
    V1 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The ID of the member to assign.
        member_id: String,
        /// Each assignment.
        assignments: Vec<sync_group_request::v1::Assignments>,
    },
    V2 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The ID of the member to assign.
        member_id: String,
        /// Each assignment.
        assignments: Vec<sync_group_request::v2::Assignments>,
    },
    V3 {
        /// The unique group identifier.
        group_id: String,
        /// The generation of the group.
        generation_id: i32,
        /// The ID of the member to assign.
        member_id: String,
        /// The unique identifier of the consumer instance provided by end user.
        group_instance_id: crate::types::NullableString,
        /// Each assignment.
        assignments: Vec<sync_group_request::v3::Assignments>,
    },
}

pub mod sync_group_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The ID of the member to assign.
            pub member_id: String,
            /// The member assignment.
            pub assignment: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The ID of the member to assign.
            pub member_id: String,
            /// The member assignment.
            pub assignment: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The ID of the member to assign.
            pub member_id: String,
            /// The member assignment.
            pub assignment: crate::types::Bytes,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The ID of the member to assign.
            pub member_id: String,
            /// The member assignment.
            pub assignment: crate::types::Bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SyncGroupResponse {
    V0 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The member assignment.
        assignment: crate::types::Bytes,
    },
    V1 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The member assignment.
        assignment: crate::types::Bytes,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The member assignment.
        assignment: crate::types::Bytes,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The member assignment.
        assignment: crate::types::Bytes,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeGroupsRequest {
    V0 {
        /// The names of the groups to describe
        groups: Vec<String>,
    },
    V1 {
        /// The names of the groups to describe
        groups: Vec<String>,
    },
    V2 {
        /// The names of the groups to describe
        groups: Vec<String>,
    },
    V3 {
        /// The names of the groups to describe
        groups: Vec<String>,
        /// Whether to include authorized operations.
        include_authorized_operations: bool,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DescribeGroupsResponse {
    V0 {
        /// Each described group.
        groups: Vec<describe_groups_response::v0::Groups>,
    },
    V1 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each described group.
        groups: Vec<describe_groups_response::v1::Groups>,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each described group.
        groups: Vec<describe_groups_response::v2::Groups>,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Each described group.
        groups: Vec<describe_groups_response::v3::Groups>,
    },
}

pub mod describe_groups_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The describe error, or 0 if there was no error.
            pub error_code: i16,
            /// The group ID string.
            pub group_id: String,
            /// The group state string, or the empty string.
            pub group_state: String,
            /// The group protocol type, or the empty string.
            pub protocol_type: String,
            /// The group protocol data, or the empty string.
            pub protocol_data: String,
            /// The group members.
            pub members: Vec<Members>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member ID assigned by the group coordinator.
            pub member_id: String,
            /// The client ID used in the member's latest join group request.
            pub client_id: String,
            /// The client host.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use.
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader.
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The describe error, or 0 if there was no error.
            pub error_code: i16,
            /// The group ID string.
            pub group_id: String,
            /// The group state string, or the empty string.
            pub group_state: String,
            /// The group protocol type, or the empty string.
            pub protocol_type: String,
            /// The group protocol data, or the empty string.
            pub protocol_data: String,
            /// The group members.
            pub members: Vec<Members>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member ID assigned by the group coordinator.
            pub member_id: String,
            /// The client ID used in the member's latest join group request.
            pub client_id: String,
            /// The client host.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use.
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader.
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The describe error, or 0 if there was no error.
            pub error_code: i16,
            /// The group ID string.
            pub group_id: String,
            /// The group state string, or the empty string.
            pub group_state: String,
            /// The group protocol type, or the empty string.
            pub protocol_type: String,
            /// The group protocol data, or the empty string.
            pub protocol_data: String,
            /// The group members.
            pub members: Vec<Members>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member ID assigned by the group coordinator.
            pub member_id: String,
            /// The client ID used in the member's latest join group request.
            pub client_id: String,
            /// The client host.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use.
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader.
            pub member_assignment: crate::types::Bytes,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Groups {
            /// The describe error, or 0 if there was no error.
            pub error_code: i16,
            /// The group ID string.
            pub group_id: String,
            /// The group state string, or the empty string.
            pub group_state: String,
            /// The group protocol type, or the empty string.
            pub protocol_type: String,
            /// The group protocol data, or the empty string.
            pub protocol_data: String,
            /// The group members.
            pub members: Vec<Members>,
            /// 32-bit bitfield to represent authorized operations for this group.
            pub authorized_operations: i32,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Members {
            /// The member ID assigned by the group coordinator.
            pub member_id: String,
            /// The client ID used in the member's latest join group request.
            pub client_id: String,
            /// The client host.
            pub client_host: String,
            /// The metadata corresponding to the current group protocol in use.
            pub member_metadata: crate::types::Bytes,
            /// The current assignment provided by the group leader.
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
        /// The SASL mechanism chosen by the client.
        mechanism: String,
    },
    V1 {
        /// The SASL mechanism chosen by the client.
        mechanism: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SaslHandshakeResponse {
    V0 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The mechanisms enabled in the server.
        mechanisms: Vec<String>,
    },
    V1 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The mechanisms enabled in the server.
        mechanisms: Vec<String>,
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
        /// The topics to create.
        topics: Vec<create_topics_request::v0::Topics>,
        /// How long to wait in milliseconds before timing out the request.
        timeout_ms: i32,
    },
    V1 {
        /// The topics to create.
        topics: Vec<create_topics_request::v1::Topics>,
        /// How long to wait in milliseconds before timing out the request.
        timeout_ms: i32,
        /// If true, check that the topics can be created as specified, but don't
        /// create anything.
        validate_only: bool,
    },
    V2 {
        /// The topics to create.
        topics: Vec<create_topics_request::v2::Topics>,
        /// How long to wait in milliseconds before timing out the request.
        timeout_ms: i32,
        /// If true, check that the topics can be created as specified, but don't
        /// create anything.
        validate_only: bool,
    },
    V3 {
        /// The topics to create.
        topics: Vec<create_topics_request::v3::Topics>,
        /// How long to wait in milliseconds before timing out the request.
        timeout_ms: i32,
        /// If true, check that the topics can be created as specified, but don't
        /// create anything.
        validate_only: bool,
    },
}

pub mod create_topics_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The configuration name.
            pub name: String,
            /// The number of partitions to create in the topic, or -1 if we are
            /// specifying a manual partition assignment.
            pub num_partitions: i32,
            /// The number of replicas to create for each partition in the topic, or -
            /// 1 if we are specifying a manual partition assignment.
            pub replication_factor: i16,
            /// The manual partition assignment, or the empty array if we are using
            /// automatic assignment.
            pub assignments: Vec<Assignments>,
            /// The custom topic configurations to set.
            pub configs: Vec<Configs>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The partition index.
            pub partition_index: i32,
            /// The brokers to place the partition on.
            pub broker_ids: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Configs {
            /// The configuration name.
            pub name: String,
            /// The configuration value.
            pub value: crate::types::NullableString,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The configuration name.
            pub name: String,
            /// The number of partitions to create in the topic, or -1 if we are
            /// specifying a manual partition assignment.
            pub num_partitions: i32,
            /// The number of replicas to create for each partition in the topic, or -
            /// 1 if we are specifying a manual partition assignment.
            pub replication_factor: i16,
            /// The manual partition assignment, or the empty array if we are using
            /// automatic assignment.
            pub assignments: Vec<Assignments>,
            /// The custom topic configurations to set.
            pub configs: Vec<Configs>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The partition index.
            pub partition_index: i32,
            /// The brokers to place the partition on.
            pub broker_ids: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Configs {
            /// The configuration name.
            pub name: String,
            /// The configuration value.
            pub value: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The configuration name.
            pub name: String,
            /// The number of partitions to create in the topic, or -1 if we are
            /// specifying a manual partition assignment.
            pub num_partitions: i32,
            /// The number of replicas to create for each partition in the topic, or -
            /// 1 if we are specifying a manual partition assignment.
            pub replication_factor: i16,
            /// The manual partition assignment, or the empty array if we are using
            /// automatic assignment.
            pub assignments: Vec<Assignments>,
            /// The custom topic configurations to set.
            pub configs: Vec<Configs>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The partition index.
            pub partition_index: i32,
            /// The brokers to place the partition on.
            pub broker_ids: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Configs {
            /// The configuration name.
            pub name: String,
            /// The configuration value.
            pub value: crate::types::NullableString,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The configuration name.
            pub name: String,
            /// The number of partitions to create in the topic, or -1 if we are
            /// specifying a manual partition assignment.
            pub num_partitions: i32,
            /// The number of replicas to create for each partition in the topic, or -
            /// 1 if we are specifying a manual partition assignment.
            pub replication_factor: i16,
            /// The manual partition assignment, or the empty array if we are using
            /// automatic assignment.
            pub assignments: Vec<Assignments>,
            /// The custom topic configurations to set.
            pub configs: Vec<Configs>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Assignments {
            /// The partition index.
            pub partition_index: i32,
            /// The brokers to place the partition on.
            pub broker_ids: Vec<i32>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Configs {
            /// The configuration name.
            pub name: String,
            /// The configuration value.
            pub value: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreateTopicsResponse {
    V0 {
        /// Results for each topic we tried to create.
        topics: Vec<create_topics_response::v0::Topics>,
    },
    V1 {
        /// Results for each topic we tried to create.
        topics: Vec<create_topics_response::v1::Topics>,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Results for each topic we tried to create.
        topics: Vec<create_topics_response::v2::Topics>,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// Results for each topic we tried to create.
        topics: Vec<create_topics_response::v3::Topics>,
    },
}

pub mod create_topics_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
            /// The error message, or null if there was no error.
            pub error_message: crate::types::NullableString,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
            /// The error message, or null if there was no error.
            pub error_message: crate::types::NullableString,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Topics {
            /// The topic name.
            pub name: String,
            /// The error code, or 0 if there was no error.
            pub error_code: i16,
            /// The error message, or null if there was no error.
            pub error_message: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteTopicsRequest {
    V0 {
        /// The names of the topics to delete
        topic_names: Vec<String>,
        /// The length of time in milliseconds to wait for the deletions to
        /// complete.
        timeout_ms: i32,
    },
    V1 {
        /// The names of the topics to delete
        topic_names: Vec<String>,
        /// The length of time in milliseconds to wait for the deletions to
        /// complete.
        timeout_ms: i32,
    },
    V2 {
        /// The names of the topics to delete
        topic_names: Vec<String>,
        /// The length of time in milliseconds to wait for the deletions to
        /// complete.
        timeout_ms: i32,
    },
    V3 {
        /// The names of the topics to delete
        topic_names: Vec<String>,
        /// The length of time in milliseconds to wait for the deletions to
        /// complete.
        timeout_ms: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DeleteTopicsResponse {
    V0 {
        /// The results for each topic we tried to delete.
        responses: Vec<delete_topics_response::v0::Responses>,
    },
    V1 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The results for each topic we tried to delete.
        responses: Vec<delete_topics_response::v1::Responses>,
    },
    V2 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The results for each topic we tried to delete.
        responses: Vec<delete_topics_response::v2::Responses>,
    },
    V3 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The results for each topic we tried to delete.
        responses: Vec<delete_topics_response::v3::Responses>,
    },
}

pub mod delete_topics_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// The topic name
            pub name: String,
            /// The deletion error, or 0 if the deletion succeeded.
            pub error_code: i16,
        }
    }
    pub mod v1 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// The topic name
            pub name: String,
            /// The deletion error, or 0 if the deletion succeeded.
            pub error_code: i16,
        }
    }
    pub mod v2 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// The topic name
            pub name: String,
            /// The deletion error, or 0 if the deletion succeeded.
            pub error_code: i16,
        }
    }
    pub mod v3 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// The topic name
            pub name: String,
            /// The deletion error, or 0 if the deletion succeeded.
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
        /// The transactional id, or null if the producer is not transactional.
        transactional_id: crate::types::NullableString,
        /// The time in ms to wait for before aborting idle transactions sent by
        /// this producer. This is only relevant if a TransactionalId has been
        /// defined.
        transaction_timeout_ms: i32,
    },
    V1 {
        /// The transactional id, or null if the producer is not transactional.
        transactional_id: crate::types::NullableString,
        /// The time in ms to wait for before aborting idle transactions sent by
        /// this producer. This is only relevant if a TransactionalId has been
        /// defined.
        transaction_timeout_ms: i32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InitProducerIdResponse {
    V0 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The current producer id.
        producer_id: i64,
        /// The current epoch associated with the producer id.
        producer_epoch: i16,
    },
    V1 {
        /// The duration in milliseconds for which the request was throttled due
        /// to a quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The current producer id.
        producer_id: i64,
        /// The current epoch associated with the producer id.
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
    V3 {
        /// Broker id of the follower. For normal consumers, use -1.
        replica_id: i32,
        /// An array of topics to get epochs for
        topics: Vec<offset_for_leader_epoch_request::v3::Topics>,
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
    pub mod v3 {
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
    V3 {
        /// Duration in milliseconds for which the request was throttled due to
        /// quota violation (Zero if the request did not violate any quota)
        throttle_time_ms: i32,
        /// An array of topics for which we have leader offsets for some requested
        /// partition leader epoch
        topics: Vec<offset_for_leader_epoch_response::v3::Topics>,
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
    pub mod v3 {
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
            pub resource_pattern_type: i8,
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
            pub resource_pattern_type: i8,
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
            pub resource_pattern_type: i8,
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
        /// The SASL authentication bytes from the client, as defined by the SASL
        /// mechanism.
        auth_bytes: crate::types::Bytes,
    },
    V1 {
        /// The SASL authentication bytes from the client, as defined by the SASL
        /// mechanism.
        auth_bytes: crate::types::Bytes,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SaslAuthenticateResponse {
    V0 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The error message, or null if there was no error.
        error_message: crate::types::NullableString,
        /// The SASL authentication bytes from the server, as defined by the SASL
        /// mechanism.
        auth_bytes: crate::types::Bytes,
    },
    V1 {
        /// The error code, or 0 if there was no error.
        error_code: i16,
        /// The error message, or null if there was no error.
        error_message: crate::types::NullableString,
        /// The SASL authentication bytes from the server, as defined by the SASL
        /// mechanism.
        auth_bytes: crate::types::Bytes,
        /// The SASL authentication bytes from the server, as defined by the SASL
        /// mechanism.
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
        /// The election results, or an empty array if the requester did not have
        /// permission and the request asks for all partitions.
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IncrementalAlterConfigsRequest {
    V0 {
        /// The incremental updates for each resource.
        resources: Vec<incremental_alter_configs_request::v0::Resources>,
        /// True if we should validate the request, but not change the
        /// configurations.
        validate_only: bool,
    },
}

pub mod incremental_alter_configs_request {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Resources {
            /// The resource type.
            pub resource_type: i8,
            /// The resource name.
            pub resource_name: String,
            /// The configurations.
            pub configs: Vec<Configs>,
        }
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Configs {
            /// The configuration key name.
            pub name: String,
            /// The type (Set, Delete, Append, Subtract) of operation.
            pub config_operation: i8,
            /// The value to set for the configuration key.
            pub value: crate::types::NullableString,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IncrementalAlterConfigsResponse {
    V0 {
        /// Duration in milliseconds for which the request was throttled due to a
        /// quota violation, or zero if the request did not violate any quota.
        throttle_time_ms: i32,
        /// The responses for each resource.
        responses: Vec<incremental_alter_configs_response::v0::Responses>,
    },
}

pub mod incremental_alter_configs_response {
    pub mod v0 {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Responses {
            /// The resource error code.
            pub error_code: i16,
            /// The resource error message, or null if there was no error.
            pub error_message: crate::types::NullableString,
            /// The resource type.
            pub resource_type: i8,
            /// The resource name.
            pub resource_name: String,
        }
    }
}
