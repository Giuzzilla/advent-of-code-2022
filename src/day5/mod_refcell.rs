//! Alternative implementation using RefCell (interior mutability pattern)

use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

struct Instruction {
    from: u8,
    to: u8,
    quantity: u8,
}

type StackMap = HashMap<u8, RefCell<VecDeque<char>>>;

fn get_stacks() -> StackMap {
    let lines = include_str!("./input.txt").lines().collect::<Vec<&str>>();
    let stacklines = lines[..9].iter();

    let mut stacks = StackMap::new();
    for i in 0..9 {
        let stack = VecDeque::new();
        stacks.insert(i as u8, RefCell::new(stack));
    }

    for line in stacklines {
        let re = Regex::new(r"(?:(\[\w\]|   )(?: |$))").unwrap();
        let captures = re.captures_iter(line);
        for (i, capture) in (0_u8..).zip(captures) {
            let mut stack = stacks.get_mut(&i).unwrap().borrow_mut();
            if let Some(c) = capture.get(1) {
                if c.as_str() != "   " {
                    stack.push_back(c.as_str().chars().nth(1).unwrap());
                }
            }
        }
    }
    stacks
}

fn get_instructions() -> Vec<Instruction> {
    include_str!("./input.txt").lines().collect::<Vec<&str>>()[10..]
        .iter()
        .map(|line| {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let capture = re.captures(line).unwrap();
            let from = capture.get(2).unwrap().as_str().parse::<u8>().unwrap() - 1;
            let to = capture.get(3).unwrap().as_str().parse::<u8>().unwrap() - 1;
            let quantity = capture.get(1).unwrap().as_str().parse::<u8>().unwrap();
            Instruction { from, to, quantity }
        })
        .collect()
}

fn get_tops(stacks: &StackMap) -> Vec<char> {
    let mut backs = <Vec<char>>::new();
    for i in 0..9 {
        let stack = stacks.get(&(i as u8)).unwrap().borrow();
        if let Some(c) = stack.front() {
            backs.push(*c);
        }
    }
    backs
}

fn execute_instruction(stacks: &mut StackMap, instruction: &Instruction, maintain_order: bool) {
    let mut from = stacks.get(&instruction.from).unwrap().borrow_mut();
    let mut to = stacks.get(&instruction.to).unwrap().borrow_mut();
    if maintain_order {
        let mut accumulator = <Vec<char>>::new();
        for _ in 0..instruction.quantity {
            let el = from.pop_front().unwrap();
            accumulator.push(el);
        }
        for el in accumulator.iter().rev() {
            to.push_front(*el);
        }
    } else {
        for _ in 0..instruction.quantity {
            let el = from.pop_front().unwrap();
            to.push_front(el);
        }
    }
}

fn first_star() -> String {
    let mut stacks = get_stacks();
    let instructions = get_instructions();

    for instruction in instructions {
        execute_instruction(&mut stacks, &instruction, false);
    }
    get_tops(&stacks).iter().collect()
}

fn second_star() -> String {
    let mut stacks = get_stacks();
    let instructions = get_instructions();

    for instruction in instructions {
        execute_instruction(&mut stacks, &instruction, true);
    }
    get_tops(&stacks).iter().collect()
}

pub fn day5() {
    println!(
        "Day 5 - First star: {}, Second star: {}",
        first_star(),
        second_star(),
    );
}
