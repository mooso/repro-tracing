use std::net::SocketAddr;

use local_ip_address::local_ip;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const BUF_SIZE: usize = 1024;

pub struct LocalConnection {
    addr: SocketAddr,
}

impl LocalConnection {
    pub async fn connect() -> Self {
        let listener = TcpListener::bind((local_ip().unwrap(), 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();

        let _listener_task = tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buf = [0u8; BUF_SIZE];
                loop {
                    let n = stream.read(&mut buf).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    stream.write(&buf).await.unwrap();
                }
            }
        });

        Self { addr }
    }

    pub async fn work(&self) {
        let mut stream = tokio::net::TcpStream::connect(self.addr).await.unwrap();
        let mut buf = [0u8; BUF_SIZE];
        stream.write(&buf).await.unwrap();
        let _n = stream.read(&mut buf).await.unwrap();
    }
}
