use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Make a macro to avoid repeating code

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(&file);
    let trees: Vec<(usize, Vec<u64>)> = file
        .lines()
        .map(|line| {
            let vec: Vec<u64> = line
                .unwrap()
                .chars()
                .map(|c| {
                    if c == '#' {
                        return 1;
                    } else {
                        return 0;
                    }
                })
                .collect();
            vec
        })
        .enumerate()
        .collect();

    let slope11 = trees
        .iter()
        .filter(|(i, vector)| {
            let mut position = i * 1;
            while position > 30 {
                position -= 31;
            }

            if vector[position] == 1 {
                true
            } else {
                false
            }
        })
        .count() as u64;

    let slope31 = trees
        .iter()
        .filter(|(i, vector)| {
            let mut position = i * 3;
            while position > 30 {
                position -= 31;
            }

            if vector[position] == 1 {
                true
            } else {
                false
            }
        })
        .count() as u64;

    let slope51: u64 = trees
        .iter()
        .filter(|(i, vector)| {
            let mut position = i * 5;
            while position > 30 {
                position -= 31;
            }

            if vector[position] == 1 {
                true
            } else {
                false
            }
        })
        .count() as u64;

    let slope71 = trees
        .iter()
        .filter(|(i, vector)| {
            let mut position = i * 7;
            while position > 30 {
                position -= 31;
            }

            if vector[position] == 1 {
                true
            } else {
                false
            }
        })
        .count() as u64;

    let slope12 = trees
        .iter()
        .filter(|(i, vector)| {
            if i % 2 == 0 {
                let mut position = i / 2;
                while position > 30 {
                    position -= 31;
                }

                if vector[position] == 1 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count() as u64;

    let product: u64 = slope11 * slope31 * slope51 * slope71 * slope12;

    println!("{} {} {} {} {}", slope11, slope31, slope51, slope71, slope12);
    println!("Task 1: The number of trees is {}", slope31);
    println!("Task 2: The product of the number trees is {}", product);
    Ok(())
}
