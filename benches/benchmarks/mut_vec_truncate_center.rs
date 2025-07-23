use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_vec_truncate_10_center(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut Vec.truncate 100 to 10 'l' center", |b| {
        b.iter(|| {
            let mut v: Vec<char> = std::iter::repeat_n('a', 100).collect();
            black_box((&mut v).pad(width, Alignment::Center, 'l'));
        });
    });
}

pub fn mut_vec_truncate_100_center(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut Vec.truncate 1000 to 100 '💀' center", |b| {
        b.iter(|| {
            let mut v: Vec<char> = std::iter::repeat_n('a', 1000).collect();
            black_box((&mut v).pad(width, Alignment::Center, '💀'));
        });
    });
}

pub fn mut_vec_truncate_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut Vec.truncate 10000 to 1000 '@' center", |b| {
        b.iter(|| {
            let mut v: Vec<char> = std::iter::repeat_n('a', 10_000).collect();
            black_box((&mut v).pad(width, Alignment::Center, '@'));
        });
    });
}

pub fn mut_vec_truncate_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&mut Vec.truncate 20000 to 10000 '드' center", |b| {
        b.iter(|| {
            let mut v: Vec<char> = std::iter::repeat_n('a', 20_000).collect();
            black_box((&mut v).pad(width, Alignment::Center, '드'));
        });
    });
}

pub fn mut_vec_truncate_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&mut Vec.truncate 50000 to 25000 '»' center", |b| {
        b.iter(|| {
            let mut v: Vec<char> = std::iter::repeat_n('a', 50_000).collect();
            black_box((&mut v).pad(width, Alignment::Center, '»'));
        });
    });
}

pub fn mut_vec_truncate_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&mut Vec.truncate 100000 to 50000 'ö' center", |b| {
        b.iter(|| {
            let mut v: Vec<char> = std::iter::repeat_n('a', 100_000).collect();
            black_box((&mut v).pad(width, Alignment::Center, 'ö'));
        });
    });
}

criterion_group!(
    truncates,
    mut_vec_truncate_10_center,
    mut_vec_truncate_100_center,
    mut_vec_truncate_1000_center,
    mut_vec_truncate_10000_center,
    mut_vec_truncate_25000_center,
    mut_vec_truncate_50000_center,
);
