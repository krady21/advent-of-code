use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! slope {
    ($iter:expr, $right:expr, $down:expr) => {
        $iter.filter(|(i, vector)| {
            if i % $down == 0 {
                let mut position = i * $right / $down;
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
        .count() as u64
    }
}

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

    let slope11 = slope!(trees.iter(), 1, 1);
    let slope31 = slope!(trees.iter(), 3, 1);
    let slope51 = slope!(trees.iter(), 5, 1);
    let slope71 = slope!(trees.iter(), 7, 1);
    let slope12 = slope!(trees.iter(), 1, 2);

    let product: u64 = slope11 * slope31 * slope51 * slope71 * slope12;

    println!("{} {} {} {} {}", slope11, slope31, slope51, slope71, slope12);
    println!("Task 1: The number of trees is {}", slope31);
    println!("Task 2: The product of the number trees is {}", product);

    Ok(())

}
