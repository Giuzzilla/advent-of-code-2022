enum Instruction {
    AddX(i32),
    Noop,
}

trait Cycler {
    fn run_cycles(&mut self) {
        let instructions = parse_instructions();
        let mut x = 1;
        for instruction in instructions {
            self.cycle(x);
            match instruction {
                Instruction::AddX(value) => {
                    self.cycle(x);
                    x += value;
                }
                Instruction::Noop => (),
            }
        }
    }

    fn cycle(&mut self, x: i32);
}

fn parse_instructions() -> Vec<Instruction> {
    include_str!("./input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let instruction = parts.next().expect("Should have an instruction");

            match instruction {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(
                    parts
                        .next()
                        .expect("Should have a value")
                        .parse::<i32>()
                        .expect("Should have a numeric value"),
                ),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

struct Star1Cycler {
    cycles_lst: Vec<i32>,
}

impl Star1Cycler {
    fn new() -> Self {
        Star1Cycler {
            cycles_lst: Vec::<i32>::new(),
        }
    }
}

impl Cycler for Star1Cycler {
    fn cycle(&mut self, x: i32) {
        self.cycles_lst.push(x);
    }
}

fn first_star() -> i32 {
    let mut cycler = Star1Cycler::new();

    cycler.run_cycles();

    let desired_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    desired_cycles
        .iter()
        .map(|cycle| *cycle as i32 * cycler.cycles_lst[cycle - 1])
        .sum()
}

struct Star2Cycler {
    cycle: i32,
    board: Vec<Vec<char>>,
}

impl Star2Cycler {
    fn new() -> Self {
        Star2Cycler {
            cycle: 0,
            board: Vec::<Vec<char>>::new(),
        }
    }

    fn draw_char(&mut self, x: i32) {
        if self.cycle % 40 == 0 {
            self.board.push(Vec::<char>::new());
        }

        let last_row = self.board.last_mut().expect("Should have a last row");
        if should_draw_hash(self.cycle, x) {
            last_row.push('#');
        } else {
            last_row.push(' ');
        }
    }
}

impl Cycler for Star2Cycler {
    fn cycle(&mut self, x: i32) {
        self.draw_char(x);
        self.cycle += 1;
    }
}

fn second_star() {
    let mut cycler = Star2Cycler::new();

    cycler.run_cycles();

    for row in cycler.board {
        println!("{}", row.iter().collect::<String>());
    }
}

fn should_draw_hash(cycle: i32, x: i32) -> bool {
    let cycle_mod = cycle % 40;
    vec![x - 1, x, x + 1].contains(&cycle_mod)
}

pub fn day10() {
    println!("Day 10 - First star: {}, Second star:", first_star());
    second_star();
}
