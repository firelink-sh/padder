use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn str_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&str.pad 10 'l' right", |b| {
        b.iter(|| {
            black_box("a".pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn str_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&str.pad 100 '💀' right", |b| {
        b.iter(|| {
            black_box("babage".pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn str_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&str.pad 1000 '@' right", |b| {
        b.iter(|| {
            black_box("solaire is awesome".pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn str_pad_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&str.pad 10_000 '드' right", |b| {
        b.iter(|| {
            black_box("don't you dare go hollow..!!#".pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn str_pad_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&str.pad 25_000 '»' right", |b| {
        b.iter(|| {
            black_box("東風 ぬが ㅀㆈ".pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn str_pad_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&str.pad 50_000 'ö' right", |b| {
        b.iter(|| {
            black_box("plant needs water".pad(width, Alignment::Right, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    str_pad_10_right,
    str_pad_100_right,
    str_pad_1000_right,
    str_pad_10000_right,
    str_pad_25000_right,
    str_pad_50000_right,
);
