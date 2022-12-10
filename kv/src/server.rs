use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use pb::*;

mod pb;

#[derive(Debug)]
struct ServerState {
    store: sled::Db,
}

impl ServerState {
    fn new() -> Self {
        Self {
            store: sled::open("persist.db").unwrap(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr)
        .await
        .expect("bind listening port error: ");

    info!("listening at {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await.expect("accept error");
        info!("new connection established: {:?}", addr);
        let state = state.clone();

        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);
            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("got a command: {:?}", msg);

                let response = match msg.command {
                    Some(request::Command::Get(RequestGet { key })) => {
                        match state.store.get(&key) {
                            Ok(v) => match v {
                                Some(v) => Response::success(key, v.to_vec()),
                                None => Response::error(404, key),
                            },
                            Err(e) => {
                                error!("get {}'s value error: {}", key, e);
                                Response::error(500, key)
                            }
                        }
                    }

                    Some(request::Command::Put(RequestPut { key, value })) => {
                        match state.store.insert(key.clone(), value.clone()) {
                            Ok(_) => Response::success(key, "success".as_bytes().to_vec()),
                            Err(e) => {
                                error!("insert key={} error: {}", key, e);
                                Response::error(500, key)
                            }
                        }
                    }
                    None => Response::error(405, Default::default()),
                };

                stream.send(response.try_into().unwrap()).await?;
            }

            Ok::<(), anyhow::Error>(())
        });
    }
}
