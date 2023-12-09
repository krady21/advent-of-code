use std::{collections::HashMap, fs};

// blatantly stolen from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut blocks = file.split("\n\n");
    let steps_block = blocks.next().unwrap().trim();
    let mut steps = steps_block.chars().cycle();

    let nodes: HashMap<&str, (&str, &str)> = blocks
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|s| (&s[0..=2], &s[7..=9], &s[12..=14]))
        .fold(HashMap::new(), |mut acc, t| {
            acc.insert(t.0, (t.1, t.2));
            acc
        });

    let mut iter = "AAA";
    let mut part1 = 0;
    while iter != "ZZZ" {
        part1 += 1;
        let step = steps.next().unwrap();
        iter = if step == 'L' {
            nodes[iter].0
        } else {
            nodes[iter].1
        }
    }

    let min_steps_arr = nodes
        .keys()
        .filter(|node| node.chars().last().unwrap() == 'A')
        .map(|&node| {
            let mut iter = node.clone();
            let mut min_steps = 0;
            let mut steps = steps_block.chars().cycle();

            while iter.chars().last().unwrap() != 'Z' {
                min_steps += 1;
                let step = steps.next().unwrap();
                iter = if step == 'L' {
                    nodes[iter].0
                } else {
                    nodes[iter].1
                }
            }
            min_steps
        })
        .collect::<Vec<usize>>();

    let part2 = lcm(&min_steps_arr);

    println!("Day 8 part 1 is {part1}");
    println!("Day 8 part 2 is {part2}");
}
