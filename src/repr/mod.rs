/*!
One-line description.

More detailed description, with

# Example

```rust
```

*/

use crate::error::Error;
#[cfg(any(
    feature = "repr-array",
    feature = "repr-base64",
    feature = "repr-dump",
    feature = "repr-string"
))]
use crate::Binary;
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
pub enum BinaryFormat {
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
pub fn format(value: &Binary<'_>, options: &BinaryFormat) -> String {
    match options {
        #[cfg(feature = "repr-array")]
        BinaryFormat::Array(options) => array_representation(value, options),
        #[cfg(feature = "repr-base64")]
        BinaryFormat::Base64(options) => base64_representation(value, options),
        #[cfg(feature = "repr-dump")]
        BinaryFormat::Dump(options) => dump_representation(value, options),
        #[cfg(feature = "repr-string")]
        BinaryFormat::String(options) => string_representation(value, options),
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

#[cfg(not(feature = "repr-color"))]
pub mod color {
    use crate::repr::{ByteStyle, ReprStyle};

    pub type Style = str;

    impl ByteStyle {
        #[inline(always)]
        pub fn display_style(&self, _: bool) -> &'static Style {
            ""
        }
        #[inline(always)]
        pub fn byte_to_style(_: u8) -> Self {
            Self::Printable
        }
        #[inline(always)]
        pub fn ascii_char_display_style(_: &u8, _: bool) -> &'static Style {
            ""
        }
    }

    impl ReprStyle {
        #[inline(always)]
        pub fn display_style(&self, _: bool) -> &'static Style {
            ""
        }
    }
}

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
        pub fn display_style(&self, colored: bool) -> &'static Style {
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

        pub fn ascii_char_display_style(byte: &u8, colored: bool) -> &'static Style {
            Self::byte_style(*byte).display_style(colored)
        }

        #[allow(clippy::self_named_constructors)]
        pub fn byte_style(byte: u8) -> Self {
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
        pub fn display_style(&self, colored: bool) -> &'static Style {
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
use crate::repr::array::{array_representation, ArrayFormatOptions};

#[cfg(feature = "repr-base64")]
pub mod base64;
#[cfg(feature = "repr-base64")]
use crate::repr::base64::{base64_representation, Base64FormatOptions};

#[cfg(feature = "repr-dump")]
pub mod dump;
#[cfg(feature = "repr-dump")]
use crate::repr::dump::{dump_representation, DumpFormatOptions};

#[cfg(feature = "repr-string")]
pub mod string;
#[cfg(feature = "repr-string")]
use crate::repr::string::{string_representation, StringFormatOptions};
