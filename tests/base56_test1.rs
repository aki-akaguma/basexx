use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

#[test]
fn test_base56_1() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "UT9ZN6uuGzmJTem".to_string();
    let base56 = Base56::default();
    assert_eq!(base56.encode(&inp).unwrap(), oup);
}

#[test]
fn test_base56_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "UT9ZN6uuGzmJTem".to_string();
    let base56 = Base56::default();
    let r1 = base56.encode(&inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = base56.decode(&r1).unwrap();
    assert_eq!(r2, inp);
}
/*
*/

#[template]
#[rstest]
//
#[case(b"ABCDEFGHIJKL", "4AuuZMqSfnYxvFJ7w")]
#[case(b"abcdefghijkl", "5ES5tPV3nWYDV563w")]
#[case(b"0123456789+/", "3bJAs3TG6UpNTagnH")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "2QsyyNQQqW7s495")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255], "A7rnxTaNwbiiZUA6Z")]
//
#[case(b"ABCDEFGHIJK", "UT9ZN6uuGzmJTem")]
#[case(b"abcdefghijk", "hUGrYxB9zFc2fRd")]
#[case(b"0123456789+", "MXmwWKmLUsRibQV")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "272E3PxuVfeHkU")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254], "3mHhagrLaWPAstUG")]
//
#[case(b"ABCDEFGHIJ", "7mzucA69VmhEMc")]
#[case(b"abcdefghij", "AdVjuX9FzqigeC")]
#[case(b"0123456789", "6HGsFCtinP7CdK")]
#[case(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9], "237JdK54JYzCT")]
#[case(&[244u8, 245, 246, 247, 248, 249, 250, 251, 252, 253], "PiCQuVrpcDdCgp")]
//
#[case(&[0u8, 1], "23")]
#[case(&[0u8,0, 1], "223")]
#[case(&[0u8,0,0, 1], "2223")]
#[case(&[0u8,0,0,0, 1], "22223")]
#[case(&[0u8,0,0,0,0, 1], "222223")]
//
#[case(&[0u8,0,0,0,0,0, 1], "2222223")]
#[case(&[0u8,0,0,0,0,0,0, 1], "22222223")]
#[case(&[0u8,0,0,0,0,0,0,0, 1], "222222223")]
#[case(&[0u8,0,0,0,0,0,0,0,0, 1], "2222222223")]
#[case(&[0u8,0,0,0,0,0,0,0,0,0, 1], "22222222223")]
fn two_simple_case_1(#[case] input: &[u8], #[case] output: &str) {}

#[apply(two_simple_case_1)]
fn base56_encode_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base56 = Base56::default();
    assert_eq!(base56.encode(&inp).unwrap(), oup);
}

#[apply(two_simple_case_1)]
fn base56_decode_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base56 = Base56::default();
    assert_eq!(base56.decode(&inp).unwrap(), oup);
}
