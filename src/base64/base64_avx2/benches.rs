use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_base64_avx2_enc(c: &mut Criterion) {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    c.bench_function("base64_avx2_enc", |b| {
        b.iter(|| unsafe { _encode_base64_avx2(black_box(&ags), black_box(&v)) })
    });
}

#[allow(dead_code)]
pub fn bench_base64_avx2_dec(c: &mut Criterion) {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let a = unsafe { _encode_base64_avx2(black_box(&ags), black_box(&v)).unwrap() };
    c.bench_function("base64_avx2_dec", |b| {
        b.iter(|| unsafe { _decode_base64_avx2(black_box(&ags), black_box(&a)) })
    });
}
