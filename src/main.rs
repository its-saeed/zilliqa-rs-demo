use std::error::Error;

use zilliqa_rs::middlewares::Middleware;
use zilliqa_rs::providers::{Http, Provider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?.with_chain_id(222);
    let balance = provider
        .get_balance("0x381f4008505e940ad7681ec3468a719060caf796")
        .await;

    println!("{balance:?}");
    Ok(())
}
