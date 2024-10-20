use std::{fs::File, net::SocketAddr, sync::Arc};

use local_ip_address::local_ip;
use tempfile::{tempdir, TempDir};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tracing::Subscriber;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer};

pub struct Outputs {
    output_dir: TempDir,
    counter: usize,
    pub outputs: Vec<Arc<File>>,
}

impl Outputs {
    fn new(capacity: usize) -> Self {
        Outputs {
            output_dir: tempdir().unwrap(),
            counter: 0,
            outputs: Vec::with_capacity(capacity),
        }
    }

    fn new_layer<S>(&mut self) -> impl Layer<S>
    where
        S: Sized + Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        let output = Arc::new(
            File::create_new(self.output_dir.path().join(format!("{}", self.counter))).unwrap(),
        );
        self.counter += 1;
        self.outputs.push(output.clone());
        let layer = fmt::layer().with_writer(output);
        layer
    }
}

pub fn setup_tracing_one_subscriber() -> Outputs {
    let mut outputs = Outputs::new(1);
    registry().with(outputs.new_layer()).set_default();
    outputs
}

pub fn setup_tracing_two_subscribers() -> Outputs {
    let mut outputs = Outputs::new(1);
    registry()
        .with(outputs.new_layer())
        .with(outputs.new_layer())
        .set_default();
    outputs
}

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
