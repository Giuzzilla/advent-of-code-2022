use std::collections::HashSet;

struct Move {
    dir: char,
    size: u8,
}

type Position = (i32, i32);

fn get_moves() -> Vec<Move> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let dir = line.chars().next().expect("Should have a direction");
            let size = line
                .split(' ')
                .nth(1)
                .expect("Should have a size")
                .parse::<u8>()
                .expect("Should have a numeric size");
            Move { dir, size }
        })
        .collect()
}

fn first_star() -> usize {
    let moves = get_moves();
    let mut tail_pos = (0, 0);
    let mut head_pos = (0, 0);
    let mut visited_positions: HashSet<Position> = HashSet::new();
    visited_positions.insert(tail_pos);

    for move_ in moves {
        for _ in 0..move_.size {
            head_pos = iterate_head(head_pos, move_.dir);
            tail_pos = follow_head(tail_pos, head_pos);
            visited_positions.insert(tail_pos);
        }
    }
    visited_positions.len()
}

fn second_star() -> usize {
    let moves = get_moves();
    const NUM_TAILS: usize = 9;
    let mut tails: Vec<Position> = Vec::new();
    for _ in 0..NUM_TAILS {
        tails.push((0, 0));
    }
    let mut head_pos = (0, 0);
    let mut visited_positions: HashSet<Position> = HashSet::new();
    visited_positions.insert((0, 0));
    for move_ in moves {
        for _ in 0..move_.size {
            head_pos = iterate_head(head_pos, move_.dir);
            for i in 0..NUM_TAILS {
                let pos_to_follow = if i == 0 { head_pos } else { tails[(i - 1)] };
                tails[i as usize] = follow_head(tails[i as usize], pos_to_follow);
                if i == 8 {
                    visited_positions.insert(tails[i as usize]);
                }
            }
        }
    }

    visited_positions.len()
}

fn iterate_head(pos: Position, dir: char) -> Position {
    match dir {
        'U' => (pos.0, pos.1 + 1),
        'D' => (pos.0, pos.1 - 1),
        'L' => (pos.0 - 1, pos.1),
        'R' => (pos.0 + 1, pos.1),
        _ => panic!("Invalid direction"),
    }
}

fn follow_head(tail_pos: Position, head_pos: Position) -> Position {
    let (x, y) = head_pos;
    let (tx, ty) = tail_pos;

    let diff_x = x - tx;
    let abs_diff_x = diff_x.abs();
    let sign_x = if diff_x > 0 { 1 } else { -1 };

    let diff_y = y - ty;
    let abs_diff_y = diff_y.abs();
    let sign_y = if diff_y > 0 { 1 } else { -1 };

    if (abs_diff_x > 0 && abs_diff_y > 1) || (abs_diff_x > 1 && abs_diff_y > 0) {
        (tx + sign_x, ty + sign_y)
    } else if abs_diff_x > 1 {
        (tx + sign_x, ty)
    } else if abs_diff_y > 1 {
        (tx, ty + sign_y)
    } else {
        tail_pos
    }
}

pub fn day9() {
    println!(
        "Day 9 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
