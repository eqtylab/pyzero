use std::io::Write;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::guest_interface;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PythonCodeResult {
    pub public_code: Vec<Option<String>>,
    pub public_args: Vec<Option<String>>,
    pub stdout: String,
}

impl PythonCodeResult {
    pub fn write_summary(&self, w: &mut dyn Write) -> Result<()> {
        let bar_width = 80;

        let write_bar = |w: &mut dyn Write| -> Result<()> {
            for _ in 0..bar_width {
                write!(w, "=")?;
            }
            writeln!(w)?;
            Ok(())
        };

        write_bar(w)?;
        writeln!(w, "PyZero Result Summary")?;
        write_bar(w)?;
        writeln!(w, "Code:\n")?;

        for (i, line) in self.public_code.iter().enumerate() {
            writeln!(
                w,
                "{:<4}| {}",
                i + 1,
                line.as_deref().unwrap_or("### __PY_ZERO_REDACTED__")
            )?;
        }

        write_bar(w)?;
        writeln!(w, "Args:\n")?;

        for (i, arg) in self.public_args.iter().enumerate() {
            writeln!(
                w,
                "sys.argv[{i}] = {}",
                arg.as_deref().unwrap_or("__PY_ZERO_REDACTED__")
            )?;
        }

        write_bar(w)?;
        writeln!(w, "Output:\n")?;

        for line in self.stdout.lines() {
            writeln!(w, "{line}")?;
        }

        write_bar(w)?;

        Ok(())
    }
}

impl From<guest_interface::PythonCodeResult> for PythonCodeResult {
    fn from(result: guest_interface::PythonCodeResult) -> Self {
        let guest_interface::PythonCodeResult {
            public_code,
            public_args,
            stdout,
        } = result;

        Self {
            public_code,
            public_args,
            stdout,
        }
    }
}
