use std::collections::BTreeMap;
use std::fs::read_to_string;

fn hash(s: &str) -> u32 {
    s.chars().map(|c| c as u32).fold(0, |mut acc, x| {
        acc += x;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn main() {
    let file = read_to_string("input.txt").unwrap();

    let lenses = file.trim().split(',').into_iter();

    let results = lenses.clone().map(|s| hash(s));
    let part1: u32 = results.sum();

    let mut labels: BTreeMap<u32, Vec<&str>> = BTreeMap::new();

    for lens in lenses.clone() {
        let label = lens.split(|c| c == '-' || c == '=').next().unwrap();
        let op = lens
            .chars()
            .filter(|c| *c == '-' || *c == '=')
            .next()
            .unwrap();

        let label_idx = hash(label);
        let entry = labels.entry(label_idx);

        if op == '=' {
            entry
                .and_modify(|v| {
                    match v.iter().position(|&x| x.starts_with(label)) {
                        Some(i) => {
                            v[i] = lens;
                        }
                        None => {
                            v.push(lens);
                        }
                    };
                })
                .or_insert(vec![lens]);
        } else {
            entry.and_modify(|v| {
                if let Some(i) = v.iter().position(|&x| x.starts_with(label)) {
                    v.remove(i);
                }
            });
        }
    }

    let focusing_powers = labels.iter().flat_map(|(box_id, v)| {
        v.iter().enumerate().map(|(i, &s)| {
            (
                *box_id + 1,
                i as u32 + 1,
                s.chars().last().unwrap().to_digit(10).unwrap(),
            )
        })
    });

    let part2: u32 = focusing_powers
        .map(|(box_id, slot, focal_len)| box_id * slot * focal_len)
        .sum();

    println!("Day 15 part 1 is {part1}");
    println!("Day 15 part 2 is {part2}");
}
