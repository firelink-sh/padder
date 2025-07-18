use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::str_pad_left::pads,
}
