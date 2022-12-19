#![feature(iter_array_chunks)]

use std::collections::{hash_map::RandomState, HashSet};

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("invalid item"),
    }
}

fn main() {
    // part 1
    let sum: u32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let sack = line.chars().map(|item| priority(item)).collect::<Vec<_>>();
            let split_idx = sack.len() / 2;
            let (cpt1, cpt2) = sack.split_at(split_idx);
            let set: HashSet<u32, RandomState> = HashSet::from_iter(cpt1.into_iter().cloned());
            let overlap = cpt2
                .into_iter()
                .cloned()
                .filter(|item| set.contains(item))
                .next()
                .unwrap();
            overlap
        })
        .sum();
    println!("{:?}", sum);

    // part 2
    let sum: u32 = include_str!("input.txt")
        .lines()
        .array_chunks()
        .map(|[sack1, sack2, sack3]| {
            let sack1: HashSet<u32, RandomState> =
                HashSet::from_iter(sack1.chars().map(|item| priority(item)));
            let sack2: HashSet<u32, RandomState> =
                HashSet::from_iter(sack2.chars().map(|item| priority(item)));
            let sack3: HashSet<u32, RandomState> =
                HashSet::from_iter(sack3.chars().map(|item| priority(item)));
            let overlap = sack1
                .intersection(&sack2)
                .filter(|item| sack3.contains(item))
                .next()
                .unwrap()
                .clone();
            overlap
        })
        .sum();
    println!("{:?}", sum);
}
