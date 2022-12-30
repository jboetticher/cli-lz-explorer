use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: TransactionHashCli = TransactionHashCli::parse();

    // Parse network from argument
    let network: Network = match args.network.as_str() {
        "Mainnet" | "m" | "main" | "mainnet" => Network::Mainnet,
        "Testnet" | "t" | "test" | "testnet" => Network::Testnet,
        "Sandbox" | "s" | "sand" | "sandbox" => Network::Sandbox,
        _ => panic!("Incorrect network! Consider \"mainnet\", \"testnet\", or \"sandbox\"."),
    };

    // Communicate with the API
    let client = Client::new();
    let url = match network {
        Network::Mainnet => "https://api-mainnet.layerzero-scan.com",
        Network::Testnet => "https://api-testnet.layerzero-scan.com",
        Network::Sandbox => "https://api-sandbox.layerzero-scan.com"
    }.to_owned() + "/tx/" + args.tx_hash.as_str();
    let response = client.get(url).send().await?;

    // println!("{:?}", &response.text().await?);

    // Parse the response
    let data = response.json::<Messages>().await?;

    println!("{:?}", data);

    Ok(())
}

#[derive(Debug)]
enum Network {
    Mainnet,
    Testnet,
    Sandbox,
}

#[derive(Debug, Parser)]
#[command(version, long_about = None)]
#[command(author = "Jeremy Boetticher <jeremy.boetticher@purestake.com>")]
#[command(about = "A CLI that allows for easy reading of LayerZero transaction statuses.")]
struct TransactionHashCli {
    /// The source chain's transaction hash that sent a cross-chain transaction
    tx_hash: String,

    /// The network type on which the transaction was on
    network: String,
}

#[derive(Debug, Deserialize)]
struct Messages {
    messages: Box<[Message]>
}

#[derive(Debug, Deserialize)]
struct Message {
    srcUaAddress: String,
    dstUaAddress: String,
    srcChainId: u32,
    dstChainId: u32,
    dstTxHash: Option<String>,
    dstTxError: Option<String>,
    srcTxHash: String,
    srcBlockHash: String,
    srcBlockNumber: String,
    srcUaNonce: u128,
    status: String
}
