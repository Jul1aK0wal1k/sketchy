use criterion::{criterion_group, criterion_main, Criterion};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
// use sketchy::CountMinSketch;

fn cms_add(c: &mut Criterion) {
    // let mut cms = CountMinSketch::new(10000, 10000);
    // let mut rng = rand::thread_rng();
    // let key: String = (0..100).map(|_| rng.sample(Alphanumeric) as char).collect();
    // c.bench_function("count min sketch add", |b| b.iter(|| cms.add(&key, 1)));
}

fn cms_estimate(c: &mut Criterion) {
    // let mut cms = CountMinSketch::new(10000, 10000);
    // let key: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 100);
    // c.bench_function("count min sketch estimate", |b| b.iter(|| cms.add(&key, 1)));
}

criterion_group!(benches, cms_add, cms_estimate);
criterion_main!(benches);
