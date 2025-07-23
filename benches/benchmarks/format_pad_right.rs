use criterion::{Criterion, criterion_group};

use std::hint::black_box;

pub fn format_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("format! 10 'l' right", |b| {
        b.iter(|| black_box(format!("{:l>width$}", "a", width = width)))
    });
}

pub fn format_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("format! 100 '💀' right", |b| {
        b.iter(|| black_box(format!("{:💀>width$}", "babage", width = width)))
    });
}

pub fn format_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("format! 1000 '@' right", |b| {
        b.iter(|| black_box(format!("{:@>width$}", "solaire is awesome", width = width)))
    });
}

pub fn format_pad_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("format! 10_000 '드' right", |b| {
        b.iter(|| {
            black_box(format!(
                "{:드>width$}",
                "don't you dare go hollow..!!#\"",
                width = width
            ))
        })
    });
}

pub fn format_pad_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("format! 25_000 '»' right", |b| {
        b.iter(|| black_box(format!("{:»>width$}", "東風 ぬが ㅀㆈ", width = width)))
    });
}

pub fn format_pad_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("format! 50_000 'ö' right", |b| {
        b.iter(|| black_box(format!("{:ö>width$}", "plant needs water", width = width)))
    });
}

criterion_group!(
    pads,
    format_pad_10_right,
    format_pad_100_right,
    format_pad_1000_right,
    format_pad_10000_right,
    format_pad_25000_right,
    format_pad_50000_right,
);
