#![cfg(feature = "fmt")]

use pretty_assertions::assert_eq;
use wrapbin::Binary;

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

const LOREM_IPSUM_TEXT: &[u8] = b"Lorem ipsum";

#[test]
fn test_fmt_as_binary() {
    const EXPECTED: &str = "0b[01001100, 01101111, 01110010, 01100101, 01101101, 00100000, 01101001, 01110000, 01110011, 01110101, 01101101]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let binary_str = format!("{value:b}");
    assert_eq!(EXPECTED, binary_str);
}

#[test]
fn test_fmt_as_octal() {
    const EXPECTED: &str = "0o[114, 157, 162, 145, 155, 040, 151, 160, 163, 165, 155]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let octal_str = format!("{value:o}");
    assert_eq!(EXPECTED, octal_str);
}

#[test]
fn test_fmt_as_decimal() {
    const EXPECTED: &str = "0d[076, 111, 114, 101, 109, 032, 105, 112, 115, 117, 109]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let decimal_str = format!("{value}");
    assert_eq!(EXPECTED, decimal_str);
}

#[test]
fn test_fmt_as_lower_hex() {
    const EXPECTED: &str = "0x[4c, 6f, 72, 65, 6d, 20, 69, 70, 73, 75, 6d]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let lower_hex_str = format!("{value:x}");
    assert_eq!(EXPECTED, lower_hex_str);
}

#[test]
fn test_fmt_as_upper_hex() {
    const EXPECTED: &str = "0X[4C, 6F, 72, 65, 6D, 20, 69, 70, 73, 75, 6D]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let upper_hex_str = format!("{value:X}");
    assert_eq!(EXPECTED, upper_hex_str);
}

#[test]
fn test_fmt_as_binary_compact() {
    const EXPECTED: &str = "0b[1001100,1101111,1110010,1100101,1101101,100000,1101001,1110000,1110011,1110101,1101101]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let binary_str = format!("{value:#b}");
    assert_eq!(EXPECTED, binary_str);
}

#[test]
fn test_fmt_as_octal_compact() {
    const EXPECTED: &str = "0o[114,157,162,145,155,40,151,160,163,165,155]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let octal_str = format!("{value:#o}");
    assert_eq!(EXPECTED, octal_str);
}

#[test]
fn test_fmt_as_decimal_compact() {
    const EXPECTED: &str = "0d[76,111,114,101,109,32,105,112,115,117,109]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let decimal_str = format!("{value:#}");
    assert_eq!(EXPECTED, decimal_str);
}

#[test]
fn test_fmt_as_lower_hex_compact() {
    const EXPECTED: &str = "0x[4c,6f,72,65,6d,20,69,70,73,75,6d]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let lower_hex_str = format!("{value:#x}");
    assert_eq!(EXPECTED, lower_hex_str);
}

#[test]
fn test_fmt_as_upper_hex_compact() {
    const EXPECTED: &str = "0X[4C,6F,72,65,6D,20,69,70,73,75,6D]";
    let value = Binary::from(LOREM_IPSUM_TEXT);
    let upper_hex_str = format!("{value:#X}");
    assert_eq!(EXPECTED, upper_hex_str);
}
