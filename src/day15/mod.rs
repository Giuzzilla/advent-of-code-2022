use num::abs;
use regex::Regex;
use std::collections::HashSet;

type Coord = (i64, i64);

struct Sensor {
    coords: Coord,
    closest_beacon: Coord,
    range: i64,
}

fn dist(a: Coord, b: Coord) -> i64 {
    (abs(a.0 - b.0) + abs(a.1 - b.1)) as i64
}

impl Sensor {
    fn new(s: &str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .expect("Invalid regex");
        let capture = re.captures(s).expect("Invalid input string");
        let sx = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let sy = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let bx = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let by = capture.get(4).unwrap().as_str().parse::<i64>().unwrap();

        let closest_beacon = (bx, by);
        let coords = (sx, sy);
        let range = dist(coords, (bx, by));
        Sensor {
            coords,
            closest_beacon,
            range,
        }
    }

    fn dist(&self, other: Coord) -> i64 {
        dist(self.coords, other)
    }
}

fn get_sensors() -> Vec<Sensor> {
    include_str!("./input.txt")
        .lines()
        .map(Sensor::new)
        .collect()
}

fn get_set_used_coords(sensors: &Vec<Sensor>) -> HashSet<Coord> {
    let mut used_coords = HashSet::new();
    for sensor in sensors {
        used_coords.insert(sensor.coords);
        used_coords.insert(sensor.closest_beacon);
    }
    used_coords
}

const Y_FIRST_STAR: i64 = 2000000;

fn first_star() -> usize {
    let sensors = get_sensors();
    let used_coords = get_set_used_coords(&sensors);
    let mut matches: HashSet<Coord> = HashSet::new();
    for sensor in sensors {
        for x in (sensor.coords.0 - sensor.range)..=(sensor.coords.0 + sensor.range) {
            if sensor.dist((x, Y_FIRST_STAR)) <= sensor.range
                && !used_coords.contains(&(x, Y_FIRST_STAR))
            {
                matches.insert((x, Y_FIRST_STAR));
            }
        }
    }
    matches.len() as usize
}

const MIN_COORD: i64 = 0;
const MAX_COORD: i64 = 4000000;

fn second_star() -> i64 {
    let sensors = get_sensors();
    let mut points: Vec<Coord> = Vec::new();
    for sensor in &sensors {
        let extern_dist = sensor.range + 1;
        for dx in -extern_dist..=extern_dist {
            let dy = extern_dist - dx;
            let x = sensor.coords.0 + dx;
            let y = sensor.coords.1 + dy;
            if (MIN_COORD..=MAX_COORD).contains(&x) && (MIN_COORD..=MAX_COORD).contains(&y) {
                points.push((x, y));
            }
        }
    }

    let matching_point: Option<Coord> = points.iter().cloned().find(|point| {
        for sensor in &sensors {
            if sensor.dist(*point) <= sensor.range {
                return false;
            }
        }
        true
    });
    match matching_point {
        Some(point) => point.0 * MAX_COORD + point.1,
        None => panic!("No matching point found"),
    }
}

pub fn day15() {
    println!(
        "Day 15 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
