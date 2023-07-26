use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::Service;
use kv::{CommandRequest, CommandResponse, MemTable};
use tokio::net::TcpListener;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected ", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("got a new command :{:?}", msg);
                let res = svc.execute(msg);
                stream.send(res).await.unwrap();
                // let mut resp = CommandResponse::default();
                // resp.status = 404;
                // resp.message = "Not Found".to_string();
                // stream.send(resp).await.unwrap();
            }
            info!("client:{:?} disconnected", addr);
        });
    }
}
