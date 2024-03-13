use anyhow::{bail, Result};

use crate::{image_id_from_u32_array, proof::Proof};

pub fn verify_proof(proof: &Proof) -> Result<bool> {
    let Proof { image_id, receipt } = proof;

    let image_id = *image_id;

    // TODO: potentially add support for image IDs for old pyzero versions
    let pyzero_guest_id = pyzero_methods::PYZERO_GUEST_ID;
    if image_id != pyzero_guest_id {
        bail!(
            "Unsupported guest image ID. Expected: {}, got: {}",
            image_id_from_u32_array(pyzero_guest_id),
            image_id_from_u32_array(image_id)
        );
    }

    let result = receipt.verify(image_id);
    let verified = result.is_ok();

    Ok(verified)
}
