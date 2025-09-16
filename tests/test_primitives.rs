use pretty_assertions::assert_eq;
use wrapbin::Binary;

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_from_u8() {
    let bin = Binary::from(u8::MIN);
    assert_eq!(bin, Binary::from(vec![00_u8]));
    let bin = Binary::from(0x0F_u8);
    assert_eq!(bin, Binary::from(vec![15_u8]));
    let bin = Binary::from(u8::MAX);
    assert_eq!(bin, Binary::from(vec![255_u8]));
}

#[test]
#[ignore = "wonky i8/u8 behavior"]
fn test_from_i8() {
    let bin = Binary::from(i8::MIN);
    assert_eq!(bin, Binary::from(vec![0x7F]));
    let bin = Binary::from(0_i8);
    assert_eq!(bin, Binary::from(vec![0x00]));
    let bin = Binary::from(i8::MAX);
    assert_eq!(bin, Binary::from(vec![0x00]));
}
