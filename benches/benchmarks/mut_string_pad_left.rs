use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut String.pad 10 la", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut "a".to_string()).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut String.pad 100 la", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut "a".to_string()).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut String.pad 1000 la", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut "a".to_string()).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("&mut String.pad 10000 la", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut "a".to_string()).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25000;
    c.bench_function("&mut String.pad 25000 la", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut "a".to_string()).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50000;
    c.bench_function("&mut String.pad 50000 la", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut "a".to_string()).pad(width, Alignment::Left, 'l'));
        });
    });
}

criterion_group!(
    pads,
    mut_string_pad_10_left,
    mut_string_pad_100_left,
    mut_string_pad_1000_left,
    mut_string_pad_10000_left,
    mut_string_pad_25000_left,
    mut_string_pad_50000_left,
);
