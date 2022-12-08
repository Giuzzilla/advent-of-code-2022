fn parse_input() -> Vec<Vec<u32>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).expect("Must be a number"))
                .collect()
        })
        .collect()
}

fn get_subvectors(
    input: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut top = vec![];
    for r in 0..i {
        top.push(input[r][j]);
    }
    let mut bottom = vec![];
    for r in (i + 1)..input.len() {
        bottom.push(input[r][j]);
    }
    let mut left = vec![];
    for c in 0..j {
        left.push(input[i][c]);
    }
    let mut right = vec![];
    for c in (j + 1)..input[0].len() {
        right.push(input[i][c]);
    }
    (top, bottom, left, right)
}

fn has_blocked_view(curr: u32, subvec: Vec<u32>) -> bool {
    subvec.len() > 0 && subvec.into_iter().any(|el| el >= curr)
}

fn first_star(input: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let curr = input[i][j];
            let (top, bottom, left, right) = get_subvectors(&input, i, j);
            let blocked_top = has_blocked_view(curr, top);
            let blocked_bottom = has_blocked_view(curr, bottom);
            let blocked_left = has_blocked_view(curr, left);
            let blocked_right = has_blocked_view(curr, right);
            if !blocked_top | !blocked_bottom | !blocked_left | !blocked_right {
                count += 1;
            }
        }
    }
    count
}

fn how_many_before_block(curr: u32, subvec: Vec<u32>) -> u32 {
    let mut count = 0;

    for el in subvec.into_iter() {
        if el < curr {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

fn second_star(input: &Vec<Vec<u32>>) -> u32 {
    let mut scores = vec![];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let curr = input[i][j];
            let (mut top, bottom, mut left, right) = get_subvectors(&input, i, j);

            top.reverse();
            let score_top = how_many_before_block(curr, top);
            let score_bottom = how_many_before_block(curr, bottom);

            left.reverse();
            let score_left = how_many_before_block(curr, left);
            let score_right = how_many_before_block(curr, right);
            scores.push(score_top * score_bottom * score_left * score_right);
        }
    }
    scores
        .into_iter()
        .max()
        .expect("Should have at least a score")
}

pub fn day8() {
    let input = parse_input();

    println!(
        "Day 8 - First star: {}, Second star: {}",
        first_star(&input),
        second_star(&input),
    );
}
