use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rand::thread_rng;
use rand::seq::SliceRandom;

use skewheap::SkewHeap;

fn fill_drain(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill then drain skewheap of size");
    let counts = [10, 50, 100, 500];

    for count in counts.iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(*count), &count, |b, &count| {
            let mut items: Vec<u32> = (0..*count).collect();
            items.shuffle(&mut thread_rng());

            b.iter(|| {
                let mut s = SkewHeap::new();

                for n in &items{
                    s.put(n);
                }

                while !s.is_empty() {
                    s.take();
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, fill_drain);
criterion_main!(benches);
