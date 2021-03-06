use failure::Fail;
use std::string::ToString;
use std::io;
use syn;

#[derive(Debug, Fail)]
pub enum EngWasmError {
    #[fail(display = "I/O error: {:?}", error)]
    IoError{
        error: String,
    },
    #[fail(display = "Json error: {}", error)]
    JsonError {
        error: String,
    },
    #[fail(display = "Token parse error: {}", error)]
    TokenParseError {
        error: String,
    }
}


impl From<io::Error> for EngWasmError {
    fn from(error: io::Error) -> Self {
        match error {
            _ => EngWasmError::IoError {error: error.to_string()},
        }
    }
}


impl From<serde_json::Error> for EngWasmError {
    fn from(err: serde_json::Error) -> Self {
        match err {
            _ => EngWasmError::JsonError {error: err.to_string()},
        }
    }
}

impl From<syn::parse::Error> for EngWasmError {
    fn from (err: syn::parse::Error) -> Self {
        match err {
            _ => EngWasmError::TokenParseError{error: err.to_string()}
        }
    }
}