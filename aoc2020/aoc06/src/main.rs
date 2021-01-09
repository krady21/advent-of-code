use std::collections::{HashMap, HashSet};
use std::fs;

fn task1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut set: HashSet<char> = HashSet::new();
            for line in group.lines() {
                for c in line.chars() {
                    set.insert(c);
                }
            }
            set.iter().count()
        })
        .sum()
}

fn task2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut map: HashMap<char, usize> = HashMap::new();
            let persons = group.lines().count();
            for line in group.lines() {
                for c in line.chars() {
                    if map.contains_key(&c) {
                        map.insert(c, map.get(&c).unwrap() + 1);
                    } else {
                        map.insert(c, 1);
                    }
                }
            }
            map.iter().filter(|(_, &y)| y == persons).count()
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("input.txt")?;

    let sum1 = task1(&file);
    let sum2 = task2(&file);

    println!("Task 1: The sum is: {}", sum1);
    println!("Task 2: The sum is: {}", sum2);

    Ok(())
}
