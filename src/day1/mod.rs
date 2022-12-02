fn get_groups() -> Vec<u32> {
    let lines = include_str!("./input.txt").lines();
    let mut list = Vec::<u32>::new();
    lines.fold(0, |acc, line| {
        if let Ok(num) = line.parse::<u32>() {
            acc + num
        } else {
            list.push(acc);
            0
        }
    });
    list
}

fn first_star() -> u32 {
    let groups = get_groups();
    let max_value = groups.iter().max();
    match max_value {
        Some(val) => *val,
        None => 0,
    }
}

fn second_star() -> u32 {
    let mut groups = get_groups();
    groups.sort();
    groups.reverse();
    groups.iter().take(3).sum()
}

pub fn day1() {
    println!(
        "1st day - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
