mod download;
mod server;

use server::*;

#[tokio::main]
async fn main() {
	match run().await {
		_ => (),
	};
}
