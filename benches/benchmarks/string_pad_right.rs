use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn string_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("String.pad 10 'l' right", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box(s.pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn string_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("String.pad 100 '💀' right", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(6);
            s.push_str("babage");
            black_box(s.pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn string_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("String.pad 1000 '@' right", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(18);
            s.push_str("solaire is awesome");
            black_box(s.pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn string_pad_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("String.pad 10000 '드' right", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(31);
            s.push_str("don't you dare go hollow..!!#");
            black_box(s.pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn string_pad_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("String.pad 25000 '»' right", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(20);
            s.push_str("東風 ぬが ㅀㆈ");
            black_box(s.pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn string_pad_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("String.pad 50000 'ö' right", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(17);
            s.push_str("plant needs water");
            black_box(s.pad(width, Alignment::Right, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    string_pad_10_right,
    string_pad_100_right,
    string_pad_1000_right,
    string_pad_10000_right,
    string_pad_25000_right,
    string_pad_50000_right,
);
