use risc0_zkvm::Journal;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// include this file instead of handling as a crate because `core` library
// and risc-v based `guest` binary should use different `serde` configs
include!("../../pyzero-core/methods/interface.rs");

use interface::PythonCodeResult;

#[wasm_bindgen]
pub fn json_obj_from_journal_bytes(journal: Vec<u8>) -> Result<JsValue, JsValue> {
    let pyzero_result = pyzero_result_from_journal_bytes(journal)?;

    let json_obj = serde_wasm_bindgen::to_value(&pyzero_result).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize decoded journal to JsValue: {}",
            e
        ))
    })?;

    Ok(json_obj)
}

#[wasm_bindgen]
pub fn statement_from_journal_bytes(journal: Vec<u8>) -> Result<JsValue, JsValue> {
    let pyzero_result = pyzero_result_from_journal_bytes(journal)?;

    let code = pyzero_result
        .public_code
        .iter()
        .map(|line| line.as_deref().unwrap_or("### __PYZERO_REDACTED__ ###"))
        .collect::<Vec<_>>()
        .join("\n");

    let args = pyzero_result
        .public_args
        .iter()
        .map(|arg| arg.as_deref().unwrap_or("<__PYZERO_REDACTED__>"))
        .enumerate()
        .map(|(i, arg)| format!("sys.argv[{i}] = {arg}"))
        .collect::<Vec<_>>()
        .join("\n");

    let stdout = pyzero_result.stdout;

    let statement = format!(
        r#"
# PyZero Proof Summary

## Code:
```python
{code}
```

## Args
```bash
{args}
```

## Stdout:
```bash
{stdout}
```
"#
    );

    let statement = JsValue::from_str(&statement);

    Ok(statement)
}

fn pyzero_result_from_journal_bytes(journal: Vec<u8>) -> Result<PythonCodeResult, JsValue> {
    let journal = Journal::new(journal);

    let journal = journal
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode journal: {e}")))?;

    Ok(journal)
}
