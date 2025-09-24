use basexx::*;
use rstest::rstest;
use rstest_reuse::{self, *};

mod utils;

#[template]
#[rstest]
// New test cases
#[case(b"", "")]
#[case(b"1", "GE")]
#[case(b"12", "GEZA")]
#[case(b"123", "GEZDG")]
#[case(b"1234", "GEZDGNA")]
#[case(b"12345", "GEZDGNBV")]
#[case(b"123456", "GEZDGNBVGY")]
//
#[case(b"Hello, World!", "JBSWY3DPFQQFO33SNRSCC")]
#[case(b"Rust is awesome!", "KJ2XG5BANFZSAYLXMVZW63LFEE")]
#[case(b"Gemini CLI", "I5SW22LONEQEGTCJ")]
fn base32_more_cases_3(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base32_more_cases_3)]
fn base32_encode_more_test_3(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base32 = Base32::default();
    assert_eq!(base32.encode(&inp).unwrap(), oup);
}

#[apply(base32_more_cases_3)]
fn base32_decode_more_test_3(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base32 = Base32::default();
    assert_eq!(base32.decode(&inp).unwrap(), oup);
}

#[template]
#[rstest]
// New test cases
#[case(b"", "2")]
#[case(b"1", "t")]
#[case(b"12", "62u")]
#[case(b"123", "LN7V")]
#[case(b"1234", "3Vvt6N")]
#[case(b"12345", "8rfAMwP")]
#[case(b"123456", "ZKtqD9Sy")]
//
#[case(b"Hello, World!", "CxBzR9ze9K74Arc9Ct")]
#[case(b"Rust is awesome!", "PHTDz6GvQTDQBUQDPFVHHT")]
#[case(b"Gemini CLI", "8LUHKPEZFrQhsB")]
fn base56_more_cases_3(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base56_more_cases_3)]
fn base56_encode_more_test_3(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base56 = Base56::default();
    assert_eq!(base56.encode(&inp).unwrap(), oup);
}

#[apply(base56_more_cases_3)]
fn base56_decode_more_test_3(#[case] output: &[u8], #[case] input: &str) {
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
#[case(b"1", "r")]
#[case(b"12", "4k9")]
#[case(b"123", "HXRC")]
#[case(b"1234", "2FwFnT")]
#[case(b"12345", "6YvUFcg")]
#[case(b"123456", "RVu1HWU5")]
//
#[case(b"Hello, World!", "72k1xXWG59fYdzSNoA")]
#[case(b"Rust is awesome!", "BBaUGTwUuHnKxDhBvQWudJ")]
#[case(b"Gemini CLI", "51ea3smEp6bwBn")]
fn base58_more_cases_3(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base58_more_cases_3)]
fn base58_encode_more_test_3(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base58 = Base58::default();
    assert_eq!(base58.encode(&inp).unwrap(), oup);
}

#[apply(base58_more_cases_3)]
fn base58_decode_more_test_3(#[case] output: &[u8], #[case] input: &str) {
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
#[case(b"1", "MQ")]
#[case(b"12", "MTI")]
#[case(b"123", "MTIz")]
#[case(b"1234", "MTIzNA")]
#[case(b"12345", "MTIzNDU")]
#[case(b"123456", "MTIzNDU2")]
//
#[case(b"Hello, World!", "SGVsbG8sIFdvcmxkIQ")]
#[case(b"Rust is awesome!", "UnVzdCBpcyBhd2Vzb21lIQ")]
#[case(b"Gemini CLI", "R2VtaW5pIENMSQ")]
fn base64_more_cases_3(#[case] input: &[u8], #[case] output: &str) {}

#[apply(base64_more_cases_3)]
fn base64_encode_more_test_3(#[case] input: &[u8], #[case] output: &str) {
    let inp = input.to_vec();
    let oup = output.to_string();
    let base64 = Base64::default();
    assert_eq!(base64.encode(&inp).unwrap(), oup);
}

#[apply(base64_more_cases_3)]
fn base64_decode_more_test_3(#[case] output: &[u8], #[case] input: &str) {
    let inp = input.to_string();
    let oup = output.to_vec();
    let base64 = Base64::default();
    assert_eq!(base64.decode(&inp).unwrap(), oup);
}
