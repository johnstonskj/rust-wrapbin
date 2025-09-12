/*!
Provides this crate's [`Error`] and [`Result`] types as well as helper functions.

 */

use alloc::{format, string::ToString};
use core::{
    cmp::PartialEq,
    convert::From,
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    option::Option::{self, None, Some},
    result::Result as StdResult,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(PartialEq)]
pub enum Error {
    /// Invalid representation of a binary string, or could not detect encoding.
    InvalidRepresentation,
    /// A string representation is missing a radix prefix (e.g., `0x` for hex).
    MissingRadixPrefix,
    /// A string representation has an invalid radix prefix (e.g., not one of `0b`, `0d`, `0o`, `0x`, or `0X`).
    InvalidRadixPrefix,
    /// A string representation is not correctly enclosed in double quotes.
    InvalidStringQuotes,
    /// An array representation is not correctly enclosed in brackets `[` and `]`.
    InvalidArrayBrackets,
    /// A string representing a `u8` byte value could not be parsed.
    InvalidByteRepresentation { source: ParseIntError },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = StdResult<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an `Error` from the provided source error.
#[inline]
pub fn parse_error(source: ParseIntError) -> Error {
    Error::InvalidByteRepresentation { source }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidRepresentation => write!(f, "InvalidRepresentation"),
            Self::MissingRadixPrefix => write!(f, "MissingRadixPrefix"),
            Self::InvalidRadixPrefix => write!(f, "InvalidRadixPrefix"),
            Self::InvalidStringQuotes => write!(f, "InvalidStringQuotes"),
            Self::InvalidArrayBrackets => write!(f, "InvalidArrayBrackets"),
            Self::InvalidByteRepresentation { source } => f
                .debug_struct("InvalidByteRepresentation")
                .field("source", source)
                .finish(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidRepresentation =>
                    "The binary string representation is invalid.".to_string(),
                Self::MissingRadixPrefix =>
                    "The binary string representation is missing a radix prefix.".to_string(),
                Self::InvalidRadixPrefix =>
                    "The binary string representation has an invalid radix prefix.".to_string(),
                Self::InvalidStringQuotes =>
                    "The binary string representation must be correctly enclosed in double quotes: '\"'."
                        .to_string(),
                Self::InvalidArrayBrackets =>
                    "The binary array representation must be correctly enclosed in brackets: '[' and ']'.".to_string(),
                Self::InvalidByteRepresentation { source } => {
                    format!("Failed to parse individual byte representation; source error: {source}")
                }
            }
        )
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidByteRepresentation { source } => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From<*>
// ------------------------------------------------------------------------------------------------

impl From<ParseIntError> for Error {
    fn from(source: ParseIntError) -> Self {
        Self::InvalidByteRepresentation { source }
    }
}
