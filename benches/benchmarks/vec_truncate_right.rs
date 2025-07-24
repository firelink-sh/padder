use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn vec_truncate_10_right(c: &mut Criterion) {
    let width: usize = 10;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 100).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 100 to 10 'l' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn vec_truncate_100_right(c: &mut Criterion) {
    let width: usize = 100;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 1000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 1000 to 100 '💀' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn vec_truncate_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 10_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 10_000 to 1000 '@' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn vec_truncate_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 20_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 20_000 to 10_000 '드' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn vec_truncate_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 50_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 50_000 to 25_000 '»' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn vec_truncate_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source: Vec<char> = std::iter::repeat_n('ł', 100_000).collect();
    source.shrink_to_fit();
    c.bench_function("Vec.truncate 100_000 to 50_000 'ö' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Right, 'ö'));
        });
    });
}

criterion_group!(
    truncates,
    vec_truncate_10_right,
    vec_truncate_100_right,
    vec_truncate_1000_right,
    vec_truncate_10000_right,
    vec_truncate_25000_right,
    vec_truncate_50000_right,
);
