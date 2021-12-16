use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rand::thread_rng;
use rand::seq::SliceRandom;

use skewheap::SkewHeap;

fn make_filled_skewheap(size: usize) -> SkewHeap<u32> {
    let mut s = SkewHeap::new();
    for n in 0..size {
        s.put(n as u32);
    }

    s
}

fn put(c: &mut Criterion) {
    let mut group = c.benchmark_group("put into skewheap of size");
    let sizes = [0, 10, 50, 100, 500, 1000];

    for size in sizes {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut s = make_filled_skewheap(size);
            b.iter(|| s.put(42 as u32))
        });
    }

    group.finish();
}

fn take(c: &mut Criterion) {
    let mut group = c.benchmark_group("take from skewheap of size");
    let sizes = [0, 10, 50, 100, 500, 1000];

    for size in sizes {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut s = make_filled_skewheap(size);
            b.iter(|| s.take())
        });
    }

    group.finish();
}

fn fill_drain(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill then drain skewheap of size");
    let counts = [10, 50, 100, 500];

    for count in counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let mut items: Vec<u32> = (0..count).collect();
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

criterion_group!(benches, put, take, fill_drain);
criterion_main!(benches);
