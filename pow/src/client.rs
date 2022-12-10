mod pb;
use pb::{pow_builder_client::*, *};

#[tokio::main]
async fn main() {
    let addr = "http://localhost:8088";
    let mut client = PowBuilderClient::connect(addr).await.unwrap();

    let mut stream = client
        .subscribe(ClientInfo {
            name: "client1".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    let res = client
        .submit(Block {
            data: b"hello world".to_vec(),
            ..Default::default()
        })
        .await
        .unwrap()
        .into_inner();

    println!("block status: {:?}", res);

    while let Some(block_hash) = stream.message().await.unwrap() {
        println!(
            "receive block_hash: id={} hash={} nonce={}",
            hex::encode(block_hash.id),
            hex::encode(block_hash.hash),
            block_hash.nonce
        );
    }
}
