use std::env::args;

fn main() {
    let txHash = args().nth(1).expect("No transaction hash given!");

    println!("{:?}", txHash);
}
