use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn slice_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    let symbol: char = 'l';
    let v: &[char] = &['a'];
    c.bench_function("&[T].pad 10 'l' right", |b| {
        b.iter(|| {
            black_box(v.pad(width, Alignment::Right, symbol));
        });
    });
}

pub fn slice_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    let symbol: char = '💀';
    let v: &[char] = &['b', 'a', 'b', 'a', 'g', 'e'];
    c.bench_function("&[T].pad 100 '💀' right", |b| {
        b.iter(|| {
            black_box(v.pad(width, Alignment::Right, symbol));
        });
    });
}

pub fn slice_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    let symbol: char = '@';
    let v: &[char] = &['s', 'o', 'l', 'a', 'i', 'r', 'e'];
    c.bench_function("&[T].pad 1000 '@' right", |b| {
        b.iter(|| {
            black_box(v.pad(width, Alignment::Right, symbol));
        });
    });
}

pub fn slice_pad_10_000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    let symbol: char = '드';
    let v: &[char] = &['h', 'o', 'l', 'l', 'o', 'w'];
    c.bench_function("&[T].pad 10_000 '드' right", |b| {
        b.iter(|| {
            black_box(v.pad(width, Alignment::Right, symbol));
        });
    });
}

pub fn slice_pad_25_000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    let symbol: char = '»';
    let v: &[char] = &['東', '風', ' ', 'ぬ', 'が', ' ', 'ㅀ', 'ㆈ'];
    c.bench_function("&[T].pad 25_000 '»' right", |b| {
        b.iter(|| {
            black_box(v.pad(width, Alignment::Right, symbol));
        });
    });
}

pub fn slice_pad_50_000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    let symbol: char = 'ö';
    let v: &[char] = &['©', 'p', 'l', 'a', 'n', 't', '„'];
    c.bench_function("&[T].pad 50_000 'ö' right", |b| {
        b.iter(|| {
            black_box(v.pad(width, Alignment::Right, symbol));
        });
    });
}

criterion_group!(
    pads,
    slice_pad_10_right,
    slice_pad_100_right,
    slice_pad_1000_right,
    slice_pad_10_000_right,
    slice_pad_25_000_right,
    slice_pad_50_000_right,
);
