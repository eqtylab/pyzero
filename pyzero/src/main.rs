use std::fs;

use anyhow::Result;
use clap::Parser;
use pyzero::{
    cli,
    redactions::{
        redactions_from_arg_redactions_list_string, redactions_from_code_redactions_list_string,
    },
};
use pyzero_core::{
    manifest::make_python_code_manifest,
    proof::journal_from_proof,
    prove::{execute_dryrun, generate_proof},
};

fn main() -> Result<()> {
    let cli::Args {
        python_file,
        python_args,
        code_redactions,
        arg_redactions,
        journal_path,
        proof_path,
        receipt_path,
        dryrun,
    } = cli::Args::parse();

    let manifest = {
        let code = fs::read_to_string(&python_file)?;

        let args = python_args;

        let redactions = {
            let mut r = vec![];

            if let Some(code_redactions) = code_redactions {
                r.extend(redactions_from_code_redactions_list_string(
                    &code_redactions,
                )?);
            }

            if let Some(arg_redactions) = arg_redactions {
                r.extend(redactions_from_arg_redactions_list_string(&arg_redactions)?);
            }

            r
        };

        make_python_code_manifest(&code, args, redactions)?
    };

    let (journal, proof) = if dryrun {
        println!("\nRunning PyZero dryrun. No proof will be generated.\n");

        let journal = execute_dryrun(manifest)?;

        println!("\nCompleted dryrun with no errors.\n");

        (journal, None)
    } else {
        println!("\nGenerating PyZero proof.\n");

        let proof = generate_proof(manifest)?;

        let journal = journal_from_proof(&proof)?;

        println!("\nCompleted proof generation with no errors.\n");

        (journal, Some(proof))
    };

    // write journal file if user requested it
    if let Some(journal_path) = journal_path {
        fs::write(journal_path, serde_json::to_string_pretty(&journal)?)?;
    }

    // write proof file if we generated a proof
    if let Some(proof) = proof.clone() {
        fs::write(proof_path, serde_json::to_string(&proof)?)?;
    }

    // write receipt file if we generated a proof and receipt file was requested
    if let (Some(proof), Some(receipt_path)) = (proof, receipt_path) {
        let receipt = bincode::serialize(&proof.receipt)?;
        fs::write(receipt_path, receipt)?;
    }

    journal.write_summary(&mut std::io::stdout())?;

    Ok(())
}
