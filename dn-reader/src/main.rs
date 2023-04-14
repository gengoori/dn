use std::process::ExitCode;

use clap::Parser;
use dnlib::proof::CheckUpResult;
use dnlib::proof;

#[derive(Parser)]
struct Cli {
    filename: std::path::PathBuf,
}

//#[derive(Debug)]
enum Answer {
    ErrorReadingTheFile(std::io::Error),
    ErrorReadingTheProof(proof::ReadError),
    ErrorsInTheProof{
        first_error: usize,
        errors: Vec<(usize, proof::SemanticError)>,
    },
    InternalError(usize),
    AllRight,
}

impl std::process::Termination for Answer {
    fn report(self) -> std::process::ExitCode {
        match self {
            Answer::ErrorReadingTheFile(e) => {
                println!("Error reading the file: {}", e.to_string());
                ExitCode::FAILURE
            },
            Answer::ErrorReadingTheProof(e) => {
                println!("Error reading the proof:");
                ExitCode::FAILURE
            },
            Answer::ErrorsInTheProof { first_error, errors } => {
                println!("The first wrong record is: {}", first_error);
                println!("The following errors occured:");
                for error in errors {
                    println!("  At record {}: {}", error.0, error.1)
                }
                ExitCode::FAILURE
            },
            Answer::InternalError(no) => {
                println!("An internal error occured (#{}).", no);
                ExitCode::FAILURE
            }
            Answer::AllRight => {
                println!("Proof is valid");
                ExitCode::SUCCESS
            },
        }
    }
}

fn main() -> Answer {
    let args = Cli::parse();
    let content = match std::fs::read_to_string(&args.filename) {
        Ok(o) => o,
        Err(e) => return Answer::ErrorReadingTheFile(e),
    };
    let mut proof = match proof::Proof::read_proof(&content) {
        Ok(o) => o,
        Err(e) => return Answer::ErrorReadingTheProof(e),
    };
    proof.check();
    match proof.into_state() {
        CheckUpResult::NotChecked => Answer::InternalError(0),
        CheckUpResult::ValidUntil(_) => Answer::InternalError(1),
        CheckUpResult::SemanticErrors { first_error, errors } => Answer::ErrorsInTheProof { first_error , errors },
        CheckUpResult::Valid => Answer::AllRight,
    }
}
