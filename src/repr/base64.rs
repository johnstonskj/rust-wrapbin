/*!
A base64 encoding of binary data with optional padding.

*/

use crate::{Binary, error::Error};
use alloc::string::String;
use base64::prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD, Engine as _};
use core::{
    clone::Clone,
    cmp::{Eq, PartialEq},
    fmt::Debug,
    marker::Copy,
    result::Result::{self, Ok},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Base64FormatOptions {
    compact: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn base64_representation(value: &Binary<'_>, options: &Base64FormatOptions) -> String {
    let engine = if options.compact {
        BASE64_STANDARD_NO_PAD
    } else {
        BASE64_STANDARD
    };
    engine.encode(value.as_ref())
}

pub fn parse_base64_representation(s: &str) -> Result<Binary<'_>, Error> {
    Ok(Binary::from(BASE64_STANDARD.decode(s).unwrap()))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Base64FormatOptions {
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}
