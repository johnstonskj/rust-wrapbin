//!
//! An array-like representation of binary data, comma-separated and enclosed in square brackets with
//! an identifying radix prefix. The *compact* representation **does not** allow whitespace after
//! commas or between bytes and the enclosing brackets.
//!
//! ```ebnf
//! ArrayRepresentation
//!     ::= BinaryArrayRepr | DecimalArrayRepr | OctalArrayRepr
//!         | LowerHexArrayRepr | UpperHexArrayRepr
//!
//! BinaryArrayRepr
//!     ::= '0b' '[' [ BinaryByte { ',' BinaryByte } ] ']'
//!
//! BinaryByte
//!     ::= [0-1]{1-8}
//!
//! DecimalArrayRepr
//!     ::= '0d' '[' [ DecimalByte { ',' DecimalByte } ] ']'
//!
//! DecimalByte
//!     ::= [09]{1-3}
//!
//! OctalArrayRepr
//!     ::= '0o' '[' [ OctalByte { ',' OctalByte } ] ']'
//!
//! OctalByte
//!     ::= [0-7]{1-3}
//!
//! LowerHexArrayRepr
//!     ::= '0x' '[' [ LowerHexByte { ',' LowerHexByte } ] ']'
//!
//! LowerHexByte
//!     ::= [0-9a-f]{1-2}
//!
//! UpperHexArrayRepr
//!     ::= '0X' '[' [ UpperHexByte { ',' UpperHexByte } ] ']'
//!
//! UpperHexByte
//!     ::= [0-9A-F]{1-2}
//! ```
//!
//! # Examples
//!
#![cfg_attr(not(feature = "repr-array"), doc = "```ignore")]
#![cfg_attr(
    any(
        all(feature = "repr-dump", not(feature = "repr-color")),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```rust"
)]
//! use wrapbin::{
//!     Binary,
//!     repr::{array::ArrayFormatOptions, BinaryFormatOptions, format}
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
//!         ArrayFormatOptions::default()),
//!     "0X[7B, E6, D4, F2, 25, 5C, 62, D3, 21, 24, AB, 7E, 40, F1, 7B, CE, 17, 3C, 08, D2, D1, CE, CC, 17]"
//!         .to_string(),
//! );
//! ```
//!
#![cfg_attr(
    any(
        all(feature = "repr-dump", not(feature = "repr-color")),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```rust"
)]
#![cfg_attr(not(feature = "repr-array"), doc = "```ignore")]
//! use wrapbin::{
//!     Binary,
//!     repr::{array::ArrayFormatOptions, BinaryFormatOptions, format}
//! };
//!
//! let binary = Binary::from([
//!     0x7b_u8,0xe6_u8,0xd4_u8,0xf2_u8,0x25_u8,0x5c_u8,0x62_u8,0xd3_u8,
//! ]);
//!
//! assert_eq!(
//!     format(
//!         &binary,
//!         ArrayFormatOptions::default().with_binary_bytes()),
//!     "0b[01111011, 11100110, 11010100, 11110010, 00100101, 01011100, 01100010, 11010011]"
//!         .to_string(),
//! );
//! ```
//!
#![cfg_attr(not(feature = "repr-array"), doc = "```ignore")]
#![cfg_attr(
    any(
        all(feature = "repr-dump", not(feature = "repr-color")),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```rust"
)]
//! use wrapbin::{
//!     Binary,
//!     repr::{array::ArrayFormatOptions, BinaryFormatOptions, format}
//! };
//!
//! let binary = Binary::from([
//!     0x7b_u8,0xe6_u8,0xd4_u8,0xf2_u8,0x25_u8,0x5c_u8,0x62_u8,0xd3_u8,
//! ]);
//!
//! assert_eq!(
//!     format(
//!         &binary,
//!         ArrayFormatOptions::default().with_octal_bytes()),
//!     "0o[173, 346, 324, 362, 045, 134, 142, 323]"
//!         .to_string(),
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
    cmp::{Eq, PartialEq},
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
pub struct ArrayFormatOptions {
    radix_format: RadixFormat,
    compact: bool,
    colored: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn array_representation(value: &Binary<'_>, options: &ArrayFormatOptions) -> String {
    let prefix = if options.colored {
        let style = ReprComponentKind::Prefix.display_style(true);
        format!("{style}{}{style:#}", options.radix_format.prefix_str(),)
    } else {
        options.radix_format.prefix_str().to_string()
    };
    let (left_paren, right_paren) = if options.colored {
        let style = ReprComponentKind::Delimiter.display_style(true);
        (format!("{style}[{style:#}"), format!("{style}]{style:#}"))
    } else {
        ("[".to_string(), "]".to_string())
    };
    let comma = if options.colored {
        let style = ReprComponentKind::Separator.display_style(true);
        format!(
            "{style},{style:#}{}",
            if options.compact { "" } else { " " }
        )
    } else {
        if options.compact { "," } else { ", " }.to_string()
    };
    format!(
        "{prefix}{left_paren}{}{right_paren}",
        value
            .as_ref()
            .iter()
            .map(|b| {
                if options.colored {
                    let style = ByteKind::ascii_char_display_style(b, true);
                    format!(
                        "{style}{}{style:#}",
                        options.radix_format.format(b, options.compact)
                    )
                } else {
                    options.radix_format.format(b, options.compact).to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(&comma)
    )
}

pub fn parse_array_representation(s: &str) -> Result<Binary<'_>, Error> {
    if !s.starts_with('0') {
        return Err(Error::MissingRadixPrefix);
    }
    let s = &s[1..];
    if !s.starts_with(['b', 'd', 'o', 'x', 'X']) {
        return Err(Error::InvalidRadixPrefix);
    }
    let radix_char = s.chars().next().unwrap();
    let s = &s[1..];
    if !(s.starts_with('[') && s.ends_with(']')) {
        return Err(Error::InvalidArrayBrackets);
    }
    let s = &s[1..s.len() - 1];
    if s.is_empty() {
        Ok(Binary::from(Vec::new()))
    } else {
        let byte_format = RadixFormat::from(Some(radix_char))?;
        let radix = byte_format.radix();
        let bytes = s.split(',');
        let mut result = Vec::new();
        for byte in bytes {
            result.push(u8::from_str_radix(byte.trim(), radix)?);
        }
        Ok(Binary::from(result))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<ArrayFormatOptions> for BinaryFormatOptions {
    fn from(value: ArrayFormatOptions) -> Self {
        Self::Array(value)
    }
}

impl ArrayFormatOptions {
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

    /// Use a compact representation, this will remove any extraneous whitespace from the
    /// generated form and also any leading zeros from generated, padded, numerics.
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
