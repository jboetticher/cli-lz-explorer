use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, Attr, color};

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

    // Parse the response
    let data = response.json::<Messages>().await?;

    // Initialize table
    let mut table = Table::new();
    if args.verbose {
        println!("Verbose table placeholder!");
        table.add_row(row![ 
            b->"Status", 
            b->"SrcChainId", b->"SrcUaNonce", b->"SrcUaAddress",  b->"SrcBlockHash", b->"SrcBlockNumber", 
            b->"DstChainId", b->"DstUaAddress", b->"DstTx" 
        ]);
    }
    else {
        table.add_row(row![ b->"Status", b->"DstChainId", b->"SrcUaNonce" ]);
    }

    // Format & print
    data.messages.iter().for_each(|m| {
        let status_color = match m.status.as_str() {
            "INFLIGHT" => color::YELLOW,
            "SUCCESS" => color::GREEN,
            "STORED" => color::RED,
            _ => color::WHITE
        };

        if args.verbose {
            let dst_tx = if m.dstTxHash.is_some() {
                m.dstTxHash.clone().unwrap()
            } else if m.dstTxError.is_some() {
                String::from("Error!")
            }
            else {
                String::from("")
            };

            table.add_row(Row::new(vec![
                Cell::new(m.status.as_str()).with_style(Attr::ForegroundColor(status_color)),

                Cell::new(m.srcChainId.to_string().as_str()),
                Cell::new(m.srcUaNonce.to_string().as_str()),
                Cell::new(m.srcUaAddress.as_str()),
                Cell::new(m.srcBlockHash.as_str()),
                Cell::new(m.srcBlockNumber.as_str()),

                Cell::new(m.dstChainId.to_string().as_str()),
                Cell::new(m.dstUaAddress.as_str()),
                Cell::new(dst_tx.as_str())
            ]));        
        }
        else {
            table.add_row(Row::new(vec![
                Cell::new(m.status.as_str()).with_style(Attr::ForegroundColor(status_color)),
                Cell::new(m.dstChainId.to_string().as_str()),
                Cell::new(m.srcUaNonce.to_string().as_str())
            ]));
        }
    });
    table.printstd();

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

    /// Whether or not to show all of the information for each cross-chain message
    #[arg(short, long, default_value_t = false)]
    verbose: bool
}

#[derive(Debug, Deserialize)]
struct Messages {
    messages: Box<[Message]>
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
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
