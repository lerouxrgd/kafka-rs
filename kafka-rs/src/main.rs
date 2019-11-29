mod acc;
mod req;

use std::{collections::HashMap, convert::TryFrom, sync::Arc};

use async_std::{net::TcpStream, prelude::*, task};
use futures::{channel::mpsc, channel::oneshot, SinkExt};
use kafka_protocol::{
    codec::{self, decode_resp, encode_req, Compression, Deserializer, Serializer},
    model::*,
    types::*,
};
use serde::ser::Serialize;
use strum::EnumCount;

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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

type SendOne<T> = oneshot::Sender<T>;
type ReceiveOne<T> = oneshot::Receiver<T>;

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

    let mut brokers = HashMap::<String, Sender<Vec<u8>>>::new();
    let mut brokers_index = HashMap::<(String, i32), String>::new();
    let mut make_correlation_id = correlation_generator();

    let mut api_versions = HashMap::<ApiKey, i16>::new(); // TODO: once_cell/arc-swap ?

    while let Some(event) = events.next().await {
        match event {
            Event::Payload(mut payload) => {
                let broker_id = brokers_index
                    .get(&(payload.topic, payload.partition))
                    .unwrap();
                let broker = brokers.get_mut(broker_id).unwrap();

                let correlation_id = make_correlation_id();
                payload.data.splice(
                    CORR_ID_POS..CORR_ID_POS + 4,
                    correlation_id.to_be_bytes().iter().cloned(),
                );

                broker.send(payload.data).await.unwrap();
            }
            Event::Init => {
                // TODO: ApiVersions request, await response
                // TODO: setup global map valid for all brokers ? (vers. intersection)
                let addrs = vec!["127.0.0.1:9092"];
                let topic = vec!["test"];

                for addr in addrs {
                    let mut stream = TcpStream::connect(addr).await?;

                    let versions = broker_api_versions(&mut stream).await?;

                    let stream = Arc::new(stream);
                    let (broker_tx, broker_rx) = mpsc::unbounded();
                    brokers.insert(addr.into(), broker_tx);
                    spawn_and_log_error(broker_send_loop(broker_rx, Arc::clone(&stream)));
                }
            }
        }
    }

    Ok(())
}

async fn broker_api_versions(stream: &mut TcpStream) -> Result<HashMap<ApiKey, (usize, usize)>> {
    for version in 0..ApiVersionsRequest::count() {
        let header = HeaderRequest {
            api_key: ApiKey::ApiVersions,
            api_version: version as i16,
            correlation_id: 0,
            client_id: NullableString(None),
        };
        let bytes = encode_req(&header, &ApiVersionsRequest::V0 {})?;

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
        stream.write_all(&request).await?;
    }
    Ok(())
}

struct Payload {
    topic: String,
    partition: i32,
    data: Vec<u8>,
    response: SendOne<Response>,
}

// TODO: wrap all possible response enums ?
enum Response {}

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
