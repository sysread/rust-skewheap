use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rand::thread_rng;
use rand::seq::SliceRandom;

use skewheap::SkewHeap;

fn shuffled_list_of_u32s(count: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = (0..count).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

fn put_all_take_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("put all, take all");
    let counts = [10, 50, 100, 500, 1000];

    for count in counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let items = shuffled_list_of_u32s(count);

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

criterion_group!(benches, put_all_take_all);
criterion_main!(benches);
