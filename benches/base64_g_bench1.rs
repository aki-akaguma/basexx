use basexx::*;
use criterion::*;

mod utils;

fn bench_base64_enc(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base64 = Base64G::default();
    c.bench_function("base64_g_enc", |b| b.iter(|| base64.encode(black_box(&v))));
}

fn bench_base64_dec(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base64 = Base64G::default();
    let a = base64.encode(black_box(&v)).unwrap();
    c.bench_function("base64_g_dec", |b| b.iter(|| base64.decode(black_box(&a))));
}

criterion_group!(benches, bench_base64_enc, bench_base64_dec);
criterion_main!(benches);
