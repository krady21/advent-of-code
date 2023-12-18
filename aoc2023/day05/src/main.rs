use std::cmp::{max, min};
use std::fs;

type R = (usize, usize);

trait Set {
    fn intersection(&self, other: &R) -> Option<R>;
    fn difference(&self, other: &R) -> Vec<R>;
}

impl Set for R {
    fn intersection(&self, other: &R) -> Option<R> {
        let start = max(self.0, other.0);
        let end = min(self.1, other.1);

        if end > start {
            Some((start, end))
        } else {
            None
        }
    }

    fn difference(&self, other: &R) -> Vec<R> {
        if self.1 < other.0 || other.1 < self.0 {
            vec![self.clone()]
        } else if other.0 <= self.0 && self.1 <= other.1 {
            vec![]
        } else if self.0 < other.0 && other.1 < self.1 {
            vec![(self.0, other.0 - 1), (other.1 + 1, self.1)]
        } else if self.0 < other.0 {
            vec![(self.0, other.0 - 1)]
        } else {
            vec![(other.1 + 1, self.1)]
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let blocks: Vec<&str> = file.split("\n\n").collect();

    let mut seeds = blocks[0]
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let seeds2 = seeds.clone();

    let maps = blocks[1..].iter().map(|&b| {
        b.trim()
            .lines()
            .skip(1)
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|s| s.trim().parse().ok())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>()
    });

    for seed in seeds.iter_mut() {
        for map in maps.clone() {
            for line in map {
                let source_start = line[1];
                let source_end = line[1] + line[2];

                let diff = if line[1] > line[0] {
                    line[1] - line[0]
                } else {
                    line[0] - line[1]
                };

                if source_start <= *seed && *seed <= source_end {
                    *seed = if line[1] > line[0] {
                        *seed - diff
                    } else {
                        *seed + diff
                    };
                    break;
                }
            }
        }
    }

    let part1 = seeds.iter().min().unwrap();

    let mut start_ranges = seeds2
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect::<Vec<R>>();

    for map in maps.clone() {
        let mut already_mapped: Vec<R> = vec![];

        for line in map {
            let source_start = line[1];
            let source_end = line[1] + line[2];
            let source_interval: R = (source_start, source_end);

            let diff = if line[1] > line[0] {
                line[1] - line[0]
            } else {
                line[0] - line[1]
            };

            // Split each range into multiples ranges (the ranges that remain the same) and flatten
            // them.
            start_ranges = start_ranges
                .iter()
                .flat_map(|r| {
                    let has_intersection = r.intersection(&source_interval);
                    let difference = r.difference(&source_interval);

                    if let Some(intersection) = has_intersection {
                        let dest_interval = if line[1] > line[0] {
                            (intersection.0 - diff, intersection.1 - diff)
                        } else {
                            (intersection.0 + diff, intersection.1 + diff)
                        };
                        // Add the mapped interval to a separate vector to avoid matching on more
                        // than one rule from a map. TLDR: Allow only one match.
                        already_mapped.push(dest_interval);
                    }

                    difference
                })
                .collect();

            if start_ranges.is_empty() {
                break;
            }
        }

        start_ranges.extend(already_mapped.clone());
    }

    let part2 = start_ranges.iter().map(|r| r.0).min().unwrap();

    println!("Day 5 part 1 is {part1}");
    println!("Day 5 part 2 in {part2}");
}
