use std::collections::HashMap;

struct Round {
    first: char,
    second: char,
}

fn get_rounds() -> Vec<Round> {
    let lines = include_str!("./input.txt").lines();
    lines
        .map(|line| {
            if let [first, ' ', second] = line.chars().collect::<Vec<char>>()[..] {
                Round {
                    first: first,
                    second: second,
                }
            } else {
                panic!("Invalid line: {}", line);
            }
        })
        .collect()
}

fn first_star() -> u32 {
    let rounds = get_rounds();
    let mut score: u32 = 0;
    let order: HashMap<char, u32> = HashMap::from([('A', 0), ('B', 1), ('C', 2)]);
    let trans: HashMap<char, char> = HashMap::from([('X', 'A'), ('Y', 'B'), ('Z', 'C')]);

    for round in rounds {
        let translated = trans[&round.second];
        let idx1: u32 = order[&round.first];
        let idx2 = order[&translated];
        if round.first == translated {
            score += 3 + idx2 + 1;
        } else if (idx1 + 1) % 3 == idx2 {
            score += 6 + idx2 + 1;
        } else {
            score += idx2 + 1;
        }
    }
    score
}

fn second_star() -> u32 {
    let rounds = get_rounds();
    let mut score: u32 = 0;
    let order: HashMap<char, u32> = HashMap::from([('A', 0), ('B', 1), ('C', 2)]);

    for round in rounds {
        let idx = order[&round.first] as i32;
        let lose_idx = (idx - 1).rem_euclid(3);
        let win_idx = (idx + 1).rem_euclid(3);
        let increase = match round.second {
            'X' => lose_idx + 1,
            'Y' => 3 + idx + 1,
            'Z' => 6 + win_idx + 1,
            _ => panic!("Invalid round: {} {}", round.first, round.second),
        };
        score += increase as u32;
    }
    score
}

pub fn day2() {
    println!(
        "2nd day - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
