#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;
//use rstest::rstest;
//use rstest_reuse::{self, *};

fn gen_data_ags_32() -> (AsciiGraphicSet, Vec<u8>, Vec<u8>) {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP32);
    let mut bin_buf = Vec::<u8>::with_capacity(32);
    for i in 0..32 {
        bin_buf.push(i);
    }
    let mut asc_buf = Vec::<u8>::with_capacity(32);
    asc_buf.extend_from_slice(&test_utils::_CMAP32);
    (ags, bin_buf, asc_buf)
}

#[test]
fn test_ags_binary_to_ascii_32_avx2_1() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (ags, bin_buf, asc_buf) = gen_data_ags_32();
    let mut buf = bin_buf.clone();
    let r = unsafe { _binary_to_ascii_32_avx2(&ags.binmap, &mut buf) };
    assert!(r.is_ok());
    assert_eq!(buf, asc_buf);
}

#[test]
fn test_ags_binary_to_ascii_32_avx2_2() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (ags, bin_buf0, asc_buf0) = gen_data_ags_32();
    let mut buf = Vec::<u8>::with_capacity(4 * 32);
    buf.extend_from_slice(&bin_buf0);
    buf.extend_from_slice(&bin_buf0);
    buf.extend_from_slice(&bin_buf0);
    buf.extend_from_slice(&bin_buf0);
    let r = unsafe { _binary_to_ascii_32_avx2(&ags.binmap, &mut buf) };
    assert!(r.is_ok());
    let mut cor = Vec::<u8>::with_capacity(4 * 32);
    cor.extend_from_slice(&asc_buf0);
    cor.extend_from_slice(&asc_buf0);
    cor.extend_from_slice(&asc_buf0);
    cor.extend_from_slice(&asc_buf0);
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_binary_to_ascii_32_avx2_3() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (ags, bin_buf0, _asc_buf0) = gen_data_ags_32();
    for i in 0..32 {
        let mut buf = bin_buf0.clone();
        buf.insert(i, 32);
        let r = unsafe { _binary_to_ascii_32_avx2(&ags.binmap, &mut buf) };
        assert_eq!(r, Err(EncodeError::InvalidIndex(32)));
    }
    for i in 0..32 {
        let mut buf = bin_buf0.clone();
        buf.insert(i, 128);
        let r = unsafe { _binary_to_ascii_32_avx2(&ags.binmap, &mut buf) };
        assert_eq!(r, Err(EncodeError::InvalidIndex(128)));
    }
    for i in 0..32 {
        let mut buf = bin_buf0.clone();
        buf.insert(i, 255);
        let r = unsafe { _binary_to_ascii_32_avx2(&ags.binmap, &mut buf) };
        assert_eq!(r, Err(EncodeError::InvalidIndex(255)));
    }
}
