struct Node {
    total_under: u32,
    children: Vec<Box<Node>>,
}

fn split_cds(input: Vec<&str>) -> Vec<Vec<&str>> {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    for line in input {
        if line.starts_with("$ cd") {
            groups.push(vec![line]);
        } else if line != "" && line != "$ ls" && !line.starts_with("dir") {
            let last_group = groups.last_mut().expect("No last group");
            last_group.push(line);
        }
    }
    groups
}

fn collapse_group(group: Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in group {
        if !line.starts_with("$ cd") {
            let size = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            total += size;
        }
    }
    total
}

fn pop_from_stack(stack: &mut Vec<Node>) -> &Vec<Node> {
    let removed = stack.pop().unwrap();
    let mut last_node = stack.last_mut().unwrap();
    last_node.total_under += removed.total_under;
    last_node.children.push(Box::new(removed));
    stack
}

fn construct_tree(groups: Vec<Vec<&str>>) -> Node {
    let mut stack: Vec<Node> = Vec::new();
    for group in groups {
        let first_line = group[0];
        if first_line == "$ cd .." {
            pop_from_stack(&mut stack);
        } else if first_line.starts_with("$ cd") {
            let size: u32 = collapse_group(group);
            let new = Node {
                children: Vec::new(),
                total_under: size,
            };
            stack.push(new);
        }
    }

    while stack.len() > 1 {
        pop_from_stack(&mut stack);
    }
    stack.pop().unwrap()
}

fn collect_nodes<'a>(root: &'a Node, criterion: &dyn Fn(u32) -> bool) -> Vec<&'a Node> {
    let mut collection: Vec<&Node> = Vec::new();
    if criterion(root.total_under) {
        collection.push(root);
    }
    for child in &root.children {
        collection.extend(collect_nodes(&child, criterion));
    }
    collection
}

fn first_star(root: &Node) -> u32 {
    collect_nodes(root, &|val| val < 100000)
        .iter()
        .map(|node| node.total_under)
        .sum::<u32>()
}

fn second_star(root: &Node) -> u32 {
    collect_nodes(root, &|val| val > 30000000 - (70000000 - root.total_under))
        .iter()
        .map(|node| node.total_under)
        .min()
        .unwrap()
}

pub fn day7() {
    let input = include_str!("input.txt");
    let groups = split_cds(input.lines().collect());
    let node = construct_tree(groups);
    println!(
        "Day 7 - First star: {}, Second star: {}",
        first_star(&node),
        second_star(&node),
    );
}
