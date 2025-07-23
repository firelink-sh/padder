use criterion::{Criterion, criterion_group};

use std::hint::black_box;

pub fn format_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("format! 10 'l' left", |b| {
        b.iter(|| black_box(format!("{:l<width$}", "a", width = width)))
    });
}

pub fn format_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("format! 100 '💀' left", |b| {
        b.iter(|| black_box(format!("{:💀<width$}", "babage", width = width)))
    });
}

pub fn format_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("format! 1000 '@' left", |b| {
        b.iter(|| black_box(format!("{:@<width$}", "solaire is awesome", width = width)))
    });
}

pub fn format_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("format! 10_000 '드' left", |b| {
        b.iter(|| {
            black_box(format!(
                "{:드<width$}",
                "don't you dare go hollow..!!#\"",
                width = width
            ))
        })
    });
}

pub fn format_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("format! 25_000 '»' left", |b| {
        b.iter(|| black_box(format!("{:»<width$}", "東風 ぬが ㅀㆈ", width = width)))
    });
}

pub fn format_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("format! 50_000 'ö' left", |b| {
        b.iter(|| black_box(format!("{:ö<width$}", "plant needs water", width = width)))
    });
}

criterion_group!(
    pads,
    format_pad_10_left,
    format_pad_100_left,
    format_pad_1000_left,
    format_pad_10000_left,
    format_pad_25000_left,
    format_pad_50000_left,
);
