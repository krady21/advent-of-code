use std::fs;

// one, two, three, four, five, six, seven, eight, and nine
fn is_number_spelled(string: &str) -> Option<u32> {
    if string.starts_with("one") {
        return Some(1);
    } else if string.starts_with("two") {
        return Some(2);
    } else if string.starts_with("three") {
        return Some(3);
    } else if string.starts_with("four") {
        return Some(4);
    } else if string.starts_with("five") {
        return Some(5);
    } else if string.starts_with("six") {
        return Some(6);
    } else if string.starts_with("seven") {
        return Some(7);
    } else if string.starts_with("eight") {
        return Some(8);
    } else if string.starts_with("nine") {
        return Some(9);
    } else {
        return None;
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let part1: u32 = file
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|x| x.to_digit(10)).peekable();
            let first = *digits.peek().unwrap();
            let last = digits.last().unwrap();
            first * 10 + last
        })
        .sum();

    let part2: u32 = file.lines().map(|line| {
        let mut digits = line.char_indices()
            .filter_map(|(i, c)| c.to_digit(10).or(is_number_spelled(&line[i..]))).peekable();
        let first = *digits.peek().unwrap();
        let last = digits.last().unwrap();
        first * 10 + last
    }).sum();

    println!("Day 1 part 1 is {part1}");
    println!("Day 1 part 2 is {part2}");
}
