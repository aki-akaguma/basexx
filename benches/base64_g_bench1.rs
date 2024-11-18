use basexx::*;
use criterion::*;

mod utils;

fn bench_base64_1(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base64 = Base64G::default();
    c.bench_function("base64_g_1", |b| b.iter(|| base64.encode(black_box(&v))));
}

fn bench_base64_2(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base64 = Base64G::default();
    let a = base64.encode(black_box(&v)).unwrap();
    c.bench_function("base64_g_2", |b| b.iter(|| base64.decode(black_box(&a))));
}

criterion_group!(benches, bench_base64_1, bench_base64_2);
criterion_main!(benches);
