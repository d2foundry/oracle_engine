use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormulaFileError {
    #[error("Failed to write formula source file")]
    WriteError(#[from] std::io::Error),
    #[error("Failed to serialize formula json")]
    SerializeError(#[from] serde_json::Error),
}

fn json_0_float() -> Value {
    Value::Number(Number::from_f64(0.0).unwrap())
}

fn json_1_float() -> Value {
    Value::Number(Number::from_f64(1.0).unwrap())
}


pub fn write_formula_file(path: &Path) -> Result<(), FormulaFileError> {

    





    Ok(())
}