use std::{fs::File, process::ExitCode};

use anyhow::Result;
use clap::Parser;
use pyzero_core::verify::verify_proof;
use pyzero_verify::cli;

fn main() -> Result<ExitCode> {
    let cli::Args { proof_file } = cli::Args::parse();

    let proof = serde_json::from_reader(File::open(proof_file)?)?;

    if !verify_proof(&proof)? {
        println!("Proof failed to verify.");

        return Ok(ExitCode::from(1));
    }

    println!("Proof successfully verified.");

    Ok(ExitCode::from(0))
}