use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_truncate_10_right(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut String.truncate 100 to 10 'l' right", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 100).collect();
            black_box((&mut s).pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn mut_string_truncate_100_right(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut String.truncate 1000 to 100 '💀' right", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 1000).collect();
            black_box((&mut s).pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn mut_string_truncate_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut String.truncate 10000 to 1000 '@' right", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 10_000).collect();
            black_box((&mut s).pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn mut_string_truncate_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&mut String.truncate 20000 to 10000 '드' right", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 20_000).collect();
            black_box((&mut s).pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn mut_string_truncate_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&mut String.truncate 50000 to 25000 '»' right", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 50_000).collect();
            black_box((&mut s).pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn mut_string_truncate_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&mut String.truncate 10000 to 50000 'ö' right", |b| {
        b.iter(|| {
            let mut s: String = std::iter::repeat_n("ø", 100_000).collect();
            black_box((&mut s).pad(width, Alignment::Right, 'ö'));
        });
    });
}

criterion_group!(
    truncates,
    mut_string_truncate_10_right,
    mut_string_truncate_100_right,
    mut_string_truncate_1000_right,
    mut_string_truncate_10000_right,
    mut_string_truncate_25000_right,
    mut_string_truncate_50000_right,
);
