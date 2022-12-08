use std::io;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

fn main() {
    let mut day_str = String::new();
    println!("Which day do you want to run?");
    io::stdin()
        .read_line(&mut day_str)
        .expect("Failed to read line");

    match day_str.trim().parse::<u32>() {
        Ok(day) if day > 0 && day < 26 => match day {
            1 => day1::day1(),
            2 => day2::day2(),
            3 => day3::day3(),
            4 => day4::day4(),
            5 => day5::day5(),
            6 => day6::day6(),
            7 => day7::day7(),
            8 => day8::day8(),
            _ => println!("Day {} not implemented yet", day),
        },
        _ => println!("Invalid day, must be an integer between 1 and 25"),
    }
}
