#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "115S".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58b(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58b(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "2ERjaFfYv6E4EfgR1".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58b(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58b(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "HBb7dQEaKrdXjkN".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58b(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58b(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "4fedr2e4UP7vBb".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58b(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58b(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_4() {
    //0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
    //ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
    let inp = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec();
    let oup =
        "4tJBETbod9RcGRhGUXbdjU3t4jtbW8ySYiRVRn2XmqPVNZyexrFZ3S1mfsisKv2S2vgtSqPYk8WQy35D9dbbW"
            .to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58b(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58b(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}

use rstest::rstest;
use rstest_reuse::{self, *};

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
//
#[case(&[0u8, 1], "12")]
#[case(&[0u8,0, 1], "112")]
#[case(&[0u8,0,0, 1], "1112")]
#[case(&[0u8,0,0,0, 1], "11112")]
#[case(&[0u8,0,0,0,0, 1], "111112")]
//
#[case(&[0u8,0,0,0,0,0, 1], "1111112")]
#[case(&[0u8,0,0,0,0,0,0, 1], "11111112")]
#[case(&[0u8,0,0,0,0,0,0,0, 1], "111111112")]
#[case(&[0u8,0,0,0,0,0,0,0,0, 1], "1111111112")]
#[case(&[0u8,0,0,0,0,0,0,0,0,0, 1], "11111111112")]
fn two_simple_case_1(#[case] input: &[u8], #[case] output: &str) {}

#[apply(two_simple_case_1)]
fn base58b_encode_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base58 = Base58B::default();
    assert_eq!(base58.encode(&inp).unwrap(), oup);
}

#[apply(two_simple_case_1)]
fn base58b_decode_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base58 = Base58B::default();
    assert_eq!(base58.decode(&inp).unwrap(), oup);
}
