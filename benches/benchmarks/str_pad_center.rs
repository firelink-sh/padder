use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn str_pad_10_center(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&str.pad 10 'l' center", |b| {
        b.iter(|| {
            black_box("a".pad(width, Alignment::Center, 'l'));
        });
    });
}

pub fn str_pad_100_center(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&str.pad 100 '💀' center", |b| {
        b.iter(|| {
            black_box("babage".pad(width, Alignment::Center, '💀'));
        });
    });
}

pub fn str_pad_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&str.pad 1000 '@' center", |b| {
        b.iter(|| {
            black_box("solaire is awesome".pad(width, Alignment::Center, '@'));
        });
    });
}

pub fn str_pad_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&str.pad 10000 '드' center", |b| {
        b.iter(|| {
            black_box("don't you dare go hollow..!!#".pad(width, Alignment::Center, '드'));
        });
    });
}

pub fn str_pad_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&str.pad 25000 '»' center", |b| {
        b.iter(|| {
            black_box("東風 ぬが ㅀㆈ".pad(width, Alignment::Center, '»'));
        });
    });
}

pub fn str_pad_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&str.pad 50000 'ö' center", |b| {
        b.iter(|| {
            black_box("plant needs water".pad(width, Alignment::Center, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    str_pad_10_center,
    str_pad_100_center,
    str_pad_1000_center,
    str_pad_10000_center,
    str_pad_25000_center,
    str_pad_50000_center,
);
