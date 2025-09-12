#![cfg(all(feature = "repr-dump", feature = "repr-color"))]

use wrapbin::{
    repr::dump::{dump_representation, DumpFormatOptions},
    Binary,
};

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

const LOREM_IPSUM_TEXT: &str = include_str!("lorem_ipsum_text.txt");

#[test]
fn test_colored_dump_representation_binary() {
    let repr = dump_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &DumpFormatOptions::default().with_binary_bytes(),
    );
    println!("{repr}");
}

#[test]
fn test_colored_dump_representation_octal() {
    let repr = dump_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &DumpFormatOptions::default().with_octal_bytes(),
    );
    println!("{repr}");
}

#[test]
fn test_colored_dump_representation_decimal() {
    let repr = dump_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &DumpFormatOptions::default().with_decimal_bytes(),
    );
    println!("{repr}");
}

#[test]
fn test_colored_dump_representation_upper_hex() {
    let repr = dump_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &DumpFormatOptions::default().with_upper_hex_bytes(),
    );
    println!("{repr}");
}

#[test]
fn test_colored_dump_representation_lower_hex() {
    let repr = dump_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &DumpFormatOptions::default().with_lower_hex_bytes(),
    );
    println!("{repr}");
}

#[test]
fn test_colored_dump_representation_ascii() {
    let repr = dump_representation(
        &Binary::from(LOREM_IPSUM_TEXT.as_bytes()),
        &DumpFormatOptions::default()
            .with_upper_hex_bytes()
            .show_ascii(true),
    );
    println!("{repr}");
}

#[test]
fn test_colored_dump_ascii_extended_chart() {
    let chart = Binary::from((0u8..=255).collect::<Vec<u8>>());
    let repr = dump_representation(
        &chart,
        &DumpFormatOptions::default()
            .with_upper_hex_bytes()
            .show_extended_ascii(true),
    );
    println!("{repr}");
}
