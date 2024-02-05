#![deny(clippy::all)]

use futures::Future;
use tokio::time::{sleep, Duration};

fn call_api_one() -> impl Future<Output = String> {
    let result = "teste".to_string();
    async move {
        sleep(Duration::from_secs(1)).await;
        format!("{}: {}", result, "api_one")
    }
}
async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "api_two".to_string()
}
#[tokio::main]
async fn main() {
    let one = call_api_one().await;
    println!("{one}");
    let two = call_api_two().await;
    println!("{two}");
}
