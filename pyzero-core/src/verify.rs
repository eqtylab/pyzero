use anyhow::Result;

use crate::proof::Proof;

pub fn verify_proof(proof: &Proof) -> Result<bool> {
    let Proof { image_id, receipt } = proof;

    let result = receipt.verify(*image_id);
    let verified = result.is_ok();

    Ok(verified)
}
