use std::collections::HashSet;

fn get_chars() -> Vec<char> {
    include_str!("./input.txt").chars().collect::<Vec<char>>()
}

fn start_sequence_nchars(len: usize) -> Option<usize> {
    let chars = get_chars();
    for i in 0..chars.len() {
        let set: HashSet<&char> = HashSet::from_iter(chars[i..(i + len) % chars.len()].iter());
        if set.len() == len {
            return Some(i + len);
        }
    }
    None
}

fn first_star() -> usize {
    start_sequence_nchars(4).expect("No 4-char sequence found")
}

fn second_star() -> usize {
    start_sequence_nchars(14).expect("No 14-char sequence found")
}

pub fn day6() {
    println!(
        "Day 6 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
