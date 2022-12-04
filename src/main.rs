use std::io;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

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
            _ => println!("Day {} not implemented yet", day),
        },
        _ => println!("Invalid day, must be an integer between 1 and 25"),
    }
}
