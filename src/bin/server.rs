use ferrischat_server::entrypoint;

#[tokio::main]
async fn main() {
    entrypoint().await;
}
