#![no_main]

mod sys;

use risc0_zkvm::guest::env;
use rustpython_vm::{Interpreter, Settings};

// include this file instead of handling as a crate because `core` library
// and risc-v based `guest` binary use different `serde` configs
include!("../../interface.rs");

use interface::{
    ArgRedaction, LineRedaction, PythonArg, PythonCodeLine, PythonCodeManifest, PythonCodeResult,
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let PythonCodeManifest { code, args } = env::read();

    let mut full_code = String::new();
    let mut public_code = Vec::new();

    for PythonCodeLine { line, redaction } in &code {
        full_code.push_str(line);
        full_code.push_str("\n");

        public_code.push(match redaction {
            LineRedaction::None => Some(line.to_owned()),
            LineRedaction::FullLine => None,
        });
    }

    let mut full_args = Vec::new();
    let mut public_args = Vec::new();

    for PythonArg { arg, redaction } in &args {
        full_args.push(arg.clone());

        match redaction {
            ArgRedaction::None => public_args.push(Some(arg.clone())),
            ArgRedaction::FullString => public_args.push(None),
        }
    }

    run_python_code(&full_code, full_args);

    env::commit(&PythonCodeResult {
        public_code,
        public_args,
        stdout: sys::stdout::get_string(),
    });
}

fn run_python_code(code: &str, args: Vec<String>) {
    let mut settings = Settings::default();
    settings.argv = args;
    settings.optimize = 1;

    Interpreter::without_stdlib(settings).enter(|vm| {
        sys::add_to_vm(&vm);

        let scope = vm.new_scope_with_builtins();

        vm.run_code_string(scope, code, "<embedded>".to_owned())
            .unwrap();
    });
}
