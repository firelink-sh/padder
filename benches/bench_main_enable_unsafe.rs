use criterion::criterion_main;
mod benchmarks_enable_unsafe;

criterion_main! {
    benchmarks_enable_unsafe::mut_string_pad_center::pads,
    benchmarks_enable_unsafe::mut_string_pad_left::pads,
    benchmarks_enable_unsafe::mut_string_pad_right::pads,
    benchmarks_enable_unsafe::mut_string_truncate_center::truncates,
    benchmarks_enable_unsafe::mut_string_truncate_left::truncates,
    benchmarks_enable_unsafe::mut_string_truncate_right::truncates,
}
