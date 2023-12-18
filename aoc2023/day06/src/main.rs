use std::fs;

fn count_winners(x: (usize, usize)) -> usize {
    let time = x.0;
    let distance = x.1;

    (1..time)
        .map(|h| h * (time - h))
        .filter(|&result| result > distance)
        .count()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut lines = file
        .lines()
        .map(|l| l.split_whitespace().filter_map(|s| s.parse::<usize>().ok()));

    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let part1_winners = line1.zip(line2).map(|(x, y)| (x, y)).map(count_winners);

    let mut input2 = file.lines().map(|s| {
        s.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    });

    let time = input2.next().unwrap();
    let distance = input2.next().unwrap();

    let part2_winners = count_winners((time, distance));

    let part1: usize = part1_winners.product();
    let part2: usize = part2_winners;

    println!("Day 6 part1 is {part1}");
    println!("Day 6 part2 is {part2}");
}
