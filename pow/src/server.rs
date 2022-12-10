mod pb;
mod pow;

use std::{collections::HashMap, pin::Pin, sync::Arc, thread};

use anyhow::Result;
use futures::Stream;
use pb::{pow_builder_server::*, *};
use pow::*;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

const CHANNEL_SIZE: usize = 8;

#[derive(Debug)]
struct Shared {
    clients: HashMap<String, mpsc::Sender<Result<BlockHash, Status>>>,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            clients: Default::default(),
        }
    }
}

impl Shared {
    async fn broadcast(&self, block_hash: Option<BlockHash>) {
        let msg = block_hash.ok_or(Status::resource_exhausted("Failed to find suitable hash"));
        for (name, tx) in &self.clients {
            match tx.send(msg.clone()).await {
                Ok(_) => {
                    println!("broadcast {:?} success", name);
                }
                Err(e) => {
                    println!("broadcast {:?} with error: {:?}", name, e);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PowService {
    // send block to pow engine
    tx: mpsc::Sender<Block>,
    shared: Arc<RwLock<Shared>>,
}

impl PowService {
    pub fn new(tx: mpsc::Sender<Block>, mut rx: mpsc::Receiver<Option<BlockHash>>) -> Self {
        let server = Self {
            tx,
            shared: Arc::new(RwLock::new(Shared::default())),
        };

        let shared = server.shared.clone();

        tokio::spawn(async move {
            while let Some(block_hash) = rx.recv().await {
                let shared = shared.read().await;
                shared.broadcast(block_hash).await;
            }
        });

        server
    }
}

#[tonic::async_trait]
impl PowBuilder for PowService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockHash, Status>> + Send + Sync>>;

    async fn subscribe(
        &self,
        request: Request<ClientInfo>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let name = request.into_inner().name;

        let rx = {
            let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
            self.shared.write().await.clients.insert(name, tx);
            rx
        };

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn submit(&self, request: Request<Block>) -> Result<Response<BlockStatus>, Status> {
        let block = request.into_inner();

        match self.tx.send(block.clone()).await {
            Ok(_) => Ok(Response::new(BlockStatus { code: 0 })),
            Err(e) => {
                println!("failed to submit {:?} to pow engine. err: {:?}", block, e);
                Ok(Response::new(BlockStatus { code: 500 }))
            }
        }
    }
}

async fn start_server(addr: &str) -> Result<()> {
    // grpc --> pow
    let (tx1, mut rx1) = mpsc::channel(CHANNEL_SIZE);
    // pow --> grpc
    let (tx2, rx2) = mpsc::channel(CHANNEL_SIZE);

    thread::spawn(move || {
        while let Some(block) = rx1.blocking_recv() {
            let result = pow(block);
            tx2.blocking_send(result).unwrap();
        }
    });

    let svc = PowService::new(tx1, rx2);

    Server::builder()
        .add_service(PowBuilderServer::new(svc))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    start_server("0.0.0.0:8088").await.unwrap()
}
