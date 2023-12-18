use std::cmp;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let games = file
        .lines()
        .map(|l| l.trim().split(": ").last().unwrap())
        .map(|l| {
            l.split(";").map(|set_string| {
                let mut rgb = (0, 0, 0);

                set_string.trim().split(",").for_each(|balls| {
                    let count_and_color: Vec<&str> = balls.trim().split(" ").collect();

                    let count: usize = count_and_color[0].parse().unwrap();
                    let color = count_and_color[1];

                    if color == "red" {
                        rgb.0 = count;
                    } else if color == "green" {
                        rgb.1 = count;
                    } else {
                        rgb.2 = count;
                    }
                });

                return rgb;
            })
        });

    let part1: usize = games
        .clone()
        .enumerate()
        .filter(|(_, sets)| {
            sets.clone()
                .all(|rgb| rgb.0 <= 12 && rgb.1 <= 13 && rgb.2 <= 14)
        })
        .map(|(i, _)| i + 1)
        .sum();

    let part2: usize = games
        .map(|sets| {
            sets.fold((0, 0, 0), |max, x| {
                (cmp::max(max.0, x.0), cmp::max(max.1, x.1), cmp::max(max.2, x.2))
            })
        })
        .map(|max| max.0 * max.1 * max.2)
        .sum();

    println!("Day 2 part 1 is {part1}");
    println!("Day 2 part 2 is {part2}");
}
