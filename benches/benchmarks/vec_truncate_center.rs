use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn vec_truncate_10_center(c: &mut Criterion) {
    let width: usize = 10;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 100).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 100 to 10 'l' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, 'l'));
        });
    });
}

pub fn vec_truncate_100_center(c: &mut Criterion) {
    let width: usize = 100;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 1000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 1000 to 100 '💀' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '💀'));
        });
    });
}

pub fn vec_truncate_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 10_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 10_000 to 1000 '@' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '@'));
        });
    });
}

pub fn vec_truncate_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 20_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 20_000 to 10_000 '드' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '드'));
        });
    });
}

pub fn vec_truncate_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 50_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 50_000 to 25_000 '»' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '»'));
        });
    });
}

pub fn vec_truncate_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 100_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 100_000 to 50_000 'ö' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, 'ö'));
        });
    });
}

criterion_group!(
    truncates,
    vec_truncate_10_center,
    vec_truncate_100_center,
    vec_truncate_1000_center,
    vec_truncate_10000_center,
    vec_truncate_25000_center,
    vec_truncate_50000_center,
);
