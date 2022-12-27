use std::collections::HashSet;

type Range = (u32, u32);

struct RangePair {
    first: Range,
    second: Range,
}

type RangeSet = HashSet<u32>;

fn get_ranges() -> Vec<RangePair> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (first_range, second_range) = match line
                .split(',')
                .take(2)
                .collect::<Vec<&str>>()[..]
            {
                [first, second] => (first, second),
                _ => panic!("Invalid input"),
            };

            let (start_1, end_1) = match first_range
                .split('-')
                .take(2)
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[..]
            {
                [start, end] => (start, end),
                _ => panic!("Invalid input"),
            };

            let (start_2, end_2) = match second_range
                .split('-')
                .take(2)
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[..]
            {
                [start, end] => (start, end),
                _ => panic!("Invalid input"),
            };
            RangePair {
                first: (start_1, end_1),
                second: (start_2, end_2),
            }
        })
        .collect()
}

fn get_sets(range: RangePair) -> (RangeSet, RangeSet) {
    let first_set: HashSet<u32> = HashSet::from_iter(range.first.0..=range.first.1);
    let second_set: HashSet<u32> = HashSet::from_iter(range.second.0..=range.second.1);
    (first_set, second_set)
}

fn get_intersection(first: &RangeSet, second: &RangeSet) -> RangeSet {
    first.intersection(second).cloned().collect::<RangeSet>()
}

fn first_star() -> u32 {
    let ranges = get_ranges();
    let mut count: u32 = 0;
    for range in ranges {
        let (first_set, second_set) = get_sets(range);
        let intersection = get_intersection(&first_set, &second_set);
        if (intersection == first_set) || (intersection == second_set) {
            count += 1;
        };
    }
    count
}

fn second_star() -> u32 {
    let ranges = get_ranges();
    let mut count: u32 = 0;
    for range in ranges {
        let (first_set, second_set) = get_sets(range);
        let intersection = get_intersection(&first_set, &second_set);
        if !intersection.is_empty() {
            count += 1;
        };
    }
    count
}

pub fn day4() {
    println!(
        "Day 4 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
