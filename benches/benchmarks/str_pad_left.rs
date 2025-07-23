use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn str_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&str.pad 10 'l' left", |b| {
        b.iter(|| {
            black_box("a".pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn str_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&str.pad 100 '💀' left", |b| {
        b.iter(|| {
            black_box("babage".pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn str_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&str.pad 1000 '@' left", |b| {
        b.iter(|| {
            black_box("solaire is awesome".pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn str_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&str.pad 10_000 '드' left", |b| {
        b.iter(|| {
            black_box("don't you dare go hollow..!!#".pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn str_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&str.pad 25_000 '»' left", |b| {
        b.iter(|| {
            black_box("東風 ぬが ㅀㆈ".pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn str_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&str.pad 50_000 'ö' left", |b| {
        b.iter(|| {
            black_box("plant needs water".pad(width, Alignment::Left, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    str_pad_10_left,
    str_pad_100_left,
    str_pad_1000_left,
    str_pad_10000_left,
    str_pad_25000_left,
    str_pad_50000_left,
);
