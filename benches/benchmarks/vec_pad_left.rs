use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn vec_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    let symbol: char = 'l';
    let mut source: Vec<char> = "a".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad 10 'l' left", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            black_box(v.pad(width, Alignment::Left, symbol));
        });
    });
}

pub fn vec_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    let symbol: char = '💀';
    let mut source: Vec<char> = "babage".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad 100 '💀' left", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            black_box(v.pad(width, Alignment::Left, symbol));
        });
    });
}

pub fn vec_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    let symbol: char = '@';
    let mut source: Vec<char> = "solaire is not awesome!".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad 1000 '@' left", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            black_box(v.pad(width, Alignment::Left, symbol));
        });
    });
}

pub fn vec_pad_10_000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    let symbol: char = '드';
    let mut source: Vec<char> = "i hope you go hollow!...".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad 10_000 '드' left", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            black_box(v.pad(width, Alignment::Left, symbol));
        });
    });
}

pub fn vec_pad_25_000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    let symbol: char = '»';
    let mut source: Vec<char> = "東風 ぬが ㅀㆈ".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad 25_000 '»' left", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            black_box(v.pad(width, Alignment::Left, symbol));
        });
    });
}

pub fn vec_pad_50_000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    let symbol: char = 'ö';
    let mut source: Vec<char> = "plant doesn't need water!. :D".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad 50_000 'ö' left", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            black_box(v.pad(width, Alignment::Left, symbol));
        });
    });
}

criterion_group!(
    pads,
    vec_pad_10_left,
    vec_pad_100_left,
    vec_pad_1000_left,
    vec_pad_10_000_left,
    vec_pad_25_000_left,
    vec_pad_50_000_left,
);
