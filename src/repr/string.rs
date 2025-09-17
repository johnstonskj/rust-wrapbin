//!
//! A string-like representation of binary data, underscore-separated and enclosed in double quotes
//! with an identifying radix prefix. Note that the *compact* representation **does not allow**
//! underscores and so **all** bytes **must** be the same width with leading zeros as necessary.
//!
//! ```ebnf
//! StringRepresentation
//!     ::= BinaryStringRepr | DecimalStringRepr | OctalStringRepr
//!         | LowerHexStringRepr | UpperHexStringRepr
//!
//! BinaryStringRepr
//!     ::= '0b' '"' [
//!             BinaryByte ( { '_' BinaryByte } | { BinaryByte } )
//!         ] '"'
//!
//! BinaryByte
//!     ::= [0-1]{1-8}
//!
//! DecimalStringRepr
//!     ::= '0d' '"' [
//!             DecimalByte ( { '_' DecimalByte } | { DecimalByte } )
//!         ] '"'
//!
//! DecimalByte
//!     ::= [09]{1-3}
//!
//! OctalStringRepr
//!     ::= '0o' '"' [
//!             OctalByte ( { '_' OctalByte } | { OctalByte } )
//!         ] '"'
//!
//! OctalByte
//!     ::= [0-7]{1-3}
//!
//! LowerHexStringRepr
//!     ::= '0x' '"' [
//!             LowerHexByte ( { '_' LowerHexByte } | { LowerHexByte } )
//!         ] '"'
//!
//! LowerHexByte
//!     ::= [0-9a-f]{1-2}
//!
//! UpperHexStringRepr
//!     ::= '0X' '"' [
//!             UpperHexByte ( { '_' UpperHexByte } | { UpperHexByte } )
//!         ] '"'
//!
//! UpperHexByte
//!     ::= [0-9A-F]{1-2}
//! ```
//!
//! # Examples
//!
#![cfg_attr(not(feature = "repr-string"), doc = "```ignore")]
#![cfg_attr(
    any(
        all(feature = "repr-dump", not(feature = "repr-color")),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```rust"
)]
//! use wrapbin::{
//!     Binary,
//!     repr::{BinaryFormatOptions, format, string::StringFormatOptions}
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
//!         StringFormatOptions::default().compact(true)),
//!     r#"0X"7BE6D4F2255C62D32124AB7E40F17BCE173C08D2D1CECC17""#.to_string(),
//! );
//! ```
//!

use crate::{
    Binary,
    error::Error,
    repr::{BinaryFormatOptions, ByteKind, RadixFormat, ReprComponentKind},
};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    clone::Clone,
    convert::{AsRef, From},
    default::Default,
    fmt::Debug,
    iter::Iterator,
    marker::Copy,
    option::Option::Some,
    result::Result::{self, Err, Ok},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StringFormatOptions {
    radix_format: RadixFormat,
    compact: bool,
    colored: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn string_representation(value: &Binary<'_>, options: &StringFormatOptions) -> String {
    let prefix = if options.colored {
        let style = ReprComponentKind::Prefix.display_style(true);
        format!("{style}{}{style:#}", options.radix_format.prefix_str(),)
    } else {
        options.radix_format.prefix_str().to_string()
    };
    let quote = if options.colored {
        let style = ReprComponentKind::Delimiter.display_style(true);
        format!("{style}\"{style:#}")
    } else {
        '"'.to_string()
    };
    let underscore = if options.colored {
        let style = ReprComponentKind::Separator.display_style(true);
        format!("{style}_{style:#}")
    } else {
        '_'.to_string()
    };
    let mapped = value.as_ref().iter().map(|b| {
        // do not use variable width compact representation as compact depends
        // on knowing the width of each radix byte.
        if options.colored {
            let style = ByteKind::ascii_char_display_style(b, true);
            format!("{style}{}{style:#}", options.radix_format.format(b, false))
        } else {
            options.radix_format.format(b, false).to_string()
        }
    });
    format!(
        "{prefix}{quote}{}{quote}",
        if options.compact {
            mapped.collect::<String>()
        } else {
            mapped.collect::<Vec<_>>().join(&underscore)
        }
    )
}

pub fn parse_string_representation(s: &str) -> Result<Binary<'_>, Error> {
    if !s.starts_with('0') {
        return Err(Error::MissingRadixPrefix);
    }
    let s = &s[1..];
    if !s.starts_with(['b', 'd', 'o', 'x', 'X']) {
        return Err(Error::InvalidRadixPrefix);
    }
    let radix_char = s.chars().next().unwrap();
    let s = &s[1..];
    if !(s.starts_with('"') && s.ends_with('"')) {
        return Err(Error::InvalidStringQuotes);
    }
    let s = &s[1..s.len() - 1];
    if s.is_empty() {
        Ok(Binary::from(Vec::new()))
    } else {
        let byte_format = RadixFormat::from(Some(radix_char))?;
        let radix = byte_format.radix();
        let width = byte_format.max_width();
        let values: Vec<u8> = if s.contains('_') {
            let mut values = Vec::new();
            let bytes = s.split('_');
            for byte in bytes {
                values.push(u8::from_str_radix(byte, radix)?);
            }
            values
        } else {
            let mut rest = s;
            let mut values = Vec::new();
            while !rest.is_empty() {
                if width > rest.len() {
                    Err(Error::InvalidRepresentation)?
                }
                let (value, next) = rest.split_at(width);
                values.push(u8::from_str_radix(value, radix)?);
                rest = next;
            }
            values
        };
        Ok(Binary::from(values))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<StringFormatOptions> for BinaryFormatOptions {
    fn from(value: StringFormatOptions) -> Self {
        Self::String(value)
    }
}

impl StringFormatOptions {
    /// Sets the radix format for each byte in the array to be one of the values of the enum
    /// [`RadixFormat`].
    pub fn with_byte_radix_format(mut self, radix_format: RadixFormat) -> Self {
        self.radix_format = radix_format;
        self
    }
    /// Sets the radix format for each byte in the array to [`RadixFormat::Binary`].
    pub fn with_binary_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::Binary)
    }
    /// Sets the radix format for each byte in the array to [`RadixFormat::Decimal`].
    pub fn with_decimal_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::Decimal)
    }
    /// Sets the radix format for each byte in the array to [`RadixFormat::LowerHex`].
    pub fn with_lower_hex_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::LowerHex)
    }
    /// Sets the radix format for each byte in the array to [`RadixFormat::Octal`].
    pub fn with_octal_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::Octal)
    }
    /// Sets the radix format for each byte in the array to [`RadixFormat::UpperHex`].
    pub fn with_upper_hex_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::UpperHex)
    }

    /// Use a compact representation, this will remove any leading zeros from the
    /// generated, padded, numerics.
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Use color to denote byte kind according the ASCII conventions denoted by the
    /// enums `ByteStyle` and `ReprStyle`.
    #[cfg(feature = "repr-color")]
    pub fn use_color(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
}
