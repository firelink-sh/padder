use criterion::{Criterion, criterion_group};
use padder::*;

use std::hint::black_box;

pub fn string_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("String.pad 10 la", |b| {
        b.iter(|| black_box("🤠".to_string().pad(width, Alignment::Left, 'µ')))
    });
}

pub fn string_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("String.pad 100 la", |b| {
        b.iter(|| {
            black_box(
                "praise the sun!"
                    .to_string()
                    .pad(width, Alignment::Left, 'x'),
            )
        })
    });
}

pub fn string_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("String.pad 1000 la", |b| {
        b.iter(|| {
            black_box(
                "solaire is awesome"
                    .to_string()
                    .pad(width, Alignment::Left, '!'),
            )
        })
    });
}

pub fn string_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("String.pad 10000 la", |b| {
        b.iter(|| {
            black_box("don't you dare go hollow..!!#\"".to_string().pad(
                width,
                Alignment::Left,
                'ø',
            ))
        })
    });
}

pub fn string_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("String.pad 25000 la", |b| {
        b.iter(|| {
            black_box(
                "東風 ぬが ㅀㆈ"
                    .to_string()
                    .pad(width, Alignment::Left, 'µ'),
            )
        })
    });
}

pub fn string_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("String.pad 50000 la", |b| {
        b.iter(|| {
            black_box(
                "東風 ぬが ㅀㆈ"
                    .to_string()
                    .pad(width, Alignment::Left, 'µ'),
            )
        })
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
