use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

/*
#[test]
fn test_base64_1() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "QUJDREVGR0hJSks".to_string();
    let base64 = Base64::default();
    assert_eq!(base64.encode(&inp).unwrap(), oup);
}

#[test]
fn test_base64_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "QUJDREVGR0hJSks".to_string();
    let base64 = Base64::default();
    let r1 = base64.encode(&inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = base64.decode(&r1).unwrap();
    assert_eq!(r2, inp);
}
*/

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
fn two_simple_case_1(#[case] input: &[u8], #[case] output: &str) {}

#[apply(two_simple_case_1)]
fn base64_encode_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base64 = Base64::default();
    assert_eq!(base64.encode(&inp).unwrap(), oup);
}

#[apply(two_simple_case_1)]
fn base64_decode_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base64 = Base64::default();
    assert_eq!(base64.decode(&inp).unwrap(), oup);
}
