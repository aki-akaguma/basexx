use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

mod utils;
use utils::*;

/*
#[test]
fn test_base32_i_1() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJM".to_string();
    let base32 = Base32I::default();
    assert_eq!(base32.encode(&inp).unwrap(), oup);
}

#[test]
fn test_base32_i_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJM".to_string();
    let base32 = Base32I::default();
    let r1 = base32.encode(&inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = base32.decode(&r1).unwrap();
    assert_eq!(r2, inp);
}
*/

#[test]
fn test_base32_i_file_t4_enc() {
    let inp = read_file_data("fixtures/t4.data");
    let oup = read_file_ascii("fixtures/t4.base32.ascii");
    let base32 = Base32I::default();
    assert_eq!(base32.encode(&inp).unwrap(), oup);
}

#[test]
fn test_base32_i_file_t4_dec() {
    let inp = read_file_ascii("fixtures/t4.base32.ascii");
    let oup = read_file_data("fixtures/t4.data");
    let base32 = Base32I::default();
    assert_eq!(base32.decode(&inp).unwrap(), oup);
}

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
fn base32_i_encode_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base32 = Base32I::default();
    assert_eq!(base32.encode(&inp).unwrap(), oup);
}

#[apply(two_simple_case_1)]
fn base32_i_decode_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base32 = Base32I::default();
    assert_eq!(base32.decode(&inp).unwrap(), oup);
}
