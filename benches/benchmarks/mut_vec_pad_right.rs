use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_vec_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut Vec.pad 10 'l' right", |b| {
        b.iter(|| {
            let mut v: Vec<char> = Vec::from(&['a']);
            black_box((&mut v).pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn mut_vec_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut Vec.pad 100 '💀' right", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "babage".chars().collect();
            black_box((&mut v).pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn mut_vec_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut Vec.pad 1000 '@' right", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "solaire is awesome".chars().collect();
            black_box((&mut v).pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn mut_vec_pad_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&mut Vec.pad 10000 '드' right", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "don't you dare go hollow..!!#".chars().collect();
            black_box((&mut v).pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn mut_vec_pad_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&mut Vec.pad 25000 '»' right", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "東風 ぬが ㅀㆈ".chars().collect();
            black_box((&mut v).pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn mut_vec_pad_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&mut Vec.pad 50000 'ö' right", |b| {
        b.iter(|| {
            let mut v: Vec<char> = "plant needs water".chars().collect();
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
