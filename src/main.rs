use clap::Parser;

fn main() {
    let args: TransactionHashCli = TransactionHashCli::parse();

    // Parse network
    let network: Network = match args.network.as_str() {
        "Mainnet" | "m" | "main" | "mainnet" => Network::Mainnet,
        "Testnet" | "t" | "test" | "testnet" => Network::Testnet,
        "Sandbox" | "s" | "sand" | "sandbox" => Network::Sandbox,
        _ => panic!("Incorrect network! Consider \"mainnet\", \"testnet\", or \"sandbox\"."),
    };

    println!("{:?}, {:?}", args.tx_hash, network);
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
