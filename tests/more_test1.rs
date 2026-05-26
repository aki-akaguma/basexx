use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

mod utils;

#[template]
#[rstest]
// New test cases
#[case(b"", "")]
#[case(b"f", "MY")]
#[case(b"fo", "MZXQ")]
#[case(b"foo", "MZXW6")]
#[case(b"foob", "MZXW6YQ")]
#[case(b"fooba", "MZXW6YTB")]
#[case(b"foobar", "MZXW6YTBOI")]
//
#[case(b"Hello", "JBSWY3DP")]
#[case(b"World", "K5XXE3DE")]
#[case(b"Rust", "KJ2XG5A")]
fn base32_more_cases(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base32_more_cases)]
fn base32_encode_more_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base32 = Base32::default();
    assert_eq!(base32.encode(&inp).unwrap(), oup);
}

#[apply(base32_more_cases)]
fn base32_decode_more_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base32 = Base32::default();
    assert_eq!(base32.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "2")]
#[case(b"f", "3q")]
#[case(b"fo", "ANH")]
#[case(b"foo", "gEgZ")]
#[case(b"foob", "58k2HU")]
#[case(b"fooba", "GGs3Gib")]
#[case(b"foobar", "3BDvXmKau")]
//
#[case(b"Hello", "C6bDLV9")]
#[case(b"World", "EBtBeSe")]
#[case(b"Rust", "4WfaSw")]
fn base56_more_cases(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base56_more_cases)]
fn base56_encode_more_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base56 = Base56::default();
    assert_eq!(base56.encode(&inp).unwrap(), oup);
}

#[apply(base56_more_cases)]
fn base56_decode_more_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = if input == "2" {
        vec![0u8, 0u8]
    } else {
        output.to_vec()
    };
    let base56 = Base56::default();
    assert_eq!(base56.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "1")]
#[case(b"f", "2m")]
#[case(b"fo", "8o8")]
#[case(b"foo", "bQbp")]
#[case(b"foob", "3csAg9")]
#[case(b"fooba", "CZJRhmz")]
#[case(b"foobar", "t1Zv2yaZ")]
//
#[case(b"Hello", "9Ajdvzr")]
#[case(b"World", "As9UGqq")]
#[case(b"Rust", "37FSEo")]
fn base58_more_cases(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base58_more_cases)]
fn base58_encode_more_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base58 = Base58::default();
    assert_eq!(base58.encode(&inp).unwrap(), oup);
}

#[apply(base58_more_cases)]
fn base58_decode_more_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = if input == "1" {
        vec![0u8, 0u8]
    } else {
        output.to_vec()
    };
    let base58 = Base58::default();
    assert_eq!(base58.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "")]
#[case(b"f", "Zg")]
#[case(b"fo", "Zm8")]
#[case(b"foo", "Zm9v")]
#[case(b"foob", "Zm9vYg")]
#[case(b"fooba", "Zm9vYmE")]
#[case(b"foobar", "Zm9vYmFy")]
//
#[case(b"Hello", "SGVsbG8")]
#[case(b"World", "V29ybGQ")]
#[case(b"Rust", "UnVzdA")]
fn base64_more_cases(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base64_more_cases)]
fn base64_encode_more_test(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base64 = Base64::default();
    assert_eq!(base64.encode(&inp).unwrap(), oup);
}

#[apply(base64_more_cases)]
fn base64_decode_more_test(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base64 = Base64::default();
    assert_eq!(base64.decode(&inp).unwrap(), oup);
}
