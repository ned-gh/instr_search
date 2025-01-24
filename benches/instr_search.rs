use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use ahash::AHashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, thread_rng};
use rustc_hash::FxHashMap;

fn get_instructions() -> Vec<String> {
    fs::read_to_string("instructions.txt")
        .unwrap()
        .lines()
        .map(ToString::to_string)
        .collect()
}

pub fn gen_instr_search(c: &mut Criterion) {
    let mut rng = thread_rng();
    let instructions = get_instructions();

    c.bench_function("generated_instr_search", |b| {
        b.iter_batched(
            || instructions.choose(&mut rng).unwrap(),
            |instr| {
                black_box(instr_search::generated::instr_search(instr));
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn hashmap_instr_search(c: &mut Criterion) {
    let mut rng = thread_rng();
    let instructions = get_instructions();
    let map: HashMap<&str, u16> = instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| (instr.as_str(), i as u16))
        .collect();

    c.bench_function("hashmap_instr_search", |b| {
        b.iter_batched(
            || instructions.choose(&mut rng).unwrap(),
            |instr| {
                black_box(instr_search::hashmap::instr_search(instr, &map));
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn btreemap_instr_search(c: &mut Criterion) {
    let mut rng = thread_rng();
    let instructions = get_instructions();
    let map: BTreeMap<&str, u16> = instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| (instr.as_str(), i as u16))
        .collect();

    c.bench_function("btreemap_instr_search", |b| {
        b.iter_batched(
            || instructions.choose(&mut rng).unwrap(),
            |instr| {
                black_box(instr_search::btreemap::instr_search(instr, &map));
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn ahash_instr_search(c: &mut Criterion) {
    let mut rng = thread_rng();
    let instructions = get_instructions();
    let map: AHashMap<&str, u16> = instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| (instr.as_str(), i as u16))
        .collect();

    c.bench_function("ahash_instr_search", |b| {
        b.iter_batched(
            || instructions.choose(&mut rng).unwrap(),
            |instr| {
                black_box(instr_search::ahash::instr_search(instr, &map));
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn fxhash_instr_search(c: &mut Criterion) {
    let mut rng = thread_rng();
    let instructions = get_instructions();
    let map: FxHashMap<&str, u16> = instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| (instr.as_str(), i as u16))
        .collect();

    c.bench_function("fxhash_instr_search", |b| {
        b.iter_batched(
            || instructions.choose(&mut rng).unwrap(),
            |instr| {
                black_box(instr_search::fxhash::instr_search(instr, &map));
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    gen_instr_search,
    hashmap_instr_search,
    btreemap_instr_search,
    ahash_instr_search,
    fxhash_instr_search,
);
criterion_main!(benches);
