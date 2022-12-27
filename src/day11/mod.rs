use num::Integer;
use std::cell::RefCell;

enum Op {
    Add(u64),
    AddSelf,
    Multiply(u64),
    MultiplySelf,
}

struct Monkey {
    items: RefCell<Vec<u64>>,
    op: Op,
    div: u64,
    true_monkey: u8,
    false_monkey: u8,
}

fn get_monkeys() -> Vec<Monkey> {
    let monkey_str = include_str!("./input.txt").split("\n\n");
    monkey_str.map(|s| parse_monkey(s)).collect()
}

fn parse_monkey(string: &str) -> Monkey {
    let lines = string.lines().map(|s| s.trim()).collect::<Vec<&str>>();

    let items: Vec<u64> = lines[1]["Starting items: ".len()..]
        .split(", ")
        .map(|s| s.parse::<u64>().expect("Should be a number"))
        .collect();

    let full_operation = lines[2]["Operation: new = old ".len()..]
        .split(" ")
        .collect::<Vec<&str>>();

    let op = match full_operation[..] {
        ["+", "old"] => Op::AddSelf,
        ["*", "old"] => Op::MultiplySelf,
        ["+", value] => Op::Add(value.parse::<u64>().expect("Should be a number")),
        ["*", value] => Op::Multiply(value.parse::<u64>().expect("Should be a number")),
        _ => panic!("Unknown operation"),
    };

    let div = lines[3]["Test: divisible by ".len()..]
        .parse::<u64>()
        .expect("Should be a number");

    let true_monkey = lines[4]["If true: throw to monkey ".len()..]
        .parse::<u8>()
        .expect("Should be a number");

    let false_monkey = lines[5]["If false: throw to monkey ".len()..]
        .parse::<u8>()
        .expect("Should be a number");

    Monkey {
        items: RefCell::new(items),
        op,
        div,
        true_monkey,
        false_monkey,
    }
}

fn execute_rounds(
    monkeys: &Vec<Monkey>,
    n_rounds: u64,
    transform_fn: &dyn Fn(u64) -> u64,
) -> Vec<u64> {
    let mut counts: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..n_rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let mut monkey_items = monkey.items.borrow_mut();
            while monkey_items.len() > 0 {
                counts[i] += 1;
                let item = monkey_items.pop().expect("Should have an item");
                let new_item = transform_fn(match monkey.op {
                    Op::AddSelf => item + item,
                    Op::MultiplySelf => item * item,
                    Op::Add(value) => item + value,
                    Op::Multiply(value) => item * value,
                });
                if new_item % monkey.div == 0 {
                    monkeys[monkey.true_monkey as usize]
                        .items
                        .borrow_mut()
                        .push(new_item);
                } else {
                    monkeys[monkey.false_monkey as usize]
                        .items
                        .borrow_mut()
                        .push(new_item);
                }
            }
        }
    }
    counts
}

fn first_star() -> u64 {
    let monkeys = get_monkeys();
    let mut counts = execute_rounds(&monkeys, 20, &|x| x / 3);
    best_two_mult(&mut counts)
}

fn second_star() -> u64 {
    let monkeys = get_monkeys();
    let lcm = monkeys.iter().fold(1, |acc, monkey| acc.lcm(&monkey.div));
    let mut counts = execute_rounds(&monkeys, 10000, &|x| x % lcm);
    best_two_mult(&mut counts)
}

fn best_two_mult(counts: &mut Vec<u64>) -> u64 {
    counts.sort();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

pub fn day11() {
    println!(
        "Day 11 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    );
}
