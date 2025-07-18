use criterion::{criterion_group, Criterion};
use padder::*;

use std::hint::black_box;

pub fn str_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&str.pad 10 la", |b| {
        b.iter(|| black_box("🤠".pad(width, Alignment::Left, 'µ')))
    });
    c.bench_function("format! 10 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "🤠", width = width)))
    });
}

pub fn str_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&str.pad 100 la", |b| {
        b.iter(|| black_box("praise the sun!".pad(width, Alignment::Left, 'x')))
    });
    c.bench_function("format! 100 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "praise the sun!", width = width)))
    });

}

pub fn str_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&str.pad 1000 la", |b| {
        b.iter(|| black_box("solaire is awesome".pad(width, Alignment::Left, '!')))
    });
    c.bench_function("format! 1000 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "solaire is awesome", width = width)))
    });

}

pub fn str_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&str.pad 10000 la", |b| {
        b.iter(|| black_box("don't you dare go hollow..!!#\"".pad(width, Alignment::Left, 'ø')))
    });
    c.bench_function("format! 10000 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "don't you dare go hollow..!!#\"", width = width)))
    });

}

pub fn str_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&str.pad 25000 la", |b| {
        b.iter(|| black_box("東風 ぬが ㅀㆈ".pad(width, Alignment::Left, 'µ')))
    });
    c.bench_function("format! 25000 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "東風 ぬが ㅀㆈ", width = width)))
    });
}


pub fn str_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&str.pad 50000 la", |b| {
        b.iter(|| black_box("東風 ぬが ㅀㆈ".pad(width, Alignment::Left, 'µ')))
    });
    c.bench_function("format! 50000 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "東風 ぬが ㅀㆈ", width = width)))
    });
}

criterion_group!(
    pads,
    str_pad_10_left,
    str_pad_100_left,
    str_pad_1000_left,
    str_pad_10000_left,
    str_pad_25000_left,
    str_pad_50000_left,
);
