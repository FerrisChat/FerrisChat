use ferrischat_server::entrypoint;

#[tokio::main]
fn main() {
    entrypoint().await;
}
