#![recursion_limit = "256"]

mod acc;
mod config;
mod metadata;
mod req;

use std::{borrow::Cow, collections::HashMap, convert::TryFrom, sync::Arc};

use arc_swap::ArcSwap;
use async_std::{net::TcpStream, prelude::*, task};
use futures::{
    channel::{mpsc, oneshot},
    select, FutureExt, SinkExt,
};
use kafka_protocol::{
    codec::{self, decode_partial, decode_resp, encode_req, Compression, Deserializer, Serializer},
    model::*,
    types::*,
};
use lazy_static::lazy_static;
use serde::ser::Serialize;
use strum::{EnumCount, IntoEnumIterator};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

type SendOne<T> = oneshot::Sender<T>; // TODO: Maybe force it to be a Result<T, E>
type ReceiveOne<T> = oneshot::Receiver<T>;

lazy_static! {

    static ref SUPPORTED_API_VERSIONS: HashMap<ApiKey, (usize, usize)> = ApiKey::iter()
        .map(|api_key| match api_key {
            ApiKey::Produce => (api_key, (3, ProduceRequest::count()-1)),
            ApiKey::Fetch => (api_key, (4, FetchRequest::count()-1)),
            // ApiKey::ListOffsets => (api_key, (0, ListOffsetsRequest::count()-1)),
            ApiKey::Metadata => (api_key, (0, MetadataRequest::count()-1)),
            // ApiKey::LeaderAndIsr => (api_key, (0, LeaderAndIsrRequest::count()-1)),
            // ApiKey::StopReplica => (api_key, (0, StopReplicaRequest::count()-1)),
            // ApiKey::UpdateMetadata => (api_key, (0, UpdateMetadataRequest::count()-1)),
            // ApiKey::ControlledShutdown => (api_key, (0, ControlledShutdownRequest::count()-1)),
            // ApiKey::OffsetCommit => (api_key, (0, OffsetCommitRequest::count()-1)),
            // ApiKey::OffsetFetch => (api_key, (0, OffsetFetchRequest::count()-1)),
            // ApiKey::FindCoordinator => (api_key, (0, FindCoordinatorRequest::count()-1)),
            // ApiKey::JoinGroup => (api_key, (0, JoinGroupRequest::count()-1)),
            // ApiKey::Heartbeat => (api_key, (0, HeartbeatRequest::count()-1)),
            // ApiKey::LeaveGroup => (api_key, (0, LeaveGroupRequest::count()-1)),
            // ApiKey::SyncGroup => (api_key, (0, SyncGroupRequest::count()-1)),
            // ApiKey::DescribeGroups => (api_key, (0, DescribeGroupsRequest::count()-1)),
            // ApiKey::ListGroups => (api_key, (0, ListGroupsRequest::count()-1)),
            // ApiKey::SaslHandshake => (api_key, (0, SaslHandshakeRequest::count()-1)),
            ApiKey::ApiVersions => (api_key, (0, ApiVersionsRequest::count()-1)),
            ApiKey::CreateTopics => (api_key, (0, CreateTopicsRequest::count()-1)),
            // ApiKey::DeleteTopics => (api_key, (0, DeleteTopicsRequest::count()-1)),
            // ApiKey::DeleteRecords => (api_key, (0, DeleteRecordsRequest::count()-1)),
            // ApiKey::InitProducerId => (api_key, (0, InitProducerIdRequest::count()-1)),
            // ApiKey::OffsetForLeaderEpoch => (api_key, (0, OffsetForLeaderEpochRequest::count()-1)),
            // ApiKey::AddPartitionsToTxn => (api_key, (0, AddPartitionsToTxnRequest::count()-1)),
            // ApiKey::AddOffsetsToTxn => (api_key, (0, AddOffsetsToTxnRequest::count()-1)),
            // ApiKey::EndTxn => (api_key, (0, EndTxnRequest::count()-1)),
            // ApiKey::WriteTxnMarkers => (api_key, (0, WriteTxnMarkersRequest::count()-1)),
            // ApiKey::TxnOffsetCommit => (api_key, (0, TxnOffsetCommitRequest::count()-1)),
            // ApiKey::DescribeAcls => (api_key, (0, DescribeAclsRequest::count()-1)),
            // ApiKey::CreateAcls => (api_key, (0, CreateAclsRequest::count()-1)),
            // ApiKey::DeleteAcls => (api_key, (0, DeleteAclsRequest::count()-1)),
            // ApiKey::DescribeConfigs => (api_key, (0, DescribeConfigsRequest::count()-1)),
            // ApiKey::AlterConfigs => (api_key, (0, AlterConfigsRequest::count()-1)),
            // ApiKey::AlterReplicaLogDirs => (api_key, (0, AlterReplicaLogDirsRequest::count()-1)),
            // ApiKey::DescribeLogDirs => (api_key, (0, DescribeLogDirsRequest::count()-1)),
            // ApiKey::SaslAuthenticate => (api_key, (0, SaslAuthenticateRequest::count()-1)),
            // ApiKey::CreatePartitions => (api_key, (0, CreatePartitionsRequest::count()-1)),
            // ApiKey::CreateDelegationToken => (api_key, (0, CreateDelegationTokenRequest::count()-1)),
            // ApiKey::RenewDelegationToken => (api_key, (0, RenewDelegationTokenRequest::count()-1)),
            // ApiKey::ExpireDelegationToken => (api_key, (0, ExpireDelegationTokenRequest::count()-1)),
            // ApiKey::DescribeDelegationToken => (api_key, (0, DescribeDelegationTokenRequest::count()-1)),
            // ApiKey::DeleteGroups => (api_key, (0, DeleteGroupsRequest::count()-1)),
            // ApiKey::ElectPreferredLeaders => (api_key, (0, ElectPreferredLeadersRequest::count()-1)),
            // ApiKey::IncrementalAlterConfigs => (api_key, (0, IncrementalAlterConfigsRequest::count()-1)),
            _ => (api_key, (0, 0)), // TODO: remove when using full model
        })
        .collect::<HashMap<_, _>>();

    static ref API_VERSIONS: ArcSwap<HashMap<ApiKey, usize>> = {
        let api_versions = SUPPORTED_API_VERSIONS
            .iter()
            .map(|(api_key, (min, _))| (*api_key, *min))
            .collect::<HashMap<_, _>>();
        ArcSwap::from(Arc::new(api_versions))
    };
}

pub async fn read_resp<T>(
    stream: &mut TcpStream,
    version: usize,
) -> codec::Result<(HeaderResponse, T)>
where
    T: serde::de::DeserializeOwned,
{
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await?;
    let size = i32::from_be_bytes(buf);
    let mut bytes = vec![0; size as usize];
    stream.read_exact(&mut bytes).await?;
    decode_resp::<T>(&bytes, version)
}

// TODO: this is a draft (to identify required functionalities)
async fn req_metadata(stream: &mut TcpStream, topics: Vec<String>) -> Result<MetadataResponse> {
    let header = HeaderRequest {
        api_key: ApiKey::Metadata,
        api_version: 0,
        correlation_id: 42,
        client_id: NullableString::from("me"), // TODO: once_cell after config ?
    };

    let req = MetadataRequest::V0 {
        topics: topics
            .into_iter()
            .map(|topic| metadata_request::v0::Topics { name: topic })
            .collect::<Vec<_>>(),
    };

    let bytes = encode_req(&header, &req)?;
    stream.write_all(&bytes).await?; // TODO: thru some dispatcher with oneshot chan ?

    let (header, resp) = read_resp::<MetadataResponse>(stream, 0).await?;
    Ok(resp)
}

fn correlation_generator() -> impl FnMut() -> i32 {
    let mut correlation = 1 as i32;

    move || {
        correlation = (correlation + 1) % (1i32 << 30);
        correlation
    }
}

async fn dispatcher_loop(mut events: Receiver<Event>) -> Result<()> {
    const CORR_ID_POS: usize = (32 + 16 + 16) / 8;

    #[derive(PartialOrd, Ord, PartialEq, Eq, Hash)]
    struct IndexKey<'a> {
        topic: Cow<'a, str>,
        partition: i32,
    }

    let mut brokers = HashMap::<String, (Sender<Vec<u8>>, Sender<(i32, SendOne<Vec<u8>>)>)>::new();
    let mut brokers_index = HashMap::<IndexKey, String>::new();
    let mut make_correlation_id = correlation_generator();

    while let Some(event) = events.next().await {
        match event {
            Event::Payload(mut payload) => {
                let broker_id = brokers_index
                    .get(&IndexKey {
                        topic: Cow::Borrowed(payload.topic.as_str()),
                        partition: payload.partition,
                    })
                    .ok_or_else(|| {
                        format!(
                            "no broker for topic: {} partition: {}",
                            payload.topic, payload.partition
                        )
                    })?;

                let (send_tx, recv_tx) = brokers.get_mut(broker_id).unwrap();

                let corr_id = make_correlation_id();
                payload.data.splice(
                    CORR_ID_POS..CORR_ID_POS + 4,
                    corr_id.to_be_bytes().iter().cloned(),
                );

                recv_tx.send((corr_id, payload.responder)).await.unwrap();
                send_tx.send(payload.data).await.unwrap();
            }
            Event::Init => {
                // TODO: get seed brokers and topic from conf, somehow
                let addrs = vec!["127.0.0.1:9092"];
                let topics = vec!["test"];

                brokers_index = bootstrap_metadata(&addrs, &topics)
                    .await?
                    .into_iter()
                    .map(|((topic, partition), broker)| {
                        (
                            IndexKey {
                                topic: Cow::Owned(topic),
                                partition,
                            },
                            broker,
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let mut supported_versions = SUPPORTED_API_VERSIONS.clone();

                // TODO: fix this, it can create multiple connections to the same broker
                for (_, broker) in &brokers_index {
                    let mut stream = TcpStream::connect(broker).await?;

                    let versions = broker_api_versions(&mut stream).await?;
                    for (api_key, version) in versions {
                        let supported = supported_versions.get_mut(&api_key).unwrap();
                        match intersection(supported, &version) {
                            Some(overlap) => {
                                *supported = overlap;
                            }
                            None => {
                                return Err(format!(
                                "unsupported api {:?} version {:?} for broker {}, supported: {:?}",
                                api_key, version, broker, supported
                            )
                                .into())
                            }
                        }
                    }

                    let stream = Arc::new(stream);
                    let (send_tx, send_rx) = mpsc::unbounded();
                    let (recv_tx, recv_rx) = mpsc::unbounded();
                    brokers.insert(broker.clone(), (send_tx, recv_tx));
                    spawn_and_log_error(broker_send_loop(send_rx, Arc::clone(&stream)));
                    spawn_and_log_error(broker_receive_loop(recv_rx, Arc::clone(&stream)));
                }

                API_VERSIONS.store(Arc::new(
                    supported_versions
                        .into_iter()
                        .map(|(api_key, (_, max))| (api_key, max))
                        .collect::<HashMap<_, _>>(),
                ));
            }
        }
    }

    Ok(())
}

fn intersection(a: &(usize, usize), b: &(usize, usize)) -> Option<(usize, usize)> {
    assert!(a.0 <= a.1);
    assert!(b.0 <= b.1);
    if b.0 >= a.0 && b.0 <= a.1 {
        if b.1 >= a.1 {
            Some((b.0, a.1))
        } else {
            Some((b.0, b.1))
        }
    } else if a.0 >= b.0 && a.0 <= b.1 {
        if a.1 >= b.1 {
            Some((a.0, b.1))
        } else {
            Some((a.0, a.1))
        }
    } else {
        None
    }
}

async fn bootstrap_metadata(
    addrs: &[&str],
    topics: &[&str],
) -> Result<HashMap<(String, i32), String>> {
    const VERSION_MAX: u8 = METADATAREQUEST_COUNT as u8;

    'addr: for addr in addrs {
        'version: for version in 0..MetadataRequest::count() {
            let mut stream = TcpStream::connect(addr).await?;

            let header = HeaderRequest {
                api_key: ApiKey::Metadata,
                api_version: version as i16,
                correlation_id: 0,
                client_id: NullableString(None),
            };

            let req = match version as u8 {
                0 => MetadataRequest::V0 {
                    topics: topics
                        .into_iter()
                        .map(|topic| metadata_request::v0::Topics {
                            name: topic.to_string(),
                        })
                        .collect::<Vec<_>>(),
                },
                VERSION_MAX..=255 => unreachable!(),
            };

            let bytes = encode_req(&header, &req)?;
            match stream.write_all(&bytes).await {
                Err(_) => break 'addr,
                Ok(()) => (),
            }
            match read_resp::<MetadataResponse>(&mut stream, version).await {
                Err(_) => break 'version,
                Ok((_, resp)) => match resp {
                    MetadataResponse::V0 { topics, brokers } => {
                        let brokers = brokers
                            .into_iter()
                            .map(|b| (b.node_id, format!("{}:{}", b.host, b.port)))
                            .collect::<HashMap<_, _>>();

                        let mut brokers_index = HashMap::new();
                        for t in topics {
                            for p in t.partitions {
                                brokers_index.insert(
                                    (t.name.clone(), p.partition_index),
                                    brokers.get(&p.partition_index).unwrap().clone(),
                                );
                            }
                        }

                        return Ok(brokers_index);
                    }
                },
            }
        }
    }

    Err("Couldn't find any broker handling metadata requests".into())
}

async fn broker_api_versions(stream: &mut TcpStream) -> Result<HashMap<ApiKey, (usize, usize)>> {
    for version in 0..ApiVersionsRequest::count() {
        let header = HeaderRequest {
            api_key: ApiKey::ApiVersions,
            api_version: version as i16,
            correlation_id: 0,
            client_id: NullableString(None),
        };
        let bytes = encode_req(&header, &ApiVersionsRequest::V0 {})?; // TODO: use appropriate version somehow

        stream.write_all(&bytes).await?;
        let (_, resp) = read_resp::<ApiVersionsResponse>(stream, version).await?;

        return match resp {
            ApiVersionsResponse::V0 { error_code, .. }
            | ApiVersionsResponse::V1 { error_code, .. }
            | ApiVersionsResponse::V2 { error_code, .. }
                if error_code != ErrorCode::None as i16 =>
            {
                match ErrorCode::try_from(error_code) {
                    Ok(ErrorCode::UnsupportedVersion) => continue,
                    Ok(error) => Err(format!("unexpected error: {:?}", error).into()),
                    Err(_) => Err(format!("unknown error code: {}", error_code).into()),
                }
            }

            ApiVersionsResponse::V0 { api_versions, .. } => Ok(api_versions
                .iter()
                .filter_map(|v| match ApiKey::try_from(v.api_key) {
                    Ok(api_key) => Some((api_key, (v.min_version, v.max_version))),
                    _ => None,
                })
                .map(|(api_key, (min, max))| {
                    Ok((api_key, (usize::try_from(min)?, usize::try_from(max)?)))
                })
                .collect::<Result<HashMap<_, _>>>()?),

            ApiVersionsResponse::V1 { api_versions, .. } => Ok(api_versions
                .iter()
                .filter_map(|v| match ApiKey::try_from(v.api_key) {
                    Ok(api_key) => Some((api_key, (v.min_version, v.max_version))),
                    _ => None,
                })
                .map(|(api_key, (min, max))| {
                    Ok((api_key, (usize::try_from(min)?, usize::try_from(max)?)))
                })
                .collect::<Result<HashMap<_, _>>>()?),

            ApiVersionsResponse::V2 { api_versions, .. } => Ok(api_versions
                .iter()
                .filter_map(|v| match ApiKey::try_from(v.api_key) {
                    Ok(api_key) => Some((api_key, (v.min_version, v.max_version))),
                    _ => None,
                })
                .map(|(api_key, (min, max))| {
                    Ok((api_key, (usize::try_from(min)?, usize::try_from(max)?)))
                })
                .collect::<Result<HashMap<_, _>>>()?),
        };
    }

    Err("couldn't find a suitable version for ApiVersions request".into())
}

async fn broker_send_loop(mut requests: Receiver<Vec<u8>>, stream: Arc<TcpStream>) -> Result<()> {
    let mut stream = &*stream;
    while let Some(request) = requests.next().await {
        stream.write_all(&request).await?; // TODO: handle reconnection here ...
    }
    Ok(())
}

async fn broker_receive_loop(
    responders: Receiver<(i32, SendOne<Vec<u8>>)>,
    stream: Arc<TcpStream>,
) -> Result<()> {
    let mut stream = &*stream;
    let mut responders = responders.fuse();
    let mut pending = HashMap::<i32, SendOne<Vec<u8>>>::with_capacity(2048);
    let mut buf = [0u8; 4];

    loop {
        select! {
            responder = responders.next().fuse() => match responder {
                Some((corr_id, sender)) => {
                    pending.insert(corr_id, sender);
                }
                None => break,
            },
            read = stream.read_exact(&mut buf).fuse() => {
                if let Err(err) = read {
                    return Err(err.into());
                }

                let size = i32::from_be_bytes(buf);
                let mut bytes = vec![0; usize::try_from(size)?];
                stream.read_exact(&mut bytes).await?;
                let (header, remaining) = decode_partial(&bytes)?;
                let bytes = bytes.split_off(bytes.len() - remaining);

                match pending.remove(&header.correlation) {
                    Some(sender) => match sender.send(bytes) {
                        Ok(()) => continue,
                        Err(data) => return Err("couldn't send bytes".into())
                    },
                    None => return Err("no pending request for received response".into())
                }
            }
        }
    }

    Ok(())
}

struct Payload {
    topic: String,
    partition: i32,
    data: Vec<u8>,
    responder: SendOne<Vec<u8>>,
}

// TODO: add some Metadata variant
// TODO: also reset all broker_loop when new Metadata are received
enum Event {
    Init,
    Payload(Payload),
}

// TODO: better error handling
fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

async fn main_async() -> Result<()> {
    let addrs = vec!["127.0.0.1:9092"];
    let topic = vec!["test"];

    let mut stream = TcpStream::connect(addrs[0]).await?;

    println!("{:?}", broker_api_versions(&mut stream).await?);

    Ok(())
}

fn main() -> Result<()> {
    task::block_on(main_async())
}
