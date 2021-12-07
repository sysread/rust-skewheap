use rand::thread_rng;
use rand::seq::SliceRandom;

use skewheap::SkewHeap;

pub fn main() {
    let count = 10_000;
    let mut vec: Vec<u32> = (0..count).collect();
    vec.shuffle(&mut thread_rng());
    println!("count: {}", count);

    let mut s = SkewHeap::new();

    let put_now = std::time::Instant::now();
    for n in &vec {
        s.put(n);
    }
    let put_done = put_now.elapsed().as_millis();
    println!(" put: {} ms ({}/call)", put_done, (put_done as f32)/(count as f32));

    let take_now = std::time::Instant::now();
    while !s.is_empty() {
        s.take();
    }
    let take_done = take_now.elapsed().as_millis();
    println!("take: {} ms ({}/call)", take_done, (take_done as f32)/(count as f32));
}
