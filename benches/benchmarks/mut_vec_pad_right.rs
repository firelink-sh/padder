use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_vec_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    let mut source: Vec<char> = Vec::from(&['a']);
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 10 'l' right", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn mut_vec_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    let mut source: Vec<char> = "babage".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 100 '💀' right", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn mut_vec_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source: Vec<char> = "solaire is awesome".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 1000 '@' right", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn mut_vec_pad_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source: Vec<char> = "don't you dare go hollow..!!#".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 10_000 '드' right", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn mut_vec_pad_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source: Vec<char> = "東風 ぬが ㅀㆈ".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 25_000 '»' right", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn mut_vec_pad_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source: Vec<char> = "plant needs water".chars().collect();
    source.shrink_to_fit();
    c.bench_function("&mut Vec.pad 50_000 'ö' right", |b| {
        b.iter(|| {
            let mut v = black_box(source.clone());
            black_box((&mut v).pad(width, Alignment::Right, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    mut_vec_pad_10_right,
    mut_vec_pad_100_right,
    mut_vec_pad_1000_right,
    mut_vec_pad_10000_right,
    mut_vec_pad_25000_right,
    mut_vec_pad_50000_right,
);
