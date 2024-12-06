#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "AAABAQ".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = _encode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base64_scalar(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "QUJDREVGR0hJSktM".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = _encode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base64_scalar(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "QUJDREVGR0hJSks".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = _encode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base64_scalar(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "QUJDREVGR0hJSg".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = _encode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base64_scalar(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}

#[test]
fn test_base64_scalar_file_t4_enc() {
    let inp = test_utils::read_t4_data();
    let oup = test_utils::read_t4_base64_ascii();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = _encode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
}

#[test]
fn test_base64_scalar_file_t4_dec() {
    let inp = test_utils::read_t4_base64_ascii();
    let oup = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r2 = _decode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r2, oup);
}

use rstest::rstest;
use rstest_reuse::{self, *};

#[template]
#[rstest]
//
#[case(b"ABCDEFGHIJKL", "QUJDREVGR0hJSktM")]
#[case(b"abcdefghijkl", "YWJjZGVmZ2hpamts")]
#[case(b"0123456789+/", "MDEyMzQ1Njc4OSsv")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "AAECAwQFBgcICQoL")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255], "9PX29/j5+vv8/f7/")]
//
#[case(b"ABCDEFGHIJK", "QUJDREVGR0hJSks")]
#[case(b"abcdefghijk", "YWJjZGVmZ2hpams")]
#[case(b"0123456789+", "MDEyMzQ1Njc4OSs")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "AAECAwQFBgcICQo")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254], "9PX29/j5+vv8/f4")]
//
#[case(b"ABCDEFGHIJ", "QUJDREVGR0hJSg")]
#[case(b"abcdefghij", "YWJjZGVmZ2hpag")]
#[case(b"0123456789", "MDEyMzQ1Njc4OQ")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9], "AAECAwQFBgcICQ")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253], "9PX29/j5+vv8/Q")]
//
#[case(&[0u8, 1], "AAE")]
#[case(&[0u8,0, 1], "AAAB")]
#[case(&[0u8,0,0, 1], "AAAAAQ")]
#[case(&[0u8,0,0,0, 1], "AAAAAAE")]
#[case(&[0u8,0,0,0,0, 1], "AAAAAAAB")]
//
#[case(&[0u8,0,0,0,0,0, 1], "AAAAAAAAAQ")]
#[case(&[0u8,0,0,0,0,0,0, 1], "AAAAAAAAAAE")]
#[case(&[0u8,0,0,0,0,0,0,0, 1], "AAAAAAAAAAAB")]
#[case(&[0u8,0,0,0,0,0,0,0,0, 1], "AAAAAAAAAAAAAQ")]
#[case(&[0u8,0,0,0,0,0,0,0,0,0, 1], "AAAAAAAAAAAAAAE")]
fn two_simple_case_1(#[case] input: &[u8], #[case] output: &str) {}

#[apply(two_simple_case_1)]
fn base64_scalar_encode_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = _encode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
}

#[apply(two_simple_case_1)]
fn base64_scalar_decode_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r2 = _decode_base64_scalar(&ags, &inp).unwrap();
    assert_eq!(r2, oup);
}
