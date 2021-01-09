use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(&file);
    let mut set: HashSet<i64> = HashSet::new();

    println!("Part 1");

    for line in file.lines() {
        let first = line.unwrap().parse::<i64>().unwrap();
        let second = 2020 - first;

        set.insert(first);
        if set.contains(&second) {
            println!(
                "first entry: {}\nsecond entry: {}\nproduct: {}",
                first,
                second,
                first * second
            );
        }
    }
    println!("\n\n\nPart 2");

    let file1 = File::open("input.txt")?;
    let file1 = BufReader::new(&file1);
    let input: Vec<i64> = file1
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect();

    'outer: for first in &input {
        let target: i64 = 2020 - first;

        for second in &input {
            let third: i64 = target - second;

            if set.contains(&third) {
                println!(
                    "first entry: {}\nsecond entry: {}\nthird entry: {}\nproduct: {}",
                    first,
                    second,
                    third,
                    first * second * third
                );

                break 'outer;
            }
        }
    }

    Ok(())
}
