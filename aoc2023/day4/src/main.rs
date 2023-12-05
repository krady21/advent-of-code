use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let to_set = |v: &str| {
        let set: HashSet<usize> = v
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        set
    };

    let input = file
        .lines()
        .map(|l| l.split(": ").collect::<Vec<&str>>()[1])
        .map(|l| l.split(" | ").collect::<Vec<&str>>())
        .map(|v| (to_set(v[0]), to_set(v[1])))
        .map(|(win, my)| win.intersection(&my).clone().collect::<Vec<&usize>>().len());

    let part1: usize = input
        .clone()
        .map(|winners| {
            if winners > 0 {
                (2_usize).pow((winners - 1) as u32)
            } else {
                0_usize
            }
        })
        .sum();

    let mut map = HashMap::new();
    for (i, winners) in input.clone().enumerate() {
        map.entry(i + 1).or_insert(1);
        let current_card = *map.get(&(i + 1)).unwrap();
        for copy in i + 2..=i + 1 + winners {
            map.entry(copy)
                .and_modify(|e| *e += current_card)
                .or_insert(1 + current_card);
        }
    }

    let part2: usize = map.values().sum();

    println!("Day 4 part 1 is {part1}");
    println!("Day 4 part 2 is {part2}");
}
