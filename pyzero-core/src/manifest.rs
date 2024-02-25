use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::guest_interface;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PythonCodeManifest {
    pub code: Vec<PythonCodeLine>,
    pub args: Vec<PythonArg>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PythonCodeLine {
    pub line: String,
    pub redaction: Option<LineRedaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PythonArg {
    pub arg: String,
    pub redaction: Option<ArgRedaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LineRedaction {
    FullLine,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArgRedaction {
    FullString,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Redaction {
    Line(u32, LineRedaction),
    Arg(u32, ArgRedaction),
}

pub fn make_python_code_manifest(
    code: &str,
    args: Vec<String>,
    redactions: Vec<Redaction>,
) -> Result<PythonCodeManifest> {
    let line_redactions = code
        .lines()
        .enumerate()
        .map(|(i, _)| {
            redactions.iter().find_map(|redaction| match redaction {
                Redaction::Line(line, redaction) if *line == i as u32 => Some(redaction.clone()),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    let arg_redactions = args
        .iter()
        .enumerate()
        .map(|(i, _)| {
            redactions.iter().find_map(|redaction| match redaction {
                Redaction::Arg(arg, redaction) if *arg == i as u32 => Some(redaction.clone()),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    let code = code
        .lines()
        .zip_eq(line_redactions)
        .map(|(line, redaction)| PythonCodeLine {
            line: line.to_owned(),
            redaction,
        })
        .collect();

    let args = args
        .iter()
        .zip_eq(arg_redactions)
        .map(|(arg, redaction)| PythonArg {
            arg: arg.to_owned(),
            redaction,
        })
        .collect();

    Ok(PythonCodeManifest { code, args })
}

impl From<PythonCodeManifest> for guest_interface::PythonCodeManifest {
    fn from(python_code_manifest: PythonCodeManifest) -> Self {
        let PythonCodeManifest { code, args } = python_code_manifest;

        let code = code.into_iter().map(Into::into).collect();
        let args = args.into_iter().map(Into::into).collect();

        Self { code, args }
    }
}

impl From<PythonCodeLine> for guest_interface::PythonCodeLine {
    fn from(python_code_line: PythonCodeLine) -> Self {
        let PythonCodeLine { line, redaction } = python_code_line;

        let redaction = redaction.into();

        Self { line, redaction }
    }
}

impl From<PythonArg> for guest_interface::PythonArg {
    fn from(python_arg: PythonArg) -> Self {
        let PythonArg { arg, redaction } = python_arg;

        let redaction = redaction.into();

        Self { arg, redaction }
    }
}

impl From<Option<LineRedaction>> for guest_interface::LineRedaction {
    fn from(line_redaction: Option<LineRedaction>) -> Self {
        match line_redaction {
            None => Self::None,
            Some(LineRedaction::FullLine) => Self::FullLine,
        }
    }
}

impl From<Option<ArgRedaction>> for guest_interface::ArgRedaction {
    fn from(arg_redaction: Option<ArgRedaction>) -> Self {
        match arg_redaction {
            None => Self::None,
            Some(ArgRedaction::FullString) => Self::FullString,
        }
    }
}
