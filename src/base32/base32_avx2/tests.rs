#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "AAAACAI".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base32_avx2(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJNGA".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base32_avx2(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJM".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base32_avx2(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "IFBEGRCFIZDUQSKK".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base32_avx2(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_4() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    //0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
    //ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
    let inp = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJNGE2TSPKBIVEU2UKVLFOWCZLJQWEY3EMVTGO2DJNJVWY3LON5YHC4TTOR2XM53YPF5DAMJSGM2DKNRXHA4Q"
        .to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base32_avx2(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}

#[test]
fn test_base32_avx2_file_t4_enc() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = test_utils::read_t4_data();
    let oup = test_utils::read_t4_base32_ascii();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
}

#[test]
fn test_base32_avx2_file_t4_dec() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = test_utils::read_t4_base32_ascii();
    let oup = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r2 = unsafe { _decode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r2, oup);
}

use rstest::rstest;
use rstest_reuse::{self, *};

#[template]
#[rstest]
//
#[case(b"ABCDEFGHIJKL", "IFBEGRCFIZDUQSKKJNGA")]
#[case(b"abcdefghijkl", "MFRGGZDFMZTWQ2LKNNWA")]
#[case(b"0123456789+/", "GAYTEMZUGU3DOOBZFMXQ")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "AAAQEAYEAUDAOCAJBIFQ")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255], "6T27N57Y7H5PX7H5737Q")]
//
#[case(b"ABCDEFGHIJK", "IFBEGRCFIZDUQSKKJM")]
#[case(b"abcdefghijk", "MFRGGZDFMZTWQ2LKNM")]
#[case(b"0123456789+", "GAYTEMZUGU3DOOBZFM")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "AAAQEAYEAUDAOCAJBI")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254], "6T27N57Y7H5PX7H57Y")]
//
#[case(b"ABCDEFGHIJ", "IFBEGRCFIZDUQSKK")]
#[case(b"abcdefghij", "MFRGGZDFMZTWQ2LK")]
#[case(b"0123456789", "GAYTEMZUGU3DOOBZ")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9], "AAAQEAYEAUDAOCAJ")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253], "6T27N57Y7H5PX7H5")]
//
#[case(&[0u8, 1], "AAAQ")]
#[case(&[0u8,0, 1], "AAAAC")]
#[case(&[0u8,0,0, 1], "AAAAAAI")]
#[case(&[0u8,0,0,0, 1], "AAAAAAAB")]
#[case(&[0u8,0,0,0,0, 1], "AAAAAAAAAE")]
//
#[case(&[0u8,0,0,0,0,0, 1], "AAAAAAAAAAAQ")]
#[case(&[0u8,0,0,0,0,0,0, 1], "AAAAAAAAAAAAC")]
#[case(&[0u8,0,0,0,0,0,0,0, 1], "AAAAAAAAAAAAAAI")]
#[case(&[0u8,0,0,0,0,0,0,0,0, 1], "AAAAAAAAAAAAAAAB")]
#[case(&[0u8,0,0,0,0,0,0,0,0,0, 1], "AAAAAAAAAAAAAAAAAE")]
fn two_simple_case_1(#[case] input: &[u8], #[case] output: &str) {}

#[apply(two_simple_case_1)]
fn base32_avx2_encode_test(#[case] input: &[u8], #[case] output: &str) {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = input.to_vec();
    let oup = output.to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _encode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
}

#[apply(two_simple_case_1)]
fn base32_avx2_decode_test(#[case] output: &[u8], #[case] input: &str) {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let inp = input.to_string();
    let oup = output.to_vec();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = unsafe { _decode_base32_avx2(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
}
