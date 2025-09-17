//!
//! Hexadecimal dump of a file.
//!
//! ```ebnf
//! DumpRepresentation ::= [ HeaderLine ] { '\n' DataLine }
//!
//! HeaderLine ::= PrefixString ' '{4-7} (Column8   | Column16   | Column32  |
//!                          Column2C8 | Column2C16 | Column2C32)
//!
//! DataLine ::= LineIndex  (Column8   | Column16   | Column32  |
//!                          Column2C8 | Column2C16 | Column2C32)
//! LineIndex ::= Nybble{3-6} ': '
//!
//! Column8 ::= Byte ( ' ' Byte ){0-7}
//! Column2C8 ::= Column8 [ ' - ' Column8 ]
//! Column16 ::= Byte ( ' ' Byte ){0-15}
//! Column2C16 ::= Column16 [ ' - ' Column16 ]
//! Column32 ::= Byte ( ' ' Byte ){0-31}
//! Column2C32 ::= Column32 [ ' - ' Column32 ]
//!
//! Byte ::= Nybble Nybble Nybble?
//! Nybble ::= [0-9a-fA-F]
//! ```
//!
//! # Examples
//!
#![cfg_attr(not(feature = "repr-dump"), doc = "```ignore")]
#![cfg_attr(
    any(
        all(feature = "repr-dump", not(feature = "repr-color")),
        all(feature = "repr-dump", feature = "repr-color")
    ),
    doc = "```rust"
)]
//! use wrapbin::{
//!     Binary,
//!     repr::{BinaryFormatOptions, dump::DumpFormatOptions, format, has_color}
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

use crate::{
    Binary,
    error::Error,
    repr::{BinaryFormatOptions, ByteKind, RadixFormat, ReprComponentKind},
};
use alloc::{
    format,
    string::{String, ToString},
};
use core::{
    assert,
    clone::Clone,
    cmp::{Eq, PartialEq},
    default::Default,
    fmt::Debug,
    iter::Iterator,
    marker::Copy,
    option::Option::{self, None, Some},
    result::Result,
    todo, unreachable,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DumpFormatOptions {
    radix_format: RadixFormat,
    compact: bool,
    index_header_line: bool,
    index_line_numbers: bool,
    index_radix_format: RadixFormat,
    column_width: DumpColumnWidth,
    two_columns: bool,
    show_ascii: bool,
    show_extended_ascii: bool,
    line_index_spacing: String,
    value_spacing: String,
    column_separator: char,
    column_index_underline: Option<char>,
    colored: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(usize)]
pub enum DumpColumnWidth {
    #[default]
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn dump_representation(value: &Binary<'_>, options: &DumpFormatOptions) -> String {
    // --------------------------------------------------------------------------------------------
    // This is not supported the line indexes get ridiculous.
    // --------------------------------------------------------------------------------------------
    assert!(options.index_radix_format != RadixFormat::Binary);

    let (mid, end) = options.byte_counts();
    let mut buffer = String::default();

    // --------------------------------------------------------------------------------------------
    // Header line(s).
    // --------------------------------------------------------------------------------------------
    if options.index_header_line {
        buffer.push_str(&format!(
            "{:1$}{2:3$}",
            options.radix_format.prefix_str(),
            options.line_index_width(),
            "",
            options.line_index_spacing.len(),
        ));
        for i in 0..end {
            buffer.push_str(&options.format_column_index(i));
            if (i + 1) % end != 0 && options.two_columns && (i + 1) % mid == 0 {
                buffer.push_str(&options.format_column_separator());
            }
        }
        buffer.push('\n');
        if let Some(underline) = options.format_header_underline() {
            buffer.push_str(&format!(
                "{:1$}",
                "",
                options.line_index_width() + options.line_index_spacing.len()
            ));
            buffer.push_str(&underline);
        }
    }

    // --------------------------------------------------------------------------------------------
    // Actual data formatting.
    // --------------------------------------------------------------------------------------------
    for (index, byte) in value.iter().enumerate() {
        let one_index = index + 1;

        if options.index_line_numbers && index == 0 || index % end == 0 {
            buffer.push_str(&options.format_line_index(index));
        }

        if options.show_ascii {
            buffer.push_str(&options.format_ascii_char(byte));
        } else {
            buffer.push_str(&options.format_data_value(*byte));
        }

        if one_index % end == 0 {
            buffer.push('\n');
        } else if options.two_columns && one_index > 0 && one_index % mid == 0 {
            buffer.push_str(&options.format_column_separator());
        }
    }
    buffer
}

pub fn parse_dump_representation(_s: &str) -> Result<Binary<'_>, Error> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<DumpFormatOptions> for BinaryFormatOptions {
    fn from(value: DumpFormatOptions) -> Self {
        Self::Dump(value)
    }
}

impl Default for DumpFormatOptions {
    fn default() -> Self {
        Self {
            radix_format: RadixFormat::default(),
            compact: false,
            index_header_line: true,
            index_line_numbers: true,
            index_radix_format: RadixFormat::default(),
            column_width: DumpColumnWidth::default(),
            two_columns: true,
            show_ascii: false,
            show_extended_ascii: false,
            line_index_spacing: ":  ".to_string(),
            value_spacing: " ".to_string(),
            column_separator: '│',
            column_index_underline: Some('─'),
            colored: cfg!(feature = "repr-color"),
        }
    }
}

impl DumpFormatOptions {
    pub fn classic_hex_dump() -> Self {
        Self::default()
            .with_upper_hex_bytes()
            .with_upper_hex_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .no_column_index_underline()
            .separate_columns_with('-')
            .show_ascii(false)
    }

    pub fn ascii_hex_dump() -> Self {
        Self::default()
            .with_upper_hex_bytes()
            .with_upper_hex_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .no_column_index_underline()
            .separate_columns_with('-')
            .show_extended_ascii(true)
    }

    pub fn hex_dump() -> Self {
        Self::default()
            .with_upper_hex_bytes()
            .with_upper_hex_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .show_ascii(false)
    }

    pub fn octal_dump() -> Self {
        Self::default()
            .with_octal_bytes()
            .with_octal_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .show_ascii(false)
    }

    pub fn binary_dump() -> Self {
        Self::default()
            .with_binary_bytes()
            .with_upper_hex_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .show_ascii(false)
    }

    pub fn decimal_dump() -> Self {
        Self::default()
            .with_decimal_bytes()
            .with_decimal_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .show_ascii(false)
    }

    pub fn lower_hex_dump() -> Self {
        Self::default()
            .with_lower_hex_bytes()
            .with_lower_hex_indices()
            .compact(false)
            .two_columns_of(DumpColumnWidth::Eight)
            .show_ascii(false)
    }

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

    pub fn has_index_header_line(mut self, header_line: bool) -> Self {
        self.index_header_line = header_line;
        self
    }

    pub fn underline_column_index_with(mut self, underline: char) -> Self {
        self.column_index_underline = Some(underline);
        self
    }

    pub fn no_column_index_underline(mut self) -> Self {
        self.column_index_underline = None;
        self
    }

    pub fn separate_columns_with(mut self, column_separator: char) -> Self {
        self.column_separator = column_separator;
        self
    }

    pub fn has_index_line_numbers(mut self, line_numbers: bool) -> Self {
        self.index_line_numbers = line_numbers;
        self
    }

    /// Sets the radix format for line and column indices to be one of the values
    /// of the enum [`RadixFormat`].
    pub fn with_index_radix_format(mut self, index_radix_format: RadixFormat) -> Self {
        self.index_radix_format = index_radix_format;
        self
    }
    /// Sets the radix format for line and column indices to [`RadixFormat::Decimal`].
    pub fn with_decimal_indices(self) -> Self {
        Self::with_index_radix_format(self, RadixFormat::Decimal)
    }
    /// Sets the radix format for line and column indices to [`RadixFormat::LowerHex`].
    pub fn with_lower_hex_indices(self) -> Self {
        Self::with_index_radix_format(self, RadixFormat::LowerHex)
    }
    /// Sets the radix format for line and column indices to [`RadixFormat::Octal`].
    pub fn with_octal_indices(self) -> Self {
        Self::with_index_radix_format(self, RadixFormat::Octal)
    }
    /// Sets the radix format for line and column indices to [`RadixFormat::UpperHex`].
    pub fn with_upper_hex_indices(self) -> Self {
        Self::with_index_radix_format(self, RadixFormat::UpperHex)
    }

    pub fn has_two_columns(mut self, two_columns: bool) -> Self {
        self.two_columns = two_columns;
        self
    }

    pub fn with_column_width(mut self, column_width: DumpColumnWidth) -> Self {
        self.column_width = column_width;
        self
    }

    pub fn one_column_of(mut self, column_width: DumpColumnWidth) -> Self {
        self.two_columns = false;
        self.column_width = column_width;
        self
    }

    pub fn two_columns_of(mut self, column_width: DumpColumnWidth) -> Self {
        self.two_columns = true;
        self.column_width = column_width;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn show_ascii(mut self, show_ascii: bool) -> Self {
        self = self.with_upper_hex_bytes();
        self.show_ascii = show_ascii;
        self
    }

    pub fn show_extended_ascii(mut self, show_extended_ascii: bool) -> Self {
        self = self.show_ascii(true);
        self.show_extended_ascii = show_extended_ascii;
        self
    }

    /// Use color to denote byte kind according the ASCII conventions denoted by the
    /// enums `ByteStyle` and `ReprStyle`.
    #[cfg(feature = "repr-color")]
    pub fn use_color(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }

    const fn byte_counts(&self) -> (usize, usize) {
        match (self.two_columns, self.column_width) {
            (false, w @ DumpColumnWidth::Eight) => (0, w.byte_count()),
            (true, w @ DumpColumnWidth::Eight) => (w.byte_count(), w.two_column_byte_count()),
            (false, w @ DumpColumnWidth::Sixteen) => (0, w.byte_count()),
            (true, w @ DumpColumnWidth::Sixteen) => (w.byte_count(), w.two_column_byte_count()),
            (false, w @ DumpColumnWidth::ThirtyTwo) => (0, w.byte_count()),
            (true, w @ DumpColumnWidth::ThirtyTwo) => (w.byte_count(), w.two_column_byte_count()),
        }
    }

    fn format_column_index(&self, index: usize) -> String {
        let style = ReprComponentKind::Index.display_style(self.colored);
        match self.radix_format {
            RadixFormat::Binary => {
                format!(
                    "{style}{index:00$b}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::Decimal => {
                format!(
                    "{style}{index:00$}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::Octal => {
                format!(
                    "{style}{index:00$o}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::LowerHex => {
                format!(
                    "{style}{index:00$x}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::UpperHex => {
                format!(
                    "{style}{index:00$X}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
        }
    }

    fn format_header_underline(&self) -> Option<String> {
        if let Some(underline) = self.column_index_underline {
            let width =
                (self.data_value_width() + self.value_spacing.len()) * self.column_width as usize;
            let style = ReprComponentKind::Separator.display_style(self.colored);
            let underline = format!("{style}{}{style:#}", underline.to_string().repeat(width));
            let mut buffer = String::default();
            buffer.push_str(&underline);
            if self.two_columns {
                buffer.push_str(&self.format_column_separator());
                buffer.push_str(&underline);
            }
            buffer.push('\n');
            Some(buffer)
        } else {
            None
        }
    }

    fn format_column_separator(&self) -> String {
        let style = ReprComponentKind::Separator.display_style(self.colored);
        format!(
            "{style}{}{}{style:#}",
            self.column_separator, self.value_spacing
        )
    }

    const fn line_index_width(&self) -> usize {
        match self.index_radix_format {
            RadixFormat::Decimal | RadixFormat::Octal => 8,
            RadixFormat::LowerHex | RadixFormat::UpperHex => 6,
            _ => unreachable!(),
        }
    }

    fn format_line_index(&self, index: usize) -> String {
        let style = ReprComponentKind::Index.display_style(self.colored);
        match self.index_radix_format {
            RadixFormat::Decimal => format!(
                "{style}{index:0width$}{spacer}{style:#}",
                width = self.line_index_width(),
                spacer = self.line_index_spacing
            ),
            RadixFormat::Octal => format!(
                "{style}{index:0width$o}{spacer}{style:#}",
                width = self.line_index_width(),
                spacer = self.line_index_spacing
            ),
            RadixFormat::LowerHex => format!(
                "{style}{index:0width$x}{spacer}{style:#}",
                width = self.line_index_width(),
                spacer = self.line_index_spacing
            ),
            RadixFormat::UpperHex => format!(
                "{style}{index:0width$X}{spacer}{style:#}",
                width = self.line_index_width(),
                spacer = self.line_index_spacing
            ),
            _ => unreachable!(),
        }
    }

    const fn data_value_width(&self) -> usize {
        match self.radix_format {
            RadixFormat::Binary => 8,
            RadixFormat::Decimal | RadixFormat::Octal => 3,
            RadixFormat::LowerHex | RadixFormat::UpperHex => 2,
        }
    }

    fn format_data_value(&self, byte: u8) -> String {
        let style = ByteKind::ascii_char_display_style(&byte, self.colored);
        match self.radix_format {
            RadixFormat::Binary => {
                format!(
                    "{style}{byte:00$b}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::Decimal => {
                format!(
                    "{style}{byte:00$}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::Octal => {
                format!(
                    "{style}{byte:00$o}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::LowerHex => {
                format!(
                    "{style}{byte:00$x}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
            RadixFormat::UpperHex => {
                format!(
                    "{style}{byte:00$X}{style:#}{spacing}",
                    self.data_value_width(),
                    spacing = self.value_spacing
                )
            }
        }
    }

    fn format_ascii_char(&self, byte: &u8) -> String {
        // This follows ISO 8859-1.
        let decoded_char = match byte {
            // 7-bit ASCII control characters
            0x00 if self.show_extended_ascii => Some('␀'),
            0x01 if self.show_extended_ascii => Some('␁'),
            0x02 if self.show_extended_ascii => Some('␂'),
            0x03 if self.show_extended_ascii => Some('␃'),
            0x04 if self.show_extended_ascii => Some('␄'),
            0x05 if self.show_extended_ascii => Some('␅'),
            0x06 if self.show_extended_ascii => Some('␆'),
            0x07 if self.show_extended_ascii => Some('␇'),
            0x08 if self.show_extended_ascii => Some('␈'),
            0x09 if self.show_extended_ascii => Some('␉'),
            0x0A if self.show_extended_ascii => Some('␊'),
            0x0B if self.show_extended_ascii => Some('␋'),
            0x0C if self.show_extended_ascii => Some('␌'),
            0x0D if self.show_extended_ascii => Some('␍'),
            0x0E if self.show_extended_ascii => Some('␎'),
            0x0F if self.show_extended_ascii => Some('␏'),
            0x10 if self.show_extended_ascii => Some('␐'),
            0x11 if self.show_extended_ascii => Some('␑'),
            0x12 if self.show_extended_ascii => Some('␒'),
            0x13 if self.show_extended_ascii => Some('␓'),
            0x14 if self.show_extended_ascii => Some('␔'),
            0x15 if self.show_extended_ascii => Some('␕'),
            0x16 if self.show_extended_ascii => Some('␖'),
            0x17 if self.show_extended_ascii => Some('␗'),
            0x18 if self.show_extended_ascii => Some('␘'),
            0x19 if self.show_extended_ascii => Some('␙'),
            0x1A if self.show_extended_ascii => Some('␚'),
            0x1B if self.show_extended_ascii => Some('␛'),
            0x1C if self.show_extended_ascii => Some('␜'),
            0x1D if self.show_extended_ascii => Some('␝'),
            0x1E if self.show_extended_ascii => Some('␞'),
            0x1F if self.show_extended_ascii => Some('␟'),
            0x20 if self.show_extended_ascii => Some('␠'),
            // Printable 7-bit ASCII characters.
            0x21..=0x7E => Some(*byte as char),
            // 7-bit ASCII control character
            0x7F if self.show_extended_ascii => Some('␡'),
            // 8-bit Extended ASCII characters
            // 0x80..=0x9F: Not defined by ISO 8859-1.
            0xA0 if self.show_extended_ascii => Some('⍽'), // NBSP
            // AD: SHY (soft hyphen)
            // Printable 8-bit ASCII characters.
            0xA1..=0xAC | 0xAE..=0xFF => Some(*byte as char),
            _ => None, // Non-printable characters
        };
        let style = ByteKind::ascii_char_display_style(byte, self.colored);
        if let Some(c) = decoded_char {
            format!(
                "{style}{c:0$}{style:#}{spacing}",
                self.data_value_width(),
                spacing = self.value_spacing
            )
        } else {
            format!(
                "{style}{byte:00$X}{style:#}{spacing}",
                self.data_value_width(),
                spacing = self.value_spacing
            )
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > DumpColumnWidth
// ------------------------------------------------------------------------------------------------

impl DumpColumnWidth {
    #[inline(always)]
    pub const fn byte_count(&self) -> usize {
        *self as usize
    }

    #[inline(always)]
    pub const fn two_column_byte_count(&self) -> usize {
        self.byte_count() * 2
    }
}
