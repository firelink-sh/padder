use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::format_pad_left::pads,
    benchmarks::mut_string_pad_left::pads,
    benchmarks::str_pad_left::pads,
    benchmarks::string_pad_left::pads,
}
