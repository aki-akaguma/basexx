use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

#[test]
fn test_base58_1() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "HBb7dQEaKrdXjkN".to_string();
    let base58 = Base58::default();
    assert_eq!(base58.encode(&inp).unwrap(), oup);
}

#[test]
fn test_base58_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "HBb7dQEaKrdXjkN".to_string();
    let base58 = Base58::default();
    let r1 = base58.encode(&inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = base58.decode(&r1).unwrap();
    assert_eq!(r2, inp);
}
/*
*/

#[template]
#[rstest]
//
#[case(b"ABCDEFGHIJKL", "2ERjaFfYv6E4EfgR1")]
#[case(b"abcdefghijkl", "2qb7RmPbR2R5GaeYT")]
#[case(b"0123456789+/", "ukKH1gn66SKzxqKt")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "1FVk6iLh9oT6QH8")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255], "5d7Vt2W3PpMK5S2kv")]
//
#[case(b"ABCDEFGHIJK", "HBb7dQEaKrdXjkN")]
#[case(b"abcdefghijk", "R9doPkBWutDkQUz")]
#[case(b"0123456789+", "Cx8K8QWhaQQnkv6")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "14HUtbHhN2TkpR")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254], "23kAGBmBxvKedMAu")]
//
#[case(b"ABCDEFGHIJ", "4fedr2e4UP7vBb")]
#[case(b"abcdefghij", "6ULDKkKF2ZSzsB")]
#[case(b"0123456789", "3i37NcgooY8f1S")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9], "1kA3B2yGe2z4")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253], "EmDXB9CYhdQ2SQ")]
fn two_simple_case_1(#[case] input: &[u8], #[case] output: &str) {}

#[apply(two_simple_case_1)]
fn base58_encode_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base58 = Base58::default();
    assert_eq!(base58.encode(&inp).unwrap(), oup);
}

#[apply(two_simple_case_1)]
fn base58_decode_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base58 = Base58::default();
    assert_eq!(base58.decode(&inp).unwrap(), oup);
}
