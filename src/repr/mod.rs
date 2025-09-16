//!
//! Provides formatting of `Binary` values.
//!
//! # Example Array Representation
//!
#![cfg_attr(not(feature = "repr-array"), doc = "```ignore")]
#![cfg_attr(feature = "repr-array", doc = "```rust")]
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
//!         ArrayFormatOptions::default().compact(true)),
//!     "0X[7B,E6,D4,F2,25,5C,62,D3,21,24,AB,7E,40,F1,7B,CE,17,3C,8,D2,D1,CE,CC,17]".to_string(),
//! );
//! ```
//!
//! # Example String Representation
//!
#![cfg_attr(not(feature = "repr-string"), doc = "```ignore")]
#![cfg_attr(feature = "repr-string", doc = "```rust")]
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
//! # Example Base-64 Representation
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
//! # Example Dump Representation
//!
#![cfg_attr(not(feature = "repr-dump"), doc = "```ignore")]
#![cfg_attr(feature = "repr-dump", doc = "```rust")]
//! use wrapbin::{
//!     Binary,
//!     repr::{BinaryFormatOptions, format, dump::DumpFormatOptions}
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
//!         DumpFormatOptions::classic_hex_dump()),
//!     "0X       00 01 02 03 04 05 06 07 - 08 09 0A 0B 0C 0D 0E 0F \n000000:  7B E6 D4 F2 25 5C 62 D3 - 21 24 AB 7E 40 F1 7B CE \n000010:  17 3C 08 D2 D1 CE CC 17 - ".to_string(),
//! );
//! ```
//!

#[cfg(any(
    feature = "repr-array",
    feature = "repr-base64",
    feature = "repr-dump",
    feature = "repr-string"
))]
use crate::Binary; // only used in format() function.
use crate::error::Error;
use alloc::{format, string::String};
use core::{
    clone::Clone,
    default::Default,
    fmt::Debug,
    marker::Copy,
    option::Option::{self, Some},
    result::Result::{self, Err, Ok},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadixFormat {
    Binary,
    Decimal,
    LowerHex,
    Octal,
    #[default]
    UpperHex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryFormatOptions {
    #[cfg(feature = "repr-array")]
    Array(ArrayFormatOptions),
    #[cfg(feature = "repr-base64")]
    Base64(Base64FormatOptions),
    #[cfg(feature = "repr-dump")]
    Dump(DumpFormatOptions),
    #[cfg(feature = "repr-string")]
    String(StringFormatOptions),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteStyle {
    Control,
    Printable,
    PrintableExtended,
    Undefined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReprStyle {
    Prefix,
    Delimiter,
    Separator,
    Index,
    Value(ByteStyle),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[cfg(any(
    feature = "repr-array",
    feature = "repr-base64",
    feature = "repr-dump",
    feature = "repr-string"
))]
pub fn format<O: Into<BinaryFormatOptions>>(value: &Binary<'_>, options: O) -> String {
    match options.into() {
        #[cfg(feature = "repr-array")]
        BinaryFormatOptions::Array(options) => array_representation(value, &options),
        #[cfg(feature = "repr-base64")]
        BinaryFormatOptions::Base64(options) => base64_representation(value, &options),
        #[cfg(feature = "repr-dump")]
        BinaryFormatOptions::Dump(options) => dump_representation(value, &options),
        #[cfg(feature = "repr-string")]
        BinaryFormatOptions::String(options) => string_representation(value, &options),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Format Options ❱ RadixFormat
// ------------------------------------------------------------------------------------------------

impl RadixFormat {
    pub fn prefix_str(&self) -> &'static str {
        match self {
            RadixFormat::Binary => "0b",
            RadixFormat::Decimal => "0d",
            RadixFormat::LowerHex => "0x",
            RadixFormat::Octal => "0o",
            RadixFormat::UpperHex => "0X",
        }
    }

    pub fn format(&self, byte: &u8, compact: bool) -> String {
        match (self, compact) {
            (RadixFormat::Binary, true) => format!("{byte:b}"),
            (RadixFormat::Binary, false) => format!("{byte:08b}"),
            (RadixFormat::Decimal, true) => format!("{byte}"),
            (RadixFormat::Decimal, false) => format!("{byte:03}"),
            (RadixFormat::LowerHex, true) => format!("{byte:x}"),
            (RadixFormat::LowerHex, false) => format!("{byte:02x}"),
            (RadixFormat::Octal, true) => format!("{byte:o}"),
            (RadixFormat::Octal, false) => format!("{byte:03o}"),
            (RadixFormat::UpperHex, true) => format!("{byte:X}"),
            (RadixFormat::UpperHex, false) => format!("{byte:02X}"),
        }
    }
    pub fn from(specifier: Option<char>) -> Result<Self, Error> {
        match specifier {
            None => Ok(Self::default()),
            Some('b') => Ok(Self::Binary),
            Some('d') => Ok(Self::Decimal),
            Some('o') => Ok(Self::Octal),
            Some('x') => Ok(Self::LowerHex),
            Some('X') => Ok(Self::UpperHex),
            _ => Err(Error::InvalidRepresentation),
        }
    }
    pub fn radix(&self) -> u32 {
        match self {
            RadixFormat::Binary => 2,
            RadixFormat::Octal => 8,
            RadixFormat::Decimal => 10,
            RadixFormat::LowerHex | RadixFormat::UpperHex => 16,
        }
    }
    pub fn max_width(&self) -> usize {
        match self {
            RadixFormat::Binary => 8,
            RadixFormat::Decimal | RadixFormat::Octal => 3,
            RadixFormat::LowerHex | RadixFormat::UpperHex => 2,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[cfg(not(feature = "repr-color"))]
pub mod color {
    use crate::repr::{ByteStyle, ReprStyle};

    pub type Style = str;

    impl ByteStyle {
        #[inline(always)]
        pub const fn display_style(&self, _: bool) -> &'static Style {
            ""
        }
        #[inline(always)]
        pub const fn byte_to_style(_: u8) -> Self {
            Self::Printable
        }
        #[inline(always)]
        pub const fn ascii_char_display_style(_: &u8, _: bool) -> &'static Style {
            ""
        }
    }

    impl ReprStyle {
        #[inline(always)]
        pub const fn display_style(&self, _: bool) -> &'static Style {
            ""
        }
    }
}

#[doc(hidden)]
#[cfg(feature = "repr-color")]
pub mod color {
    use crate::repr::{ByteStyle, ReprStyle};
    use anstyle::{AnsiColor, Color};
    use core::option::Option::Some;

    pub use anstyle::Style;

    // ------------------------------------------------------------------------------------------------
    // Color/Styles
    // ------------------------------------------------------------------------------------------------

    const NO_STYLING: Style = Style::new();

    const PREFIX_STYLE: Style = Style::new();
    const DELIMITER_STYLE: Style = Style::new().dimmed();
    const SEPARATOR_STYLE: Style = Style::new().dimmed();
    const INDEX_STYLE: Style = Style::new().dimmed();

    const ASCII_CONTROL: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)))
        .bold();
    const ASCII_7BIT_PRINTABLE: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Green)))
        .bold();
    const ASCII_8BIT_PRINTABLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
    const ASCII_8BIT_UNDEFINED: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));

    // --------------------------------------------------------------------------------------------
    // Implementations
    // --------------------------------------------------------------------------------------------

    impl ByteStyle {
        pub const fn display_style(&self, colored: bool) -> &'static Style {
            if !colored {
                &NO_STYLING
            } else {
                match self {
                    Self::Control => &ASCII_CONTROL,
                    Self::Printable => &ASCII_7BIT_PRINTABLE,
                    Self::PrintableExtended => &ASCII_8BIT_PRINTABLE,
                    Self::Undefined => &ASCII_8BIT_UNDEFINED,
                }
            }
        }

        pub const fn ascii_char_display_style(byte: &u8, colored: bool) -> &'static Style {
            Self::byte_style(*byte).display_style(colored)
        }

        #[allow(clippy::self_named_constructors)]
        pub const fn byte_style(byte: u8) -> Self {
            match byte {
                0x00..=0x20 => Self::Control,
                0x21..=0x7E => Self::Printable,
                0x7F => Self::Control,
                0x80..=0x9F => Self::Undefined,
                0xA0 => Self::Control,
                0xA1..=0xAC => Self::PrintableExtended,
                0xAD => Self::Control,
                0xAE..=0xFF => Self::PrintableExtended,
            }
        }
    }

    impl ReprStyle {
        pub const fn display_style(&self, colored: bool) -> &'static Style {
            if !colored {
                &NO_STYLING
            } else {
                match self {
                    Self::Prefix => &PREFIX_STYLE,
                    Self::Delimiter => &DELIMITER_STYLE,
                    Self::Separator => &SEPARATOR_STYLE,
                    Self::Index => &INDEX_STYLE,
                    Self::Value(v) => v.display_style(colored),
                }
            }
        }
    }
}

#[cfg(feature = "repr-array")]
pub mod array;
#[cfg(feature = "repr-array")]
use crate::repr::array::{ArrayFormatOptions, array_representation};

#[cfg(feature = "repr-base64")]
pub mod base64;
#[cfg(feature = "repr-base64")]
use crate::repr::base64::{Base64FormatOptions, base64_representation};

#[cfg(feature = "repr-dump")]
pub mod dump;
#[cfg(feature = "repr-dump")]
use crate::repr::dump::{DumpFormatOptions, dump_representation};

#[cfg(feature = "repr-string")]
pub mod string;
#[cfg(feature = "repr-string")]
use crate::repr::string::{StringFormatOptions, string_representation};
