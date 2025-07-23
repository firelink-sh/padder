use criterion::{Criterion, criterion_group};

use std::hint::black_box;

pub fn format_pad_10_center(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("format! 10 ' ' center", |b| {
        b.iter(|| black_box(format!("{:^width$}", "a", width = width)))
    });
}

pub fn format_pad_100_center(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("format! 100 ' ' center", |b| {
        b.iter(|| black_box(format!("{:^width$}", "babage", width = width)))
    });
}

pub fn format_pad_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("format! 1000 ' ' center", |b| {
        b.iter(|| black_box(format!("{:^width$}", "solaire is awesome", width = width)))
    });
}

pub fn format_pad_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("format! 10000 ' ' center", |b| {
        b.iter(|| {
            black_box(format!(
                "{:^width$}",
                "don't you dare go hollow..!!#\"",
                width = width
            ))
        })
    });
}

pub fn format_pad_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("format! 25000 ' ' center", |b| {
        b.iter(|| black_box(format!("{:»<width$}", "東風 ぬが ㅀㆈ", width = width)))
    });
}

pub fn format_pad_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("format! 50000 ' ' center", |b| {
        b.iter(|| black_box(format!("{:ö<width$}", "plant needs water", width = width)))
    });
}

criterion_group!(
    pads,
    format_pad_10_center,
    format_pad_100_center,
    format_pad_1000_center,
    format_pad_10000_center,
    format_pad_25000_center,
    format_pad_50000_center,
);
