use criterion::{black_box, criterion_group, criterion_main, Criterion};
use string_cache::DefaultAtom as Atom;

pub fn benches() {
    let mut criterion = (Criterion::default()).configure_from_args();
    let mut c = criterion.sample_size(10);
    criterion_benchmark(&mut c);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("100 elements", |b| {
        b.iter(|| {
            let mut atoms = Vec::new();
            for i in 0..100 {
                atoms.push(Atom::from(black_box(format!("test{}", i))));
            }
        })
    });

    c.bench_function("10000 elements", |b| {
        b.iter(|| {
            let mut atoms = Vec::new();
            for i in 0..10_000 {
                atoms.push(Atom::from(black_box(format!("test{}", i))));
            }
        })
    });

    c.bench_function("100K elements", |b| {
        b.iter(|| {
            let mut atoms = Vec::new();
            for i in 0..100_000 {
                atoms.push(Atom::from(black_box(format!("test{}", i))));
            }
        })
    });

    c.bench_function("1M elements", |b| {
        b.iter(|| {
            let mut atoms = Vec::new();
            for i in 0..1_000_000 {
                atoms.push(Atom::from(black_box(format!("test{}", i))));
            }
        })
    });
}

criterion_main!(benches);
