use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn vec_pad_to_buffer_10_right(c: &mut Criterion) {
    let width: usize = 10;
    let symbol: char = 'l';
    let buffer: Vec<char> = Vec::with_capacity(width);
    let mut source: Vec<char> = "a".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad_to_buffer 10 'l' right", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(v.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn vec_pad_to_buffer_100_right(c: &mut Criterion) {
    let width: usize = 100;
    let symbol: char = '💀';
    let buffer: Vec<char> = Vec::with_capacity(width);
    let mut source: Vec<char> = "babage".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad_to_buffer 100 '💀' right", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(v.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn vec_pad_to_buffer_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    let symbol: char = '@';
    let buffer: Vec<char> = Vec::with_capacity(width);
    let mut source: Vec<char> = "solaire is not awesome!".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad_to_buffer 1000 '@' right", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(v.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn vec_pad_to_buffer_10_000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    let symbol: char = '드';
    let buffer: Vec<char> = Vec::with_capacity(width);
    let mut source: Vec<char> = "i hope you go hollow!...".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad_to_buffer 10_000 '드' right", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(v.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn vec_pad_to_buffer_25_000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    let symbol: char = '»';
    let buffer: Vec<char> = Vec::with_capacity(width);
    let mut source: Vec<char> = "東風 ぬが ㅀㆈ".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad_to_buffer 25_000 '»' right", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(v.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn vec_pad_to_buffer_50_000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    let symbol: char = 'ö';
    let buffer: Vec<char> = Vec::with_capacity(width);
    let mut source: Vec<char> = "plant doesn't need water!. :D".chars().collect();
    source.shrink_to_fit();
    c.bench_function("Vec.pad_to_buffer 50_000 'ö' right", |b| {
        b.iter(|| {
            let v = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(v.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

criterion_group!(
    pads,
    vec_pad_to_buffer_10_right,
    vec_pad_to_buffer_100_right,
    vec_pad_to_buffer_1000_right,
    vec_pad_to_buffer_10_000_right,
    vec_pad_to_buffer_25_000_right,
    vec_pad_to_buffer_50_000_right,
);
