use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::mut_string_pad_center::pads,
    benchmarks::mut_string_pad_left::pads,
    benchmarks::mut_string_pad_right::pads,
    benchmarks::mut_string_truncate_center::truncates,
    benchmarks::mut_string_truncate_left::truncates,
    benchmarks::mut_string_truncate_right::truncates,
    benchmarks::mut_vec_pad_center::pads,
    benchmarks::mut_vec_pad_left::pads,
    benchmarks::mut_vec_pad_right::pads,
    benchmarks::mut_vec_truncate_center::truncates,
    benchmarks::mut_vec_truncate_left::truncates,
    benchmarks::mut_vec_truncate_right::truncates,
}
