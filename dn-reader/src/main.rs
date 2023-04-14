use dn_lib::proof;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    filename: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.filename).expect("msg");
    let mut proof = proof::Proof::read_proof(&content).expect("");
    proof.check();

    println!("The proof result: {:?}",proof.state());
}
