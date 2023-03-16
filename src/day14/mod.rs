use std::collections::HashSet;

type Point = (u32, u32);

fn get_rock_points() -> HashSet<Point> {
    include_str!("./input.txt")
        .lines()
        .flat_map(|line| {
            let parts: Vec<Point> = line
                .split(" -> ")
                .map(|part| {
                    let mut parts = part.split(',');
                    let x = parts.next().unwrap().parse::<u32>().unwrap();
                    let y = parts.next().unwrap().parse::<u32>().unwrap();
                    (x, y)
                })
                .collect();

            let mut rocks: HashSet<Point> = HashSet::new();
            for (point_1, point_2) in parts.iter().zip(parts.iter().skip(1)) {
                let (x1, y1) = *point_1;
                let (x2, y2) = *point_2;
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        rocks.insert((x1, y));
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        rocks.insert((x, y1));
                    }
                };
            }
            rocks
        })
        .collect()
}

static POUR_POINT: Point = (500, 0);

fn first_star() -> u32 {
    let mut curr_sand = POUR_POINT;
    let mut rocks = get_rock_points();
    let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap();
    let mut n_sand: u32 = 0;
    while curr_sand.1 < max_y {
        if !rocks.contains(&(curr_sand.0, curr_sand.1 + 1)) {
            curr_sand = (curr_sand.0, curr_sand.1 + 1);
        } else if !rocks.contains(&(curr_sand.0 - 1, curr_sand.1 + 1)) {
            curr_sand = (curr_sand.0 - 1, curr_sand.1 + 1);
        } else if !rocks.contains(&(curr_sand.0 + 1, curr_sand.1 + 1)) {
            curr_sand = (curr_sand.0 + 1, curr_sand.1 + 1);
        } else {
            mark_bottom(&mut rocks, &mut curr_sand, &mut n_sand);
        }
    }
    n_sand
}

fn second_star() -> u32 {
    let mut curr_sand = POUR_POINT;
    let mut rocks = get_rock_points();
    let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap() + 2;
    let mut n_sand: u32 = 0;
    while !rocks.contains(&POUR_POINT) {
        if curr_sand.1 + 1 >= max_y {
            mark_bottom(&mut rocks, &mut curr_sand, &mut n_sand);
        } else if !rocks.contains(&(curr_sand.0, curr_sand.1 + 1)) {
            curr_sand = (curr_sand.0, curr_sand.1 + 1);
        } else if !rocks.contains(&(curr_sand.0 - 1, curr_sand.1 + 1)) {
            curr_sand = (curr_sand.0 - 1, curr_sand.1 + 1);
        } else if !rocks.contains(&(curr_sand.0 + 1, curr_sand.1 + 1)) {
            curr_sand = (curr_sand.0 + 1, curr_sand.1 + 1);
        } else {
            mark_bottom(&mut rocks, &mut curr_sand, &mut n_sand);
        }
    }
    n_sand
}

fn mark_bottom(rocks: &mut HashSet<Point>, curr_sand: &mut Point, n_sand: &mut u32) {
    rocks.insert(*curr_sand);
    *curr_sand = POUR_POINT;
    *n_sand += 1;
}

pub fn day14() {
    println!(
        "Day 14 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
