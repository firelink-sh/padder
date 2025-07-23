use criterion::{Criterion, criterion_group};
use padder::{Alignment, MutableSource};

use std::hint::black_box;

pub fn mut_string_pad_10_left(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("&mut String.pad 10 'l' left", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1);
            s.push_str("a");
            black_box((&mut s).pad(width, Alignment::Left, 'l'));
        });
    });
}

pub fn mut_string_pad_100_left(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("&mut String.pad 100 '💀' left", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(6);
            s.push_str("babage");
            black_box((&mut s).pad(width, Alignment::Left, '💀'));
        });
    });
}

pub fn mut_string_pad_1000_left(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("&mut String.pad 1000 '@' left", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(18);
            s.push_str("solaire is awesome");
            black_box((&mut s).pad(width, Alignment::Left, '@'));
        });
    });
}

pub fn mut_string_pad_10000_left(c: &mut Criterion) {
    let width: usize = 10_000;
    c.bench_function("&mut String.pad 10000 '드' left", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(31);
            s.push_str("don't you dare go hollow..!!#");
            black_box((&mut s).pad(width, Alignment::Left, '드'));
        });
    });
}

pub fn mut_string_pad_25000_left(c: &mut Criterion) {
    let width: usize = 25_000;
    c.bench_function("&mut String.pad 25000 '»' left", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(20);
            s.push_str("東風 ぬが ㅀㆈ");
            black_box((&mut s).pad(width, Alignment::Left, '»'));
        });
    });
}

pub fn mut_string_pad_50000_left(c: &mut Criterion) {
    let width: usize = 50_000;
    c.bench_function("&mut String.pad 50000 'ö' left", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(17);
            s.push_str("plant needs water");
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
