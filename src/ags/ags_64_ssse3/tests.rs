#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;
//use rstest::rstest;
//use rstest_reuse::{self, *};

#[test]
fn test_ags_binary_to_ascii_64_ssse3_1() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf.push(i);
    }
    let r = unsafe { _binary_to_ascii_64_ssse3(&ags.binmap, &mut buf) };
    assert!(r.is_ok());
    assert_eq!(buf, &test_utils::_CMAP64);
}

#[test]
fn test_ags_binary_to_ascii_64_ssse3_2() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf0 = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf0.push(i);
    }
    let mut buf = Vec::<u8>::with_capacity(4 * 64);
    buf.extend_from_slice(&buf0);
    buf.extend_from_slice(&buf0);
    buf.extend_from_slice(&buf0);
    buf.extend_from_slice(&buf0);
    let r = unsafe { _binary_to_ascii_64_ssse3(&ags.binmap, &mut buf) };
    assert!(r.is_ok());
    let mut cor = Vec::<u8>::with_capacity(4 * 64);
    cor.extend_from_slice(&test_utils::_CMAP64);
    cor.extend_from_slice(&test_utils::_CMAP64);
    cor.extend_from_slice(&test_utils::_CMAP64);
    cor.extend_from_slice(&test_utils::_CMAP64);
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_binary_to_ascii_64_ssse3_3() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf0 = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf0.push(i);
    }
    for i in 0..64 {
        let mut buf = buf0.clone();
        buf.insert(i, 64);
        let r = unsafe { _binary_to_ascii_64_ssse3(&ags.binmap, &mut buf) };
        assert_eq!(r, Err(EncodeError::InvalidIndex(64)));
    }
    for i in 0..64 {
        let mut buf = buf0.clone();
        buf.insert(i, 128);
        let r = unsafe { _binary_to_ascii_64_ssse3(&ags.binmap, &mut buf) };
        assert_eq!(r, Err(EncodeError::InvalidIndex(128)));
    }
    for i in 0..64 {
        let mut buf = buf0.clone();
        buf.insert(i, 255);
        let r = unsafe { _binary_to_ascii_64_ssse3(&ags.binmap, &mut buf) };
        assert_eq!(r, Err(EncodeError::InvalidIndex(255)));
    }
}

#[test]
fn test_ags_ascii_to_binary_64_ssse3_1() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf = Vec::<u8>::with_capacity(4 * 64);
    buf.extend_from_slice(&test_utils::_CMAP64);
    buf.extend_from_slice(&test_utils::_CMAP64);
    buf.extend_from_slice(&test_utils::_CMAP64);
    buf.extend_from_slice(&test_utils::_CMAP64);
    let r = unsafe { _ascii_to_binary_64_ssse3(&ags.a128map, &mut buf) };
    assert!(r.is_ok());
    let mut cor0 = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        cor0.push(i);
    }
    let mut cor = Vec::<u8>::with_capacity(4 * 64);
    cor.extend_from_slice(&cor0);
    cor.extend_from_slice(&cor0);
    cor.extend_from_slice(&cor0);
    cor.extend_from_slice(&cor0);
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_ascii_to_binary_64_ssse3_2() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf = Vec::<u8>::with_capacity(64);
    buf.extend_from_slice(&test_utils::_CMAP64);
    let r = unsafe { _ascii_to_binary_64_ssse3(&ags.a128map, &mut buf) };
    assert!(r.is_ok());
    let mut cor = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        cor.push(i);
    }
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_ascii_to_binary_64_ssse3_3() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf0 = Vec::<u8>::with_capacity(64);
    buf0.extend_from_slice(&test_utils::_CMAP64);
    for i in 0..64 {
        let mut buf = buf0.clone();
        buf.insert(i, 1);
        let r = unsafe { _ascii_to_binary_64_ssse3(&ags.a128map, &mut buf) };
        assert_eq!(r, Err(DecodeError::InvalidByte(1)));
    }
    for i in 0..64 {
        let mut buf = buf0.clone();
        buf.insert(i, b'.');
        let r = unsafe { _ascii_to_binary_64_ssse3(&ags.a128map, &mut buf) };
        assert_eq!(r, Err(DecodeError::InvalidByte(46)));
    }
}
