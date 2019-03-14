use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::*;

// All this will be generated with `protocol.pest` and `templates.rs` from `spec-parser` module

///  Numeric codes used to specify request types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
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

#[derive(Debug, Serialize)]
pub struct HeaderRequest {
    pub api_key: ApiKey,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: NullableString,
}

#[derive(Debug, Deserialize)]
pub struct HeaderResponse {
    pub correlation: i32,
}

#[derive(Debug, Deserialize)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub api_versions: Vec<ApiVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ApiVersion {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
}
