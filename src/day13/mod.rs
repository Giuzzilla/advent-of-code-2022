use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
enum Value {
    Num(i32),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        if compare(self, other) < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Pair = (Value, Value);

fn get_pairs() -> Vec<Pair> {
    include_str!("./input.txt")
        .split("\n\n")
        .map(|group| {
            let mut pairs_str = group.split("\n");
            let first: Value =
                serde_json::from_str(&pairs_str.next().expect("Expected first of pair"))
                    .expect("Wrong type of first of pair");
            let second: Value =
                serde_json::from_str(&pairs_str.next().expect("Expected second of pair"))
                    .expect("Wrong type of second of pair");
            (first, second)
        })
        .collect()
}

fn compare(v1: &Value, v2: &Value) -> i32 {
    match (v1, v2) {
        (Value::Num(n1), Value::Num(n2)) => *n1 - *n2,
        (Value::Num(n1), l2 @ Value::List(_)) => compare(&Value::List(vec![Value::Num(*n1)]), l2),
        (l1 @ Value::List(_), Value::Num(n2)) => compare(l1, &Value::List(vec![Value::Num(*n2)])),
        (Value::List(v1), Value::List(v2)) => {
            let mut ret = 0;
            let (len1, len2) = ((*v1).len(), (*v2).len());
            for (sub_v1, sub_v2) in zip(v1, v2) {
                ret = compare(sub_v1, sub_v2);
                if ret != 0 {
                    break;
                }
            }
            if ret != 0 {
                return ret;
            }

            if len1 < len2 {
                -1 as i32
            } else {
                (len1 > len2) as i32
            }
        }
    }
}

fn first_star() -> usize {
    let pairs = get_pairs();
    let mut ordered_ids: Vec<usize> = Vec::new();
    for i in 0..pairs.len() {
        if compare(&pairs[i].0, &pairs[i].1) < 0 {
            ordered_ids.push(i + 1);
        }
    }
    ordered_ids.iter().sum()
}

fn second_star() -> usize {
    let pairs = get_pairs();

    let dividers = (
        Value::List(vec![Value::List(vec![Value::Num(2)])]),
        Value::List(vec![Value::List(vec![Value::Num(6)])]),
    );
    let mut values: Vec<&Value> = pairs
        .iter()
        .flat_map(|pair| vec![&pair.0, &pair.1])
        .collect();

    values.push(&dividers.0);
    values.push(&dividers.1);

    values.sort();

    let mut mult = 1;
    for i in 0..values.len() {
        if values[i] == &dividers.0 || values[i] == &dividers.1 {
            mult *= i + 1;
        }
    }
    mult
}

pub fn day13() {
    println!(
        "Day 13 - First star: {}, Second star: {}",
        first_star(),
        second_star()
    )
}
