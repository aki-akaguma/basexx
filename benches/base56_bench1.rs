use basexx::*;
use criterion::*;

mod utils;

fn bench_base56_enc(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base56 = Base56::default();
    c.bench_function("base56_enc", |b| b.iter(|| base56.encode(black_box(&v))));
}

fn bench_base56_dec(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base56 = Base56::default();
    let a = base56.encode(black_box(&v)).unwrap();
    c.bench_function("base56_dec", |b| b.iter(|| base56.decode(black_box(&a))));
}

criterion_group!(benches, bench_base56_enc, bench_base56_dec);
criterion_main!(benches);
