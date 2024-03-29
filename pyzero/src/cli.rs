use std::path::PathBuf;

use clap::Parser;

/// pyzero
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(name = "pyzero")]
pub struct Args {
    /// Python code file to run
    #[clap(value_parser)]
    pub python_file: PathBuf,

    /// Python code arguments (sys.argv[..])
    #[clap(value_parser, last=true, num_args=0..=100)]
    pub python_args: Vec<String>,

    /// Code redactions list (example: "5,7-8")
    #[clap(short, long, value_parser)]
    pub code_redactions: Option<String>,

    /// Arg redactions list (example: "1,2")
    #[clap(short, long, value_parser)]
    pub arg_redactions: Option<String>,

    /// Proof file destination
    #[clap(short, long = "proof", default_value = "proof.bin", value_parser)]
    pub proof_path: PathBuf,

    /// Receipt file destination. (optional for convenience, receipt is also embedded in proof file)
    #[clap(short, long = "receipt", value_parser)]
    pub receipt_path: Option<PathBuf>,

    /// Execute without generating a proof
    #[clap(short, long)]
    pub dryrun: bool,
}
