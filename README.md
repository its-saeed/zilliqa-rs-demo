# Create a new rust app
The very first step is to create a binary rust project.

```bash
cargo new zilliqa-rs-demo
```

# Add zilliqa-rs and tokio to dependencies
```bash
cargo add zilliqa-rs tokio
```

# Call a simple JSON-RPC API
## Run the isolated-server using docker
```bash
docker run -d -p 5555:5555 --name iso-server zilliqa-isolated-server:latest
```

## Call GetBalance
First, we need to create a provider. In the first line of the main, we create an HTTP provider. We use the URL of the isolated server we ran in the previous step. The chain ID of this network is 222. 
Then we can call `get_balance` function of the provider, passing the address of the account we want its balance.

```rust
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
```