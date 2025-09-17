//!
//! A standard base64 encoding of binary data with optional padding.
//!
//! # Examples
//!
#![cfg_attr(not(feature = "repr-base64"), doc = "```ignore")]
#![cfg_attr(feature = "repr-base64", doc = "```rust")]
//! use wrapbin::{
//!     Binary,
//!     repr::{BinaryFormatOptions, format, base64::Base64FormatOptions}
//! };
//!
//! let binary = Binary::from([
//!     0x7b_u8,0xe6_u8,0xd4_u8,0xf2_u8,0x25_u8,0x5c_u8,0x62_u8,0xd3_u8,
//!     0x21_u8,0x24_u8,0xab_u8,0x7e_u8,0x40_u8,0xf1_u8,0x7b_u8,0xce_u8,
//!     0x17_u8,0x3c_u8,0x08_u8,0xd2_u8,0xd1_u8,0xce_u8,0xcc_u8,0x17_u8,
//! ]);
//!
//! assert_eq!(
//!     format(
//!         &binary,
//!         Base64FormatOptions::default()),
//!     "e+bU8iVcYtMhJKt+QPF7zhc8CNLRzswX".to_string(),
//! );
//! ```
//!

use crate::{Binary, error::Error, repr::BinaryFormatOptions};
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

impl From<Base64FormatOptions> for BinaryFormatOptions {
    fn from(value: Base64FormatOptions) -> Self {
        Self::Base64(value)
    }
}

impl Base64FormatOptions {
    /// Use a compact representation, this turns off standard base64 padding.
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}
