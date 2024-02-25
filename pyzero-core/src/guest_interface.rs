// include this file instead of handling as a crate because `core` library
// and risc-v based `guest` binary use different `serde` configs
include!("../methods/interface.rs");

pub(crate) use interface::*;
