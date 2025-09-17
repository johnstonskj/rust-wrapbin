//!
//! Provides formatting of `Binary` values.
//!
//! # Representation Descriptions
//!
//! Common definitions
//!
//! ```ebnf
//! PrefixString            ::= BinaryPrefixString | DecimalPrefixString
//!                           | OctalStringRepr | LowerHexPrefixString
//!                           | UpperHexPrefixString
//! BinaryPrefixString      ::= PrefixSignalChar 'b'
//! DecimalPrefixString     ::= PrefixSignalChar ''
//! OctalStringRepr         ::= PrefixSignalChar 'o'
//! LowerHexPrefixString    ::= PrefixSignalChar 'x'
//! UpperHexPrefixString    ::= PrefixSignalChar 'X'
//! PrefixSignalChar        ::= '0'
//! ```
//!
//! ```ebnf
//! BinaryByte              ::= [0-1]{1-8}
//! FixedBinaryByte         ::= [0-1]{8}
//! DecimalByte             ::= [09]{1-3}
//! FixedDecimalByte        ::= [09]{3}
//! OctalByte               ::= [0-7]{1-3}
//! FixedOctalByte          ::= [0-7]{3}
//! LowerHexByte            ::= [0-9a-f]{1-2}
//! FixedLowerHexByte       ::= [0-9a-f]{2}
//! UpperHexByte            ::= [0-9A-F]{1-2}
//! FixedUpperHexByte       ::= [0-9A-F]{2}
//! ```
//!
//! # Example Array Representation
//!
#![cfg_attr(
    any(
        not(feature = "repr-dump"),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```ignore"
)]
#![cfg_attr(
    all(feature = "repr-dump", not(feature = "repr-color")),
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
//!         ArrayFormatOptions::default().compact(true)),
//!     "0X[7B,E6,D4,F2,25,5C,62,D3,21,24,AB,7E,40,F1,7B,CE,17,3C,8,D2,D1,CE,CC,17]".to_string(),
//! );
//! ```
//!
//! # Example String Representation
//!
#![cfg_attr(
    any(
        not(feature = "repr-dump"),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```ignore"
)]
#![cfg_attr(
    all(feature = "repr-dump", not(feature = "repr-color")),
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
//! # Example Base-64 Representation
//!
#![cfg_attr(not(feature = "repr-base64"), doc = "```ignore")]
#![cfg_attr(feature = "repr-base64", doc = "```rust")]
//! use wrapbin::{
//!     Binary,
//!     repr::{BinaryFormatOptions, base64::Base64FormatOptions, format}
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
#![cfg_attr(
    any(
        not(feature = "repr-dump"),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```ignore"
)]
#![cfg_attr(
    all(feature = "repr-dump", not(feature = "repr-color")),
    doc = "```rust"
)]
//! use wrapbin::{
//!     Binary,
//!     repr::{BinaryFormatOptions, dump::DumpFormatOptions, format}
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
//!     vec![
//!         "0X       00 01 02 03 04 05 06 07 - 08 09 0A 0B 0C 0D 0E 0F ",
//!         "000000:  7B E6 D4 F2 25 5C 62 D3 - 21 24 AB 7E 40 F1 7B CE ",
//!         "000010:  17 3C 08 D2 D1 CE CC 17 - ",
//!     ].join("\n")
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

///
/// The Radix to use in representing bytes/octets in representation of binary data.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadixFormat {
    /// Represent each byte in radix-2, binary.
    Binary,
    /// Represent each byte in radix-8, octal.
    Octal,
    /// Represent each byte in radix-10, decimal.
    Decimal,
    /// Represent each byte in radix-16, hex with lower-case alpha characters.
    LowerHex,
    /// Represent each byte in radix-16, hex with upper-case alpha characters.
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

///
/// A classification for bytes based on ASCII 7-bit and 8-bit standardization.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteKind {
    /// A control character, this for some reason includes the ASCII space character.
    Control,
    /// A 7-bit ASCII printable character.
    Printable,
    /// An 8-bit (extended) ASCII printable character.
    PrintableExtended,
    /// An 8-bit (extended) ASCII undefined character
    Undefined,
}

///
/// A classification of the components makingup a representation form.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReprComponentKind {
    /// The prefix component that corresponds to the regular expression `(:?(?<radix>0[bdoxX]))`.
    Prefix,
    /// These are usually start/end pairs such as the `[` and `]` of an array or the `"` for strings.
    Delimiter,
    /// These are usually inter-byte separators such as `,` or `_`.
    Separator,
    /// Index values in the dump format that provide line and column numbers.
    Index,
    /// Actual data bytes.
    Value(ByteKind),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// This function ...
///
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
    ///
    /// Return a formatted prefix string for the current radix. This string is comprised of the
    /// character '0' followed by the radix identifying character.
    ///
    pub fn prefix_str(&self) -> &'static str {
        match self {
            RadixFormat::Binary => "0b",
            RadixFormat::Decimal => "0d",
            RadixFormat::LowerHex => "0x",
            RadixFormat::Octal => "0o",
            RadixFormat::UpperHex => "0X",
        }
    }

    ///
    /// Return the radix identifying character; one of 'b', 'd', 'o', 'x', or 'X'.
    ///
    pub fn prefix_char(&self) -> char {
        match self {
            RadixFormat::Binary => 'b',
            RadixFormat::Decimal => 'd',
            RadixFormat::LowerHex => 'd',
            RadixFormat::Octal => 'o',
            RadixFormat::UpperHex => 'X',
        }
    }

    ///
    /// Format a single byte according to the current radix and whether to pad to a
    /// fixed width or use a *compact* form.
    ///
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
    ///
    /// Attempt to parse a simgle character as a radix specifier.
    ///
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
    ///
    /// Return the radix, as an integer, for the format specifier.
    ///
    pub fn radix(&self) -> u32 {
        match self {
            RadixFormat::Binary => 2,
            RadixFormat::Octal => 8,
            RadixFormat::Decimal => 10,
            RadixFormat::LowerHex | RadixFormat::UpperHex => 16,
        }
    }
    ///
    /// Return the maximum number of digits required for the given radix.
    ///
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

#[inline(always)]
pub const fn has_color() -> bool {
    cfg!(feature = "repr-color")
}

#[doc(hidden)]
#[cfg(not(feature = "repr-color"))]
pub mod color {
    use crate::repr::{ByteKind, ReprComponentKind};

    pub type Style = str;

    impl ByteKind {
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

    impl ReprComponentKind {
        #[inline(always)]
        pub const fn display_style(&self, _: bool) -> &'static Style {
            ""
        }
    }
}

#[doc(hidden)]
#[cfg(feature = "repr-color")]
pub mod color {
    use crate::repr::{ByteKind, ReprComponentKind};
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

    impl ByteKind {
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

    impl ReprComponentKind {
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
