use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;

use crate::model::Id;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("input file not found: {0}")]
    InputFileNotFound(PathBuf),

    #[error("failed to read input file {path}: {source}")]
    ReadFailed { path: PathBuf, source: io::Error },

    #[error("malformed record for keyword {keyword}: {detail}")]
    MalformedRecord { keyword: String, detail: String },

    #[error("failed to parse integer field {field}={value:?}: {source}")]
    MalformedInteger {
        field: String,
        value: String,
        source: ParseIntError,
    },

    #[error("failed to parse float field {field}={value:?}: {source}")]
    MalformedFloat {
        field: String,
        value: String,
        source: ParseFloatError,
    },

    #[error("duplicate {kind} id: {id}")]
    DuplicateId { kind: &'static str, id: Id },

    #[error("missing group name for group id {group_id}")]
    MissingGroupName { group_id: Id },

    #[error("element group {group_id} references unknown first element id {element_id}")]
    UnknownElementGroupReference { group_id: Id, element_id: Id },
}

pub type Result<T> = std::result::Result<T, ParserError>;
