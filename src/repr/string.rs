/*!
A string-like representation of binary data, underscore-separated and enclosed in double quotes
with an identifying radix prefix. Note that the *compact* representation **does not allow**
underscores and so **all** bytes **must** be the same width with leading zeros as necessary.

```ebnf
StringRepresentation
    ::= BinaryStringRepr | DecimalStringRepr | OctalStringRepr
        | LowerHexStringRepr | UpperHexStringRepr

BinaryStringRepr
    ::= '0b' '"' [
            BinaryByte ( { '_' BinaryByte } | { BinaryByte } )
        ] '"'

BinaryByte
    ::= [0-1]{1-8}

DecimalStringRepr
    ::= '0d' '"' [
            DecimalByte ( { '_' DecimalByte } | { DecimalByte } )
        ] '"'

DecimalByte
    ::= [09]{1-3}

OctalStringRepr
    ::= '0o' '"' [
            OctalByte ( { '_' OctalByte } | { OctalByte } )
        ] '"'

OctalByte
    ::= [0-7]{1-3}

LowerHexStringRepr
    ::= '0x' '"' [
            LowerHexByte ( { '_' LowerHexByte } | { LowerHexByte } )
        ] '"'

LowerHexByte
    ::= [0-9a-f]{1-2}

UpperHexStringRepr
    ::= '0X' '"' [
            UpperHexByte ( { '_' UpperHexByte } | { UpperHexByte } )
        ] '"'

UpperHexByte
    ::= [0-9A-F]{1-2}
```

*/

use crate::{
    Binary,
    error::Error,
    repr::{ByteStyle, RadixFormat, ReprStyle},
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
    matches,
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
        let style = ReprStyle::Prefix.display_style(true);
        format!("{style}{}{style:#}", options.radix_format.prefix_str(),)
    } else {
        options.radix_format.prefix_str().to_string()
    };
    let quote = if options.colored {
        let style = ReprStyle::Delimiter.display_style(true);
        format!("{style}\"{style:#}")
    } else {
        '"'.to_string()
    };
    let underscore = if options.colored {
        let style = ReprStyle::Separator.display_style(true);
        format!("{style}_{style:#}")
    } else {
        '_'.to_string()
    };
    let mapped = value.as_ref().iter().map(|b| {
        if options.colored {
            let style = ByteStyle::ascii_char_display_style(&b, true);
            format!(
                "{style}{}{style:#}",
                options.radix_format.format(b, options.compact)
            )
        } else {
            options.radix_format.format(b, options.compact).to_string()
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
    if !s.starts_with(|c| matches!(c, 'b' | 'd' | 'o' | 'x' | 'X')) {
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

impl StringFormatOptions {
    pub fn with_byte_radix_format(mut self, radix_format: RadixFormat) -> Self {
        self.radix_format = radix_format;
        self
    }
    pub fn with_binary_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::Binary)
    }
    pub fn with_decimal_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::Decimal)
    }
    pub fn with_lower_hex_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::LowerHex)
    }
    pub fn with_octal_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::Octal)
    }
    pub fn with_upper_hex_bytes(self) -> Self {
        Self::with_byte_radix_format(self, RadixFormat::UpperHex)
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    #[cfg(feature = "repr-color")]
    pub fn use_color(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
}
