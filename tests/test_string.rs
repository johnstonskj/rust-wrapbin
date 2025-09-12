use core::num::IntErrorKind;
use pretty_assertions::assert_eq;
use wrapbin::{
    Binary,
    error::Error,
    repr::string::{StringFormatOptions, parse_string_representation, string_representation},
};

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

const LOREM_IPSUM_TEXT: &str = include_str!("lorem_ipsum_text.txt");
const LOREM_IPSUM: &str = include_str!("lorem_ipsum_s.txt");

const TEST_ARRAY: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];

#[test]
fn test_parse_error_missing_radix_prefix() {
    let result = parse_string_representation("\"\"");
    assert_eq!(result, Err(Error::MissingRadixPrefix));
}

#[test]
fn test_parse_error_invalid_radix_prefix() {
    let result = parse_string_representation("0\"\"");
    assert_eq!(result, Err(Error::InvalidRadixPrefix));
}

#[test]
fn test_parse_error_invalid_radix_prefix_2() {
    let result = parse_string_representation("0c\"\"");
    assert_eq!(result, Err(Error::InvalidRadixPrefix));
}

#[test]
fn test_parse_error_invalid_string_quotes_1() {
    let result = parse_string_representation("0x00_ff\"");
    assert_eq!(result, Err(Error::InvalidStringQuotes));
}

#[test]
fn test_parse_error_invalid_string_quotes_2() {
    let result = parse_string_representation("0x\"00_ff");
    assert_eq!(result, Err(Error::InvalidStringQuotes));
}

#[test]
fn test_parse_error_invalid_byte_representation_1() {
    let result = parse_string_representation("0x\"0x\"");
    if let Err(Error::InvalidByteRepresentation { source }) = result {
        assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);
    } else {
        panic!("Expected InvalidByteRepresentation error");
    }
}

#[test]
fn test_parse_error_invalid_byte_representation_2() {
    let result = parse_string_representation("0x\"1ff\"");
    assert_eq!(result, Err(Error::InvalidRepresentation));
}

#[test]
fn test_parse_error_invalid_byte_representation_3() {
    let result = parse_string_representation("0x\"0 ff\"");
    if let Err(Error::InvalidByteRepresentation { source }) = result {
        assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);
    } else {
        panic!("Expected InvalidByteRepresentation error; got {result:#?}");
    }
}

#[test]
fn test_parse_string() {
    let parsed = parse_string_representation(LOREM_IPSUM);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 445);

    assert_eq!(parsed.as_ref(), LOREM_IPSUM_TEXT.as_bytes());
}

#[test]
fn test_parse_compact_string() {
    let lorem_ipsum_compact = LOREM_IPSUM.replace('_', "");
    let parsed = parse_string_representation(&lorem_ipsum_compact);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 445);

    assert_eq!(parsed.as_ref(), LOREM_IPSUM_TEXT.as_bytes());
}

#[test]
fn test_parse_empty_string() {
    let parsed = parse_string_representation("0X\"\"");
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 0);

    assert_eq!(parsed.as_ref(), &[]);
}

#[test]
fn test_string_representation() {
    let repr = string_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &StringFormatOptions::default(),
    );
    assert_eq!(repr, LOREM_IPSUM);
}

#[test]
fn test_string_representation_compact() {
    let repr = string_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &StringFormatOptions::default().compact(true),
    );
    assert_eq!(repr, LOREM_IPSUM.replace('_', ""));
}

#[test]
fn test_string_representation_radix_binary() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = string_representation(&binary, &StringFormatOptions::default().with_binary_bytes());
    assert_eq!(
        repr,
        r#"0b"00000000_00000001_00000010_00000011_00000100_00000101_00000110_00000111_00001000_00001001_00001010_00001011_00001100_00001101_00001110_00001111_00010000_00010001_00010010_00010011_00010100_00010101_00010110_00010111_00011000_00011001_00011010_00011011_00011100_00011101_00011110_00011111""#
    );
}

#[test]
fn test_string_representation_radix_octal() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = string_representation(&binary, &StringFormatOptions::default().with_octal_bytes());
    assert_eq!(
        repr,
        r#"0o"000_001_002_003_004_005_006_007_010_011_012_013_014_015_016_017_020_021_022_023_024_025_026_027_030_031_032_033_034_035_036_037""#
    );
}

#[test]
fn test_string_representation_radix_decimal() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = string_representation(
        &binary,
        &StringFormatOptions::default().with_decimal_bytes(),
    );
    assert_eq!(
        repr,
        r#"0d"000_001_002_003_004_005_006_007_008_009_010_011_012_013_014_015_016_017_018_019_020_021_022_023_024_025_026_027_028_029_030_031""#
    );
}

#[test]
fn test_string_representation_radix_lower_hex() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = string_representation(
        &binary,
        &StringFormatOptions::default().with_lower_hex_bytes(),
    );
    assert_eq!(
        repr,
        r#"0x"00_01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10_11_12_13_14_15_16_17_18_19_1a_1b_1c_1d_1e_1f""#
    );
}

#[test]
fn test_string_representation_radix_upper_hex() {
    let binary = Binary::from(TEST_ARRAY.as_slice());

    let repr = string_representation(
        &binary,
        &StringFormatOptions::default().with_upper_hex_bytes(),
    );
    assert_eq!(
        repr,
        r#"0X"00_01_02_03_04_05_06_07_08_09_0A_0B_0C_0D_0E_0F_10_11_12_13_14_15_16_17_18_19_1A_1B_1C_1D_1E_1F""#
    );
}

#[cfg(feature = "repr-color")]
const TEST_ARRAY_2: [u8; 32] = [
    0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 24, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52,
    54, 56, 58, 60, 62, 64, 66,
];

#[cfg(feature = "repr-color")]
#[test]
fn test_colored_string_representation_radix_upper_hex() {
    let binary = Binary::from(TEST_ARRAY_2.as_slice());

    let repr = string_representation(
        &binary,
        &StringFormatOptions::default()
            .with_upper_hex_bytes()
            .use_color(true),
    );
    println!("{repr}");
}
