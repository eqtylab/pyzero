use std::path::PathBuf;

use anyhow::Result;
use pyzero_methods::{PYZERO_GUEST_ELF, PYZERO_GUEST_ID};
use risc0_zkvm::{default_prover, Executor, ExecutorEnv, ExternalProver};

use crate::{
    guest_interface,
    manifest::PythonCodeManifest,
    proof::{journal_from_raw_journal, Proof},
    result::PythonCodeResult,
};

pub fn generate_proof(manifest: PythonCodeManifest) -> Result<Proof> {
    let guest_env = ExecutorEnv::builder()
        .write(&guest_interface::PythonCodeManifest::from(manifest))?
        .build()?;

    let receipt = default_prover().prove(guest_env, PYZERO_GUEST_ELF)?;

    Ok(Proof {
        image_id: PYZERO_GUEST_ID,
        receipt,
    })
}

pub fn execute_dryrun(manifest: PythonCodeManifest) -> Result<PythonCodeResult> {
    let guest_env = ExecutorEnv::builder()
        .write(&guest_interface::PythonCodeManifest::from(manifest))?
        .env_var("RISC0_DEV_MODE", "true")
        .build()?;

    let session_info = r0vm_prover().execute(guest_env, PYZERO_GUEST_ELF)?;

    let journal = journal_from_raw_journal(&session_info.journal)?;

    Ok(journal)
}

fn r0vm_prover() -> ExternalProver {
    ExternalProver::new("ipc", get_r0vm_path())
}

fn get_r0vm_path() -> PathBuf {
    std::env::var("RISC0_SERVER_PATH")
        .unwrap_or("r0vm".to_owned())
        .into()
}
