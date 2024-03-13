mod guest_interface;

pub mod manifest;
pub mod proof;
#[cfg(feature = "prove")]
pub mod prove;
pub mod result;
#[cfg(feature = "verify")]
pub mod verify;

pub fn guest_id() -> String {
    image_id_from_u32_array(pyzero_methods::PYZERO_GUEST_ID)
}

pub fn image_id_from_u32_array(arr: [u32; 8]) -> String {
    hex::encode(vec_u8_from_u32_slice_little_endian(&arr))
}

fn vec_u8_from_u32_slice_little_endian(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|&x| x.to_le_bytes().to_vec()).collect()
}
