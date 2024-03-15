use criterion::{criterion_group, criterion_main, Criterion};
use editorus::rope::{self, rope::Rope};

static LOREM: &'static str = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

fn clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("CLONE");

    group.bench_function("Into empty rope", |b| {
        let rope = Rope::new();
        b.iter(|| {
            let _ = rope.clone();
        })
    });

    group.bench_function("Small rope", |b| {
        let mut rope = Rope::new();
        rope.append(LOREM);

        b.iter(|| {
            let _ = rope.clone();
        })
    });

    group.bench_function("Medium rope", |b| {
        let mut rope = Rope::new();

        for _ in 0..500 {
            rope.append(LOREM);
        }


        b.iter(|| {
            let _ = rope.clone();
        })
    });

    group.bench_function("Large rope", |b| {
        let mut rope = Rope::new();

        for _ in 0..2500 {
            rope.append(LOREM);
        }


        b.iter(|| {
            let _ = rope.clone();
        })
    });

}

criterion_group!(benches, clone);
criterion_main!(benches);
