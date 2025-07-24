use criterion::{Criterion, criterion_group};
use padder::{Alignment, Source};

use std::hint::black_box;

pub fn string_pad_to_buffer_10_right(c: &mut Criterion) {
    let width: usize = 10;
    let symbol: char = 'l';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let mut source = String::from("a");
    source.shrink_to_fit();
    c.bench_function("String.pad_to_buffer 10 'l' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(s.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn string_pad_to_buffer_100_right(c: &mut Criterion) {
    let width: usize = 100;
    let symbol: char = '💀';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let mut source = String::from("babage");
    source.shrink_to_fit();
    c.bench_function("String.pad_to_buffer 100 '💀' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(s.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn string_pad_to_buffer_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    let symbol: char = '@';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let mut source = String::from("solaire is awesome");
    source.shrink_to_fit();
    c.bench_function("String.pad_to_buffer 1000 '@' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(s.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn string_pad_to_buffer_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    let symbol: char = '드';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let mut source = String::from("don't you dare go hollow..!!#");
    source.shrink_to_fit();
    c.bench_function("String.pad_to_buffer 10_000 '드' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(s.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn string_pad_to_buffer_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    let symbol: char = '»';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let mut source = String::from("東風 ぬが ㅀㆈ");
    source.shrink_to_fit();
    c.bench_function("String.pad_to_buffer 25_000 '»' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(s.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

pub fn string_pad_to_buffer_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    let symbol: char = 'ö';
    let buffer = String::with_capacity(width * symbol.len_utf8());
    let mut source = String::from("plant needs water");
    source.shrink_to_fit();
    c.bench_function("String.pad_to_buffer 50_000 'ö' right", |b| {
        b.iter(|| {
            let s = black_box(source.clone());
            let mut b = black_box(buffer.clone());
            black_box(s.pad_to_buffer(width, Alignment::Right, symbol, &mut b));
        });
    });
}

criterion_group!(
    pads,
    string_pad_to_buffer_10_right,
    string_pad_to_buffer_100_right,
    string_pad_to_buffer_1000_right,
    string_pad_to_buffer_10000_right,
    string_pad_to_buffer_25000_right,
    string_pad_to_buffer_50000_right,
);
