use itertools::Itertools;
use std::collections::HashSet;

type Rucksack = Vec<char>;
type CharSet = HashSet<char>;

fn get_rucksacks() -> Vec<Rucksack> {
    include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn convert_char(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - (65 - 27)
    } else {
        (c as u32) - 96
    }
}

fn first_star() -> u32 {
    let rucks = get_rucksacks();
    let mut score: u32 = 0;
    for ruck in rucks {
        let mid = ruck.len() / 2;
        let mut first = ruck;
        let second = first.split_off(mid);

        let first_set = CharSet::from_iter(first);
        let second_set = CharSet::from_iter(second);

        let common = *first_set.intersection(&second_set).next().unwrap();
        score += convert_char(common);
    }
    score
}

fn second_star() -> u32 {
    let rucks = get_rucksacks();
    let mut score: u32 = 0;
    for (first, second, third) in rucks.into_iter().tuples() {
        let first_set = CharSet::from_iter(first);
        let second_set = CharSet::from_iter(second);
        let third_set = CharSet::from_iter(third);

        let common = *first_set
            .intersection(&second_set)
            .cloned()
            .collect::<CharSet>()
            .intersection(&third_set)
            .next()
            .unwrap();

        score += convert_char(common);
    }
    score
}

pub fn day3() {
    println!(
        "Day 3 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
