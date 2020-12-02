use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Clean this mess:
// - do IO separately
// - use filter and count instead of map and sum

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(&file);

    let valid_passwords_1: i64 = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let split: Vec<&str> = line.split(' ').collect();
            let values: Vec<&str> = split[0].split('-').collect();

            let min = values[0].parse::<i64>().unwrap();
            let max = values[1].parse::<i64>().unwrap();
            let chr = split[1].chars().nth(0).unwrap();

            let mut count = 0;
            for c in split[2].chars() {
                if c == chr {
                    count += 1;
                }
            }

            if count >= min && count <= max {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Task 1: Number of valid passwords: {}", valid_passwords_1);

    let file = File::open("input.txt")?;
    let file = BufReader::new(&file);
    let valid_passwords_2: i64 = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let split: Vec<&str> = line.split(' ').collect();
            let values: Vec<&str> = split[0].split('-').collect();

            let min = values[0].parse::<i64>().unwrap();
            let max = values[1].parse::<i64>().unwrap();
            let chr = split[1].chars().nth(0).unwrap();

            let copy = String::from(split[2]);
            let first_position = copy.chars().nth((min - 1) as usize).unwrap();
            let copy = String::from(split[2]);
            let second_position = copy.chars().nth((max - 1) as usize).unwrap();

            if (first_position == chr && second_position != chr)
                || (first_position != chr && second_position == chr)
            {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Task 2: Number of valid passwords: {}", valid_passwords_2);

    Ok(())
}
