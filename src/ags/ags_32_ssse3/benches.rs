use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_ags_32_ssse3_enc(c: &mut Criterion) {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let mut v = test_utils::read_t4_base32_ascii().as_bytes().to_vec();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP32);
    let _ = ags.ascii_to_binary(&mut v);
    c.bench_function("ags_32_ssse3_enc", |b| {
        b.iter(|| {
            let mut v = v.clone();
            //let _ = ags.binary_to_ascii(&mut v);
            let _ = unsafe { _binary_to_ascii_32_ssse3(&ags.binmap, &mut v) };
        })
    });
}

#[allow(dead_code)]
pub fn bench_ags_32_ssse3_dec(c: &mut Criterion) {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let v = test_utils::read_t4_base32_ascii().as_bytes().to_vec();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP32);
    c.bench_function("ags_32_ssse3_dec", |b| {
        b.iter(|| {
            let mut v = v.clone();
            //let _ = ags.ascii_to_binary(&mut v);
            let _ = unsafe { _ascii_to_binary_32_ssse3(&ags.a128map, &mut v) };
        })
    });
}
