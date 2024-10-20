#[tokio::main]
async fn main() {
    let conn = repro_tracing::LocalConnection::connect().await;
    conn.work().await;
}
