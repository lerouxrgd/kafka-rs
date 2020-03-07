use std::collections::HashMap;

// use lazy_static::lazy_static;
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
use strum::{EnumCount, IntoEnumIterator};

use crate::config::Config;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

type SendOne<T> = oneshot::Sender<T>; // TODO: Maybe force it to be a Result<T, E>
type ReceiveOne<T> = oneshot::Receiver<T>;

// lazy_static! {
//     static ref METADATA:
// }

#[derive(Debug)]
struct Metadata {
    cluster_id: String,
    by_partition: HashMap<TopicPartition, PartitionInfo>,
}

#[derive(Debug)]
struct TopicPartition(String, i32);

#[derive(Debug)]
struct PartitionInfo {
    topic: String,
    partition: i32,
    leader: Node,
    leader_epoch: Option<i32>,
    replicas: Vec<Node>,
    in_sync_replicas: Vec<Node>,
    offline_replicas: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    id: i32,
    id_string: String,
    host: String,
    port: i32,
    rack: Option<String>,
}

enum Event {
    Refresh,
}

async fn metadata_loop(conf: &Config, mut events: Receiver<Event>) -> Result<()> {
    let (addrs, topics) = (&conf.broker_addresses, &conf.topics);
    let _ = bootstrap_metadata(addrs, topics).await?;
    todo!()
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

async fn bootstrap_metadata<T: AsRef<str>>(
    addrs: &[T],
    topics: &[T],
) -> Result<HashMap<(String, i32), String>> {
    const VERSION_MAX: u8 = METADATAREQUEST_COUNT as u8;

    'addr: for addr in addrs {
        'version: for version in 0..MetadataRequest::count() {
            let mut stream = TcpStream::connect(addr.as_ref()).await?;

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
                            name: topic.as_ref().to_string(),
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
