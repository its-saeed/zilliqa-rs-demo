use std::error::Error;

use zilliqa_rs::{
    contract::HelloWorld,
    core::CreateTransactionResponse,
    middlewares::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    transaction::TransactionBuilder,
    util::parse_zil,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create the signer.
    let wallet = "0xe53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89"
        .parse::<LocalWallet>()?;

    // Create the provider with a signer.
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    // Call a JSON-RPC endpoint.
    let balance = provider
        .get_balance("0x381f4008505e940ad7681ec3468a719060caf796")
        .await;

    println!("{balance:?}");

    // Send a transaction
    let receiver = LocalWallet::create_random()?;
    let tx = TransactionBuilder::default()
        .to_address(receiver.address.clone())
        .amount(parse_zil("2.0")?)
        .gas_price(2000000000u128)
        .gas_limit(50u64)
        .build();

    provider
        .send_transaction_without_confirm::<CreateTransactionResponse>(tx)
        .await?;

    let balance = provider.get_balance(&receiver.address).await;
    println!("{balance:?}");

    // Deploy a contract
    let contract = HelloWorld::deploy(provider.into(), wallet.address).await?;
    println!("Contract address: {:?}", contract.address());

    println!("Contract owner: {:?}", contract.owner().await?);
    println!("Welcome msg: {}", contract.welcome_msg().await?);

    contract.set_hello("Salaam".to_string()).call().await?;
    println!("Welcome msg: {}", contract.welcome_msg().await?);
    Ok(())
}
