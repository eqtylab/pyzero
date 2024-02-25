use std::path::PathBuf;

use clap::Parser;

/// pyzero-verify
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(name = "pyzero-verify")]
pub struct Args {
    /// Proof file to verify
    #[clap(value_parser)]
    pub proof_file: PathBuf,
}
