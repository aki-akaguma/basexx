use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_ags_128_ssse3_dec(c: &mut Criterion) {
    if !is_x86_feature_detected!("ssse3") {
        return;
    }
    let v = test_utils::read_t4_base64_ascii().as_bytes().to_vec();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    c.bench_function("ags_128_ssse3_dec", |b| {
        b.iter(|| {
            let mut v = v.clone();
            let _ = unsafe { _ascii_to_binary_128_ssse3(&ags.a128map, &mut v) };
        })
    });
}