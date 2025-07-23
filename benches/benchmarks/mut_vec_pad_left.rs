use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_vec_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut Vec.pad 10 'l' left", |b| {
        b.iter(|| {
            let mut v: Vec<char> = Vec::from(&['a']);
            black_box((&mut v).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_vec_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut Vec.pad 100 '💀' left", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "babage".chars().collect();
            black_box((&mut v).pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn mut_vec_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut Vec.pad 1000 '@' left", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "solaire is awesome".chars().collect();
            black_box((&mut v).pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn mut_vec_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&mut Vec.pad 10000 '드' left", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "don't you dare go hollow..!!#".chars().collect();
            black_box((&mut v).pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn mut_vec_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&mut Vec.pad 25000 '»' left", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "東風 ぬが ㅀㆈ".chars().collect();
            black_box((&mut v).pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn mut_vec_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&mut Vec.pad 50000 'ö' left", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "plant needs water".chars().collect();
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
