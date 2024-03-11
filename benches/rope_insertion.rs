use criterion::{criterion_group, criterion_main, Criterion};
use editorus::rope;

static LOREM: &'static str = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

fn append(c: &mut Criterion) {
    let mut group = c.benchmark_group("append");

    group.bench_function("Into empty rope", |b| {
        let mut rope = rope::Rope::new();
        b.iter(|| {
            rope.append(LOREM);
        })
    });

    group.bench_function("Into small rope", |b| {
        let mut rope = rope::Rope::new();
        rope.append(LOREM);

        b.iter(|| {
            rope.append("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ");
        })
    });

    group.bench_function("Into medium rope", |b| {
        let mut rope = rope::Rope::new();

        for _ in 0..500 {
            rope.append(LOREM);
        }


        b.iter(|| {
            rope.append(LOREM);
        })
    });

    group.bench_function("Into large rope", |b| {
        let mut rope = rope::Rope::new();

        for _ in 0..2500 {
            rope.append(LOREM);
        }


        b.iter(|| {
            rope.append(LOREM);
        })
    });

}

criterion_group!(benches, append);
criterion_main!(benches);
