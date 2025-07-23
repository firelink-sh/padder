use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_pad_10_right(c: &mut Criterion) {
    let width: usize = 10;
    let mut source = String::from("a");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 10 'l' right", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Right, 'l'));
        });
    });
}

pub fn mut_string_pad_100_right(c: &mut Criterion) {
    let width: usize = 100;
    let mut source = String::from("babage");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 100 '💀' right", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Right, '💀'));
        });
    });
}

pub fn mut_string_pad_1000_right(c: &mut Criterion) {
    let width: usize = 1000;
    let mut source = String::from("solaire is awesome");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 1000 '@' right", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Right, '@'));
        });
    });
}

pub fn mut_string_pad_10000_right(c: &mut Criterion) {
    let width: usize = 10_000;
    let mut source = String::from("don't you dare go hollow..!!#");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 10_000 '드' right", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Right, '드'));
        });
    });
}

pub fn mut_string_pad_25000_right(c: &mut Criterion) {
    let width: usize = 25_000;
    let mut source = String::from("東風 ぬが ㅀㆈ");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 25_000 '»' right", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Right, '»'));
        });
    });
}

pub fn mut_string_pad_50000_right(c: &mut Criterion) {
    let width: usize = 50_000;
    let mut source = String::from("plant needs water");
    source.shrink_to_fit();
    c.bench_function("&mut String.pad 50_000 'ö' right", |b| {
        b.iter(|| {
            let mut s = black_box(source.clone());
            black_box((&mut s).pad(width, Alignment::Right, 'ö'));
        });
    });
}

criterion_group!(
    pads,
    mut_string_pad_10_right,
    mut_string_pad_100_right,
    mut_string_pad_1000_right,
    mut_string_pad_10000_right,
    mut_string_pad_25000_right,
    mut_string_pad_50000_right,
);
