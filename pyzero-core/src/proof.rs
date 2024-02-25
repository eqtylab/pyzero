use anyhow::Result;
use risc0_zkvm::{Journal, Receipt};
use serde::{Deserialize, Serialize};

use crate::{guest_interface, result::PythonCodeResult};

#[derive(Clone, Deserialize, Serialize)]
pub struct Proof {
    pub image_id: [u32; 8],
    pub receipt: Receipt,
}

pub fn journal_from_proof(proof: &Proof) -> Result<PythonCodeResult> {
    let journal = journal_from_raw_journal(&proof.receipt.journal)?;

    Ok(journal)
}

pub fn journal_from_raw_journal(journal: &Journal) -> Result<PythonCodeResult> {
    let journal = journal
        .decode::<guest_interface::PythonCodeResult>()?
        .into();

    Ok(journal)
}
