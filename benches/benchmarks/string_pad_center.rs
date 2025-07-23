use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn string_pad_10_center(c: &mut Criterion) {
    let width: usize = 10;
    let mut source = String::from("a");
    source.shrink_to_fit();
    c.bench_function("String.pad 10 'l' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, 'l'));
        });
    });
}

pub fn string_pad_100_center(c: &mut Criterion) {
    let width: usize = 100;
    let mut source = String::from("babage");
    source.shrink_to_fit();
    c.bench_function("String.pad 100 '💀' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '💀'));
        });
    });
}

pub fn string_pad_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source = String::from("solaire is awesome");
    source.shrink_to_fit();
    c.bench_function("String.pad 1000 '@' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '@'));
        });
    });
}

pub fn string_pad_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source = String::from("don't you dare go hollow..!!#");
    source.shrink_to_fit();
    c.bench_function("String.pad 10_000 '드' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '드'));
        });
    });
}

pub fn string_pad_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source = String::from("東風 ぬが ㅀㆈ");
    source.shrink_to_fit();
    c.bench_function("String.pad 25_000 '»' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, '»'));
        });
    });
}

pub fn string_pad_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source = String::from("plant needs water");
    source.shrink_to_fit();
    c.bench_function("String.pad 50_000 'ö' center", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            black_box(s.pad(width, Alignment::Center, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    string_pad_10_center,
    string_pad_100_center,
    string_pad_1000_center,
    string_pad_10000_center,
    string_pad_25000_center,
    string_pad_50000_center,
);
