use pretty_assertions::assert_eq;
use wrapbin::Binary;

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

const TEST_BIN: &[u8] = b"Hello, World!";

#[test]
fn test_from_array() {
    let binary = Binary::from(TEST_BIN);
    assert!(binary.is_borrowed());
    assert_eq!(binary.len(), 13);
    assert_eq!(binary.as_ref(), TEST_BIN);
}

#[test]
fn test_from_vec() {
    let vec = TEST_BIN.to_vec();
    let binary = Binary::from(vec);
    assert!(binary.is_owned());
    assert_eq!(binary.len(), 13);
    assert_eq!(binary.as_ref(), TEST_BIN);
}

#[test]
fn test_clear() {
    let mut binary = Binary::from(TEST_BIN);
    assert!(binary.is_borrowed());
    assert_eq!(binary.len(), 13);
    assert_eq!(binary.as_ref(), TEST_BIN);

    binary.clear();
    assert!(binary.is_owned());
    assert_eq!(binary.len(), 0);
}
