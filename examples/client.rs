// use anyhow::Result;
// use async_prost::AsyncProstStream;
// use futures::prelude::*;
// use kv::{CommandRequest, CommandResponse};
// use tokio::net::TcpStream;
// use tracing::info;
use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let stream = TcpStream::connect(addr).await?;
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // 生成一个hset 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "worl".into());
    let getcmd = CommandRequest::new_hget("table1", "hello");
    let exist_cmd = CommandRequest::new_hexist("table1", "hello");
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("got response {:?}", data);
    }
    client.send(getcmd).await?;
    if let Some(data) = client.next().await {
        info!("get response {:?}", data);
    }
    client.send(exist_cmd).await?;
    if let Some(data) = client.next().await {
        info!("get response {:?}", data);
    }

    Ok(())
}
