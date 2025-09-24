use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

mod utils;
use utils::*;

#[template]
#[rstest]
// New test cases
#[case(b"", "")]
#[case(b"A", "IE")]
#[case(b"AB", "IFBA")]
#[case(b"ABC", "IFBEG")]
#[case(b"ABCD", "IFBEGRA")]
#[case(b"ABCDE", "IFBEGRCF")]
#[case(b"ABCDEF", "IFBEGRCFIY")]
//
#[case(b"12345", "GEZDGNBV")]
#[case(b"abcde", "MFRGGZDF")]
#[case(b"xyz", "PB4XU")]
fn base32_more_cases_2(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base32_more_cases_2)]
fn base32_encode_more_test_2(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base32 = Base32::default();
    assert_eq!(base32.encode(&inp).unwrap(), oup);
}

#[apply(base32_more_cases_2)]
fn base32_decode_more_test_2(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base32 = Base32::default();
    assert_eq!(base32.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "2")]
#[case(b"A", "3B")]
#[case(b"AB", "7LL")]
#[case(b"ABC", "SMmV")]
#[case(b"ABCD", "3zLQne")]
#[case(b"ABCDE", "B6wAE7p")]
#[case(b"ABCDEF", "jYYPZLYy")]
//
#[case(b"12345", "8rfAMwP")]
#[case(b"abcde", "FZU9wSf")]
#[case(b"xyz", "nxfc")]
fn base56_more_cases_2(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base56_more_cases_2)]
fn base56_encode_more_test_2(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base56 = Base56::default();
    assert_eq!(base56.encode(&inp).unwrap(), oup);
}

#[apply(base56_more_cases_2)]
fn base56_decode_more_test_2(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = if input == "2" { vec![0u8, 0u8] } else { output.to_vec() };
    let base56 = Base56::default();
    assert_eq!(base56.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "1")]
#[case(b"A", "28")]
#[case(b"AB", "5y3")]
#[case(b"ABC", "NvLz")]
#[case(b"ABCD", "2fkTDm")]
#[case(b"ABCDE", "8N2njLQ")]
#[case(b"ABCDEF", "ZVptqrdj")]
//
#[case(b"12345", "6YvUFcg")]
#[case(b"abcde", "BzFRgmr")]
#[case(b"xyz", "hU2u")]
fn base58_more_cases_2(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base58_more_cases_2)]
fn base58_encode_more_test_2(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base58 = Base58::default();
    assert_eq!(base58.encode(&inp).unwrap(), oup);
}

#[apply(base58_more_cases_2)]
fn base58_decode_more_test_2(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = if input == "1" { vec![0u8, 0u8] } else { output.to_vec() };
    let base58 = Base58::default();
    assert_eq!(base58.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "")]
#[case(b"A", "QQ")]
#[case(b"AB", "QUI")]
#[case(b"ABC", "QUJD")]
#[case(b"ABCD", "QUJDRA")]
#[case(b"ABCDE", "QUJDREU")]
#[case(b"ABCDEF", "QUJDREVG")]
//
#[case(b"12345", "MTIzNDU")]
#[case(b"abcde", "YWJjZGU")]
#[case(b"xyz", "eHl6")]
fn base64_more_cases_2(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base64_more_cases_2)]
fn base64_encode_more_test_2(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base64 = Base64::default();
    assert_eq!(base64.encode(&inp).unwrap(), oup);
}

#[apply(base64_more_cases_2)]
fn base64_decode_more_test_2(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base64 = Base64::default();
    assert_eq!(base64.decode(&inp).unwrap(), oup);
}
