mod guest_interface;

pub mod manifest;
pub mod proof;
#[cfg(feature = "prove")]
pub mod prove;
pub mod result;
#[cfg(feature = "verify")]
pub mod verify;

pub fn guest_id() -> String {
    hex::encode(vec_u8_from_u32_slice_little_endian(
        &pyzero_methods::PYZERO_GUEST_ID,
    ))
}

fn vec_u8_from_u32_slice_little_endian(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|&x| x.to_le_bytes().to_vec()).collect()
}
