#[tokio::main]
async fn main() {
    println!("My IP: {}", local_ip_address::local_ip().unwrap());
}
