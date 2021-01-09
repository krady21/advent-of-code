use std::cmp::Ordering;
use std::fs;

// Part 2 was inspired by an answer from reddit, as i can't seem to wrap my head around the chinese
// remaineder theorem.
// TODO: Understand CRT

fn parse1(file: &str) -> (usize, Vec<usize>) {
    let mut lines = file.lines();
    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let vector: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&c| !c.eq("x"))
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    (timestamp, vector)
}

fn task1(input: &(usize, Vec<usize>)) -> usize {
    let mut timestamp = input.0;
    loop {
        for taxi in &input.1 {
            if timestamp % taxi == 0 {
                return taxi * (timestamp - input.0);
            }
        }
        timestamp += 1;
    }
}

fn parse2(file: &str) -> Vec<(usize, usize)> {
    file.lines()
        .last()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, c)| !c.eq(&"x"))
        .map(|(i, x)| (i as usize, x.parse::<usize>().unwrap()))
        .collect::<Vec<(usize, usize)>>()
}

fn task2(input: &Vec<(usize, usize)>) -> usize {
    input
        .iter()
        .map(|(i, x)| (x - (i + x - 1) % x - 1, x))
        .fold((0, 1), |(r1, q1), (r2, q2)| crt(r1, q1, r2, *q2))
        .0 as usize
}

fn crt(r1: usize, q1: usize, r2: usize, q2: usize) -> (usize, usize) {
    let mut a = r1;
    let mut b = r2;
    let q = q1 * q2 / gcd(q1, q2);
    loop {
        match a.cmp(&b) {
            Ordering::Less => a += ((b - a + q1 - 1) / q1) * q1,
            Ordering::Equal => return (a, q),
            Ordering::Greater => b += ((a - b + q2 - 1) / q2) * q2,
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut x, mut y) = if a < b { (a, b) } else { (b, a) };
    while x != 0 {
        let tmp = x;
        x = y % x;
        y = tmp;
    }
    y
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let input1 = parse1(&file);
    let input2 = parse2(&file);

    let magic = task1(&input1);
    let time = task2(&input2);

    println!("Task 1: The product is: {}", magic);
    println!("Task 2: The earliest timestamp is: {}", time);
}
