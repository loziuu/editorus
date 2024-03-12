use criterion::{criterion_group, criterion_main, Criterion};
use editorus::rope;

static LOREM: &'static str = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

fn append(c: &mut Criterion) {
    let mut group = c.benchmark_group("TRAVERSAL");


//    group.bench_function("Last index in small rope", |b| {
//        let mut rope = rope::Rope::new();
//        rope.append(LOREM);

//        b.iter(|| {
//            rope.do_nothing_at(rope.len());
//        })
//    });

//    group.bench_function("Last index in medium rope", |b| {
//        let mut rope = rope::Rope::new();

//        for _ in 0..500 {
//            rope.append(LOREM);
//        }

//        b.iter(|| {
//            rope.do_nothing_at(rope.len());
//        })
//    });

//    group.bench_function("Last index in large rope", |b| {
//        let mut rope = rope::Rope::new();

//        for _ in 0..2500 {
//            rope.append(LOREM);
//        }

//        b.iter(|| {
//            rope.do_nothing_at(rope.len());
//        })
//    });

}

criterion_group!(benches, append);
criterion_main!(benches);
