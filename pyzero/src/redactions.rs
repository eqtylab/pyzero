use anyhow::Result;
use pyzero_core::manifest::{ArgRedaction, LineRedaction, Redaction};

/// Converts a string of the form "1-3,5,7-9" into a list of line redactions.
pub fn redactions_from_code_redactions_list_string(
    code_redactions: &str,
) -> Result<Vec<Redaction>> {
    let redactions = index_list_from_redaction_list_string(code_redactions)?
        .iter()
        .map(|&i| {
            if i == 0 {
                panic!("Redaction line numbers are 1-indexed");
            } else {
                Redaction::Line(i - 1, LineRedaction::FullLine)
            }
        })
        .collect();

    Ok(redactions)
}

/// Converts a string of the form "0,2-4" into a list of arg redactions.
pub fn redactions_from_arg_redactions_list_string(arg_redactions: &str) -> Result<Vec<Redaction>> {
    let redactions = index_list_from_redaction_list_string(arg_redactions)?
        .iter()
        .map(|&i| Redaction::Arg(i, ArgRedaction::FullString))
        .collect();

    Ok(redactions)
}

fn index_list_from_redaction_list_string(list_string: &str) -> Result<Vec<u32>> {
    let index_list = list_string
        .split(',')
        .map(|s| {
            s.split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| match &v[..] {
            [start, end] => (*start..=*end).collect::<Vec<_>>(),
            [line] => vec![*line],
            _ => panic!("Invalid redaction list format"),
        })
        .flatten()
        .collect();

    Ok(index_list)
}
