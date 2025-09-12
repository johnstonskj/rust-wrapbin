use core::num::IntErrorKind;
use pretty_assertions::assert_eq;
use wrapbin::{
    error::Error,
    repr::array::{array_representation, parse_array_representation, ArrayFormatOptions},
    Binary,
};

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

const LOREM_IPSUM_TEXT: &str = include_str!("lorem_ipsum_text.txt");
const LOREM_IPSUM: &str = include_str!("lorem_ipsum_a.txt");

const TEST_ARRAY: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];

#[test]
fn test_parse_error_missing_radix_prefix() {
    let result = parse_array_representation("[]");
    assert_eq!(result, Err(Error::MissingRadixPrefix));
}

#[test]
fn test_parse_error_invalid_radix_prefix() {
    let result = parse_array_representation("0[]");
    assert_eq!(result, Err(Error::InvalidRadixPrefix));
}

#[test]
fn test_parse_error_invalid_radix_prefix_2() {
    let result = parse_array_representation("0c[]");
    assert_eq!(result, Err(Error::InvalidRadixPrefix));
}

#[test]
fn test_parse_error_invalid_array_brackets_1() {
    let result = parse_array_representation("0x00, ff]");
    assert_eq!(result, Err(Error::InvalidArrayBrackets));
}

#[test]
fn test_parse_error_invalid_array_brackets_2() {
    let result = parse_array_representation("0x[00, ff");
    assert_eq!(result, Err(Error::InvalidArrayBrackets));
}

#[test]
fn test_parse_error_invalid_byte_representation_1() {
    let result = parse_array_representation("0x[0x]");
    if let Err(Error::InvalidByteRepresentation { source }) = result {
        assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);
    } else {
        panic!("Expected InvalidByteRepresentation error");
    }
}

#[test]
fn test_parse_error_invalid_byte_representation_2() {
    let result = parse_array_representation("0x[1ff]");
    if let Err(Error::InvalidByteRepresentation { source }) = result {
        assert_eq!(source.kind(), &IntErrorKind::PosOverflow);
    } else {
        panic!("Expected InvalidByteRepresentation error; got {result:#?}");
    }
}

#[test]
fn test_parse_error_invalid_byte_representation_3() {
    let result = parse_array_representation("0x[1 ff]");
    if let Err(Error::InvalidByteRepresentation { source }) = result {
        assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);
    } else {
        panic!("Expected InvalidByteRepresentation error; got {result:#?}");
    }
}

#[test]
fn test_parse_array() {
    let parsed = parse_array_representation(LOREM_IPSUM);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 445);

    assert_eq!(parsed.as_ref(), LOREM_IPSUM_TEXT.as_bytes());
}

#[test]
fn test_parse_compact_array() {
    let lorem_ipsum_compact = LOREM_IPSUM.replace(", ", ",");
    let parsed = parse_array_representation(&lorem_ipsum_compact);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 445);

    assert_eq!(parsed.as_ref(), LOREM_IPSUM_TEXT.as_bytes());
}

#[test]
fn test_parse_empty_array() {
    let parsed = parse_array_representation("0X[]");
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 0);

    assert_eq!(parsed.as_ref(), &[]);
}

#[test]
fn test_array_representation() {
    let repr = array_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &ArrayFormatOptions::default(),
    );
    assert_eq!(repr, LOREM_IPSUM);
}

#[test]
fn test_array_representation_compact() {
    let repr = array_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &ArrayFormatOptions::default().compact(true),
    );
    assert_eq!(repr, LOREM_IPSUM.replace(", ", ","));
}

#[test]
fn test_array_representation_radix_binary() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = array_representation(&binary, &ArrayFormatOptions::default().with_binary_bytes());
    assert_eq!(
        repr,
        r#"0b[00000000, 00000001, 00000010, 00000011, 00000100, 00000101, 00000110, 00000111, 00001000, 00001001, 00001010, 00001011, 00001100, 00001101, 00001110, 00001111, 00010000, 00010001, 00010010, 00010011, 00010100, 00010101, 00010110, 00010111, 00011000, 00011001, 00011010, 00011011, 00011100, 00011101, 00011110, 00011111]"#
    );
}

#[test]
fn test_array_representation_radix_octal() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = array_representation(&binary, &ArrayFormatOptions::default().with_octal_bytes());
    assert_eq!(
        repr,
        r#"0o[000, 001, 002, 003, 004, 005, 006, 007, 010, 011, 012, 013, 014, 015, 016, 017, 020, 021, 022, 023, 024, 025, 026, 027, 030, 031, 032, 033, 034, 035, 036, 037]"#
    );
}

#[test]
fn test_array_representation_radix_decimal() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = array_representation(&binary, &ArrayFormatOptions::default().with_decimal_bytes());
    assert_eq!(
        repr,
        r#"0d[000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031]"#
    );
}

#[test]
fn test_array_representation_radix_lower_hex() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = array_representation(
        &binary,
        &ArrayFormatOptions::default().with_lower_hex_bytes(),
    );
    assert_eq!(
        repr,
        r#"0x[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]"#
    );
}

#[test]
fn test_array_representation_radix_upper_hex() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = array_representation(
        &binary,
        &ArrayFormatOptions::default().with_upper_hex_bytes(),
    );
    assert_eq!(
        repr,
        r#"0X[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0A, 0B, 0C, 0D, 0E, 0F, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1A, 1B, 1C, 1D, 1E, 1F]"#
    );
}

#[cfg(feature = "repr-color")]
const TEST_ARRAY_2: [u8; 32] = [
    0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 24, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52,
    54, 56, 58, 60, 62, 64, 66,
];

#[cfg(feature = "repr-color")]
#[test]
fn test_colored_array_representation_radix_upper_hex() {
    let binary = Binary::from(TEST_ARRAY_2.as_slice());

    let repr = array_representation(
        &binary,
        &ArrayFormatOptions::default()
            .with_upper_hex_bytes()
            .use_color(true),
    );
    println!("{repr}");
}
