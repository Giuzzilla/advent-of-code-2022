use bit_vec::BitVec;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};

type ValveFlows = HashMap<String, u32>;
type EdgeMap = HashMap<String, Vec<String>>;
type DistMatrix<'a> = HashMap<(&'a String, &'a String), u32>;

fn parse_valves() -> (ValveFlows, EdgeMap) {
    let reg =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (\w+(?:, \w+)*)")
            .expect("Invalid regex");

    let mut valves = ValveFlows::new();
    let mut edge_map = EdgeMap::new();

    include_str!("input.txt").lines().for_each(|line| {
        let captures = reg.captures(line).expect("Invalid input");
        let valve_name = captures.get(1).unwrap().as_str();
        let flow_rate = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let edges: Vec<String> = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        valves.insert(valve_name.to_string(), flow_rate);
        edge_map.insert(valve_name.to_string(), edges);
    });
    (valves, edge_map)
}

type ValveMap<'a> = HashMap<&'a String, BitVec>;
type SolutionMap = HashMap<BitVec, u32>;

fn floyd_warshall(edge_map: &EdgeMap) -> DistMatrix {
    let mut dist = HashMap::new();

    for (valve, edges) in edge_map {
        for valve2 in edge_map.keys() {
            if edges.contains(valve2) {
                dist.insert((valve, valve2), 1);
            } else {
                dist.insert((valve, valve2), 999999);
            }
        }
    }

    edge_map
        .keys()
        .into_iter()
        .permutations(3)
        .for_each(|combo| {
            let (k, i, j) = (combo[0], combo[1], combo[2]);
            let orig = dist.get(&(i, j)).unwrap();
            let first_seg = dist.get(&(i, k)).unwrap();
            let second_seg = dist.get(&(k, j)).unwrap();
            let min_dist = min(*orig, *first_seg + *second_seg);
            dist.insert((i, j), min_dist);
        });

    dist
}

fn solve(
    starting_valve: &String,
    starting_minutes: i32,
    dist: &DistMatrix,
    valves: &ValveFlows,
) -> SolutionMap {
    let positive_valves = valves
        .iter()
        .filter(|(_, flow_rate)| **flow_rate > 0)
        .collect::<Vec<_>>();
    let valve_map: ValveMap = positive_valves
        .iter()
        .enumerate()
        .map(|(i, (name, _))| {
            let mut bitmap = BitVec::from_elem(positive_valves.len(), false);
            bitmap.set(i, true);
            (*name, bitmap)
        })
        .collect();

    struct State<'a> {
        valve: &'a String,
        current: BitVec,
        accumulated: u32,
        minutes: i32,
    }

    let mut stack = VecDeque::<State>::new();
    stack.push_back(State {
        valve: starting_valve,
        current: BitVec::from_elem(positive_valves.len(), false),
        accumulated: 0,
        minutes: starting_minutes,
    });

    let mut solution = SolutionMap::new();

    while !stack.is_empty() {
        let State {
            valve,
            current,
            accumulated,
            minutes,
        } = stack.pop_front().unwrap();

        solution.insert(
            current.clone(),
            max(*solution.get(&current).unwrap_or(&0), accumulated),
        );

        for (valve2, flow2) in &positive_valves {
            let remaining = minutes - *dist.get(&(valve, valve2)).unwrap() as i32 - 1;

            let mut intersect_bits = valve_map.get(valve2).unwrap().clone();
            intersect_bits.and(&current);

            if remaining > 0 && intersect_bits.none() {
                let mut current_new = current.clone();
                current_new.or(valve_map.get(valve2).unwrap());
                stack.push_back(State {
                    valve: valve2,
                    current: current_new,
                    accumulated: accumulated + *flow2 * remaining as u32,
                    minutes: remaining,
                });
            }
        }
    }

    solution
}

fn first_star() -> u32 {
    let (valves, edge_map) = parse_valves();
    let dist = floyd_warshall(&edge_map);

    let sol = solve(&"AA".to_string(), 30, &dist, &valves);

    *sol.values()
        .max()
        .expect("No solution found for first star")
}

fn second_star() -> u32 {
    let (valves, edge_map) = parse_valves();
    let dist = floyd_warshall(&edge_map);

    let solution = solve(&"AA".to_string(), 26, &dist, &valves);
    let solution2 = solution
        .iter()
        .combinations(2)
        .filter(|combo| {
            let (bitmap1, bitmap2) = (combo[0].0, combo[1].0);
            let mut bitmap1_clone = bitmap1.clone();
            bitmap1_clone.and(bitmap2);
            bitmap1_clone.none()
        })
        .map(|combo| combo[0].1 + combo[1].1);

    solution2.max().expect("No solution found for second star")
}

pub fn day16() {
    println!(
        "Day 16 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
