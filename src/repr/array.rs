/*!
An array-like representation of binary data, comma-separated and enclosed in square brackets with
an identifying radix prefix. The *compact* representation **does not** allow whitespace after
commas or between bytes and the enclosing brackets.

```ebnf
ArrayRepresentation
    ::= BinaryArrayRepr | DecimalArrayRepr | OctalArrayRepr
        | LowerHexArrayRepr | UpperHexArrayRepr

BinaryArrayRepr
    ::= '0b' '[' [ BinaryByte { ',' BinaryByte } ] ']'

BinaryByte
    ::= [0-1]{1-8}

DecimalArrayRepr
    ::= '0d' '[' [ DecimalByte { ',' DecimalByte } ] ']'

DecimalByte
    ::= [09]{1-3}

OctalArrayRepr
    ::= '0o' '[' [ OctalByte { ',' OctalByte } ] ']'

OctalByte
    ::= [0-7]{1-3}

LowerHexArrayRepr
    ::= '0x' '[' [ LowerHexByte { ',' LowerHexByte } ] ']'

LowerHexByte
    ::= [0-9a-f]{1-2}

UpperHexArrayRepr
    ::= '0X' '[' [ UpperHexByte { ',' UpperHexByte } ] ']'

UpperHexByte
    ::= [0-9A-F]{1-2}
```

*/

use crate::{
    error::Error,
    repr::{ByteStyle, RadixFormat, ReprStyle},
    Binary,
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
    matches,
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
        let style = ReprStyle::Prefix.display_style(true);
        format!("{style}{}{style:#}", options.radix_format.prefix_str(),)
    } else {
        options.radix_format.prefix_str().to_string()
    };
    let (left_paren, right_paren) = if options.colored {
        let style = ReprStyle::Delimiter.display_style(true);
        (format!("{style}[{style:#}"), format!("{style}]{style:#}"))
    } else {
        ("[".to_string(), "]".to_string())
    };
    let comma = if options.colored {
        let style = ReprStyle::Separator.display_style(true);
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
                    let style = ByteStyle::ascii_char_display_style(&b, true);
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
    if !s.starts_with(|c| matches!(c, 'b' | 'd' | 'o' | 'x' | 'X')) {
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

impl ArrayFormatOptions {
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
