use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    let mut source = String::from("a");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 10 'l' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    let mut source = String::from("babage");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 100 '💀' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn mut_string_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source = String::from("solaire is awesome");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 1000 '@' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn mut_string_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source = String::from("don't you dare go hollow..!!#");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 10_000 '드' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn mut_string_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source = String::from("東風 ぬが ㅀㆈ");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 25_000 '»' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn mut_string_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source = String::from("plant needs water");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 50_000 'ö' left", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Left, 'ö'));
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
