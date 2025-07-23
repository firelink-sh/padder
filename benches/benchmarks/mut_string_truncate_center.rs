use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_truncate_10_center(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut String.truncate 100 to 10 'l' center", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 100).collect();
            black_box((&mut s).pad(width, Alignment::Center, 'l'));
        });
    });
}

pub fn mut_string_truncate_100_center(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut String.truncate 1000 to 100 '💀' center", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 1000).collect();
            black_box((&mut s).pad(width, Alignment::Center, '💀'));
        });
    });
}

pub fn mut_string_truncate_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut String.truncate 10000 to 1000 '@' center", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 10_000).collect();
            black_box((&mut s).pad(width, Alignment::Center, '@'));
        });
    });
}

pub fn mut_string_truncate_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&mut String.truncate 20000 to 10000 '드' center", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 20_000).collect();
            black_box((&mut s).pad(width, Alignment::Center, '드'));
        });
    });
}

pub fn mut_string_truncate_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&mut String.truncate 50000 to 25000 '»' center", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 50_000).collect();
            black_box((&mut s).pad(width, Alignment::Center, '»'));
        });
    });
}

pub fn mut_string_truncate_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&mut String.truncate 100000 to 50000 'ö' center", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 100_000).collect();
            black_box((&mut s).pad(width, Alignment::Center, 'ö'));
        });
    });
}

criterion_group!(
    truncates,
    mut_string_truncate_10_center,
    mut_string_truncate_100_center,
    mut_string_truncate_1000_center,
    mut_string_truncate_10000_center,
    mut_string_truncate_25000_center,
    mut_string_truncate_50000_center,
);
