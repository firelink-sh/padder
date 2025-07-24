use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn str_pad_to_buffer_10_center(c: &mut Criterion) {
    let width: usize = 10;
    let symbol: char = 'l';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let source = "a";
    c.bench_function("&str.pad_to_buffer 10 'l' center", |b| {
        b.iter(|| {
            let mut b = black_box(buffer.clone());
            black_box(source.pad_to_buffer(width, Alignment::Center, symbol, &mut b));
        });
    });
}

pub fn str_pad_to_buffer_100_center(c: &mut Criterion) {
    let width: usize = 100;
    let symbol: char = '💀';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let source = "babage";
    c.bench_function("&str.pad_to_buffer 100 '💀' center", |b| {
        b.iter(|| {
            let mut b = black_box(buffer.clone());
            black_box(source.pad_to_buffer(width, Alignment::Center, symbol, &mut b));
        });
    });
}

pub fn str_pad_to_buffer_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    let symbol: char = '@';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let source = "solaire is awesome";
    c.bench_function("&str.pad_to_buffer 1000 '@' center", |b| {
        b.iter(|| {
            let mut b = black_box(buffer.clone());
            black_box(source.pad_to_buffer(width, Alignment::Center, symbol, &mut b));
        });
    });
}

pub fn str_pad_to_buffer_10000_center(c: &mut Criterion) {
    let width: usize = 10_000;
    let symbol: char = '드';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let source = "don't you dare go hollow..!!#";
    c.bench_function("&str.pad_to_buffer 10_000 '드' center", |b| {
        b.iter(|| {
            let mut b = black_box(buffer.clone());
            black_box(source.pad_to_buffer(width, Alignment::Center, symbol, &mut b));
        });
    });
}

pub fn str_pad_to_buffer_25000_center(c: &mut Criterion) {
    let width: usize = 25_000;
    let symbol: char = '»';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let source = "東風 ぬが ㅀㆈ";
    c.bench_function("&str.pad_to_buffer 25_000 '»' center", |b| {
        b.iter(|| {
            let mut b = black_box(buffer.clone());
            black_box(source.pad_to_buffer(width, Alignment::Center, symbol, &mut b));
        });
    });
}

pub fn str_pad_to_buffer_50000_center(c: &mut Criterion) {
    let width: usize = 50_000;
    let symbol: char = 'ö';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let source = "plant needs water";
    c.bench_function("&str.pad_to_buffer 50_000 'ö' center", |b| {
        b.iter(|| {
            let mut b = black_box(buffer.clone());
            black_box(source.pad_to_buffer(width, Alignment::Center, symbol, &mut b));
        });
    });
}

criterion_group!(
    pads,
    str_pad_to_buffer_10_center,
    str_pad_to_buffer_100_center,
    str_pad_to_buffer_1000_center,
    str_pad_to_buffer_10000_center,
    str_pad_to_buffer_25000_center,
    str_pad_to_buffer_50000_center,
);
