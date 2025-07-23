use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_vec_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    let mut source: Vec<char> = Vec::from(&['a']);
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 10 'l' left", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_vec_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    let mut source: Vec<char> = "babage".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 100 '💀' left", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn mut_vec_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source: Vec<char> = "solaire is awesome".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 1000 '@' left", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn mut_vec_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source: Vec<char> = "don't you dare go hollow..!!#".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 10_000 '드' left", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn mut_vec_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source: Vec<char> = "東風 ぬが ㅀㆈ".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 25_000 '»' left", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn mut_vec_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source: Vec<char> = "plant needs water".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 50_000 'ö' left", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Left, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    mut_vec_pad_10_left,
    mut_vec_pad_100_left,
    mut_vec_pad_1000_left,
    mut_vec_pad_10000_left,
    mut_vec_pad_25000_left,
    mut_vec_pad_50000_left,
);
