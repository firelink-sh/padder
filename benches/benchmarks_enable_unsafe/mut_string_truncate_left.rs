use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_truncate_10_left(c: &mut Criterion) {
    let width: usize = 10;
    let mut source: String = std::iter::repeat_n("ø", 100).collect();
    source.shrink_to_fit();
    c.bench_function("&mut String.truncate 100 to 10 'l' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_truncate_100_left(c: &mut Criterion) {
    let width: usize = 100;
    let mut source: String = std::iter::repeat_n("ø", 1000).collect();
    source.shrink_to_fit();
    c.bench_function("&mut String.truncate 1000 to 100 '💀' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn mut_string_truncate_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source: String = std::iter::repeat_n("ø", 10_000).collect();
    source.shrink_to_fit();
    c.bench_function("&mut String.truncate 10_000 to 1000 '@' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn mut_string_truncate_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source: String = std::iter::repeat_n("ø", 20_000).collect();
    source.shrink_to_fit();
    c.bench_function("&mut String.truncate 20_000 to 10_000 '드' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn mut_string_truncate_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source: String = std::iter::repeat_n("ø", 50_000).collect();
    source.shrink_to_fit();
    c.bench_function("&mut String.truncate 50_000 to 25_000 '»' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn mut_string_truncate_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source: String = std::iter::repeat_n("ø", 100_000).collect();
    source.shrink_to_fit();
    c.bench_function("&mut String.truncate 100_000 to 50_000 'ö' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, 'ö'));
        });
    });
}

criterion_group!(
    truncates,
    mut_string_truncate_10_left,
    mut_string_truncate_100_left,
    mut_string_truncate_1000_left,
    mut_string_truncate_10000_left,
    mut_string_truncate_25000_left,
    mut_string_truncate_50000_left,
);
