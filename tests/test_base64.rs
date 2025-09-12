#![cfg(feature = "repr-base64")]

use pretty_assertions::assert_eq;
use wrapbin::{
    repr::base64::{base64_representation, parse_base64_representation, Base64FormatOptions},
    Binary,
};

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

const LOREM_IPSUM_TEXT: &str = include_str!("lorem_ipsum_text.txt");
const LOREM_IPSUM: &str = include_str!("lorem_ipsum_b.txt");

#[test]
fn test_parse_base64() {
    let parsed = parse_base64_representation(LOREM_IPSUM);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.len(), 445);

    assert_eq!(parsed.as_ref(), LOREM_IPSUM_TEXT.as_bytes());
}

#[test]
fn test_base64_representation() {
    let repr = base64_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &Base64FormatOptions::default(),
    );
    assert_eq!(repr, LOREM_IPSUM);
}

#[test]
fn test_base64_representation_compact() {
    let repr = base64_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &Base64FormatOptions::default().compact(true),
    );
    assert_eq!(repr, &LOREM_IPSUM[..LOREM_IPSUM.len() - 2]);
}
