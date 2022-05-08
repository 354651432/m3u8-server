#[tokio::main]
async fn main() {
    match work::server::run().await {
        _ => (),
    };
}
