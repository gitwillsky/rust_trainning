use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod pb;

use pb::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("send");

    let addr = "127.0.0.1:8888";

    let stream = TcpStream::connect(addr).await.unwrap();

    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);

    let msg = Request::new_put("key1".into(), "value1".into());

    stream.send(msg.into()).await.unwrap();

    let msg = Request::new_get("key1".into());
    stream.send(msg.into()).await.unwrap();

    while let Some(Ok(buf)) = stream.next().await {
        let recv_msg = Response::try_from(buf).unwrap();
        let msg: String = String::from_utf8(recv_msg.value).unwrap();
        info!("got server message: {:?}", msg);
    }
}
