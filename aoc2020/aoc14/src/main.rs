use std::collections::HashMap;
use std::fs;

fn apply_mask1(value: usize, mask: &str) -> usize {
    mask.chars().rev().enumerate().fold(value, |new, (i, c)| {
        if c == '0' {
            new & !(1 << i)
        } else if c == '1' {
            new | (1 << i)
        } else {
            new
        }
    })
}

fn apply_mask2(address: usize, mask: &str) -> Vec<usize> {
    let mut new = address;
    let mut xvec: Vec<usize> = Vec::new();

    for (i, c) in mask.chars().rev().enumerate() {
        if c == '1' {
            new |= 1 << i;
        } else if c == 'X' {
            xvec.push(i);
        }
    }

    (0..2usize.pow(xvec.len() as u32))
        .map(|i| format!("{:036b}", i))
        .map(|mask| {
            mask.chars()
                .skip(mask.len() - xvec.len())
                .enumerate()
                .fold(new, |acc, (i, c)| {
                    if c == '0' {
                        acc & !(1 << xvec[i])
                    } else if c == '1' {
                        acc | (1 << xvec[i])
                    } else {
                        acc
                    }
                })
        })
        .collect()
}

fn task1(input: &str) -> usize {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        if line[0..4].eq("mask") {
            mask = line.split_whitespace().last().unwrap();
        } else if line[0..3].eq("mem") {
            let string: String = line
                .chars()
                .filter(|c| c.is_digit(10) || c.is_whitespace())
                .map(|c| c.to_string())
                .collect();

            let mut numbers = string.split_whitespace();

            let (location, value) = (
                numbers.next().unwrap().parse::<usize>().unwrap(),
                numbers.next().unwrap().parse::<usize>().unwrap(),
            );

            memory.insert(location, apply_mask1(value, mask));
        }
    }

    memory.values().sum()
}

fn task2(input: &str) -> usize {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        if line[0..4].eq("mask") {
            mask = line.split_whitespace().last().unwrap();
        } else if line[0..3].eq("mem") {
            let string: String = line
                .chars()
                .filter(|c| c.is_digit(10) || c.is_whitespace())
                .map(|c| c.to_string())
                .collect();

            let mut numbers = string.split_whitespace();

            let (location, value) = (
                numbers.next().unwrap().parse::<usize>().unwrap(),
                numbers.next().unwrap().parse::<usize>().unwrap(),
            );

            let locations = apply_mask2(location, mask);
            for address in locations {
                memory.insert(address, value);
            }
        }
    }

    memory.values().sum()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");

    let sum1 = task1(&file);
    let sum2 = task2(&file);

    println!("Task 1: The sum of all the values is: {}", sum1);
    println!("Task 2: The sum of all the values is: {}", sum2);
}
