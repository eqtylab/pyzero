use std::{fs::File, process::ExitCode};

use anyhow::Result;
use clap::Parser;
use pyzero_core::{image_id_from_u32_array, proof::Proof, verify::verify_proof};
use pyzero_verify::cli;

fn main() -> Result<ExitCode> {
    let cli::Args { proof_file } = cli::Args::parse();

    let proof: Proof = bincode::deserialize_from(File::open(proof_file)?)?;

    println!(
        "Verifying proof (guest image ID: {})...",
        image_id_from_u32_array(proof.image_id)
    );

    if !verify_proof(&proof)? {
        println!("Proof failed to verify.");

        return Ok(ExitCode::from(1));
    }

    println!("Proof successfully verified.");

    Ok(ExitCode::from(0))
}
