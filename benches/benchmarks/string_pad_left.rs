use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn string_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    let mut source = String::from("a");
    source.shrink_to_fit();
    c.bench_function("String.pad 10 'l' left", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn string_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    let mut source = String::from("babage");
    source.shrink_to_fit();
    c.bench_function("String.pad 100 '💀' left", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn string_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source = String::from("solaire is awesome");
    source.shrink_to_fit();
    c.bench_function("String.pad 1000 '@' left", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn string_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source = String::from("don't you dare go hollow..!!#");
    source.shrink_to_fit();
    c.bench_function("String.pad 10_000 '드' left", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn string_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source = String::from("東風 ぬが ㅀㆈ");
    source.shrink_to_fit();
    c.bench_function("String.pad 25_000 '»' left", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn string_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source = String::from("plant needs water");
    source.shrink_to_fit();
    c.bench_function("String.pad 50_000 'ö' left", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Left, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    string_pad_10_left,
    string_pad_100_left,
    string_pad_1000_left,
    string_pad_10000_left,
    string_pad_25000_left,
    string_pad_50000_left,
);
