use petgraph::algo::k_shortest_path::k_shortest_path;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

type CharGrid = Vec<Vec<char>>;
type Mountain = DiGraph<(), ()>;
type NodeMap = HashMap<(usize, usize), NodeIndex>;

fn input_grid() -> CharGrid {
    include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn create_graph(grid: &CharGrid) -> (Mountain, NodeMap) {
    let mut mapping: NodeMap = HashMap::new();
    let mut graph = Mountain::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let u = (x as usize, y as usize);
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let v_check = (x as i32 + dx, y as i32 + dy);
                if v_check.0 < 0
                    || v_check.0 >= row.len() as i32
                    || v_check.1 < 0
                    || v_check.1 >= grid.len() as i32
                {
                    continue;
                }

                let v = (v_check.0 as usize, v_check.1 as usize);

                let u_idx = if !mapping.contains_key(&u) {
                    let idx = graph.add_node(());
                    mapping.insert(u, idx);
                    idx
                } else {
                    *mapping.get(&u).unwrap()
                };

                let v_idx = if !mapping.contains_key(&v) {
                    let idx = graph.add_node(());
                    mapping.insert(v, idx);
                    idx
                } else {
                    *mapping.get(&v).unwrap()
                };

                if valid_edge(*cell, input_grid()[v.1 as usize][v.0 as usize]) {
                    graph.add_edge(u_idx, v_idx, ());
                }
            }
        }
    }
    (graph, mapping)
}

fn valid_edge(u_char: char, v_char: char) -> bool {
    let u_value = if u_char != 'S' {
        u_char as u32
    } else {
        'a' as u32
    };
    let v_value = if v_char != 'E' {
        v_char as u32
    } else {
        'z' as u32
    };
    v_value <= u_value + 1
}

fn get_nodes(grid: &CharGrid, mapping: &NodeMap, c: char) -> Vec<NodeIndex> {
    let mut node_indexes: Vec<NodeIndex> = Vec::new();
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            if grid[j][i] == c {
                node_indexes.push(*mapping.get(&(i, j)).unwrap());
            }
        }
    }
    node_indexes
}

fn first_star() -> u32 {
    let grid = input_grid();
    let (graph, mapping) = create_graph(&grid);

    let start = get_nodes(&grid, &mapping, 'S')[0];
    let end = get_nodes(&grid, &mapping, 'E')[0];

    *k_shortest_path(&graph, start, Some(end), 1, |_| 1)
        .get(&end)
        .expect("No path found")
}

fn second_star() -> u32 {
    let grid = input_grid();
    let (graph, mapping) = create_graph(&grid);

    let mut starts = get_nodes(&grid, &mapping, 'a');
    starts.push(get_nodes(&grid, &mapping, 'S')[0]);
    let end = get_nodes(&grid, &mapping, 'E')[0];

    let mut min = std::u32::MAX;
    for start in starts {
        let path = *k_shortest_path(&graph, start, Some(end), 1, |_| 1)
            .get(&end)
            .unwrap_or(&std::u32::MAX);
        if path < min {
            min = path;
        }
    }
    min
}

pub fn day12() {
    println!(
        "Day 12 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
