use std::collections::{HashMap, VecDeque};
use std::fs;

enum Rule {
    Char(char),
    Standard(Vec<usize>),
}

fn parse(rules: &str) -> HashMap<usize, Vec<Rule>> {
    let mut map: HashMap<usize, Vec<Rule>> = HashMap::new();
    for line in rules.lines() {
        let mut iter = line.split(": ");

        let first = iter.next().unwrap().parse().unwrap();
        let second = iter.next().unwrap();
        let mut v = Vec::new();

        if second.contains("\"") {
            v.push(Rule::Char(second.chars().skip(1).next().unwrap()));
        } else if second.contains("|") {
            for part in second.split(" | ") {
                v.push(Rule::Standard(
                    part.split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect(),
                ));
            }
        } else {
            v.push(Rule::Standard(
                second
                    .split_ascii_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            ));
        }

        let second = v;
        map.insert(first, second);
    }

    map
}

fn matches(rules: &HashMap<usize, Vec<Rule>>, seq: &[char], mut dequeue: VecDeque<usize>) -> bool {
    match (seq.len(), dequeue.len()) {
        (0, 0) => return true,
        (_, 0) => return false,
        (0, _) => return false,
        _ => {}
    }

    let index = dequeue.pop_front().unwrap();
    let possibilities = &rules[&index];

    for rule in possibilities {
        let result = match rule {
            Rule::Char(c) => {
                if c == &seq[0] {
                    matches(&rules, &seq[1..], dequeue.clone())
                } else {
                    false
                }
            }
            Rule::Standard(expanded) => {
                if expanded.len() > seq.len() {
                    false
                } else {
                    matches(
                        &rules,
                        &seq,
                        expanded.iter().chain(dequeue.iter()).copied().collect(),
                    )
                }
            }
        };
        if result {
            return true;
        }
    }
    false
}

fn modify(rules: &mut HashMap<usize, Vec<Rule>>) {
    let r8 = vec![Rule::Standard(vec![42]), Rule::Standard(vec![42, 8])];
    let r11 = vec![Rule::Standard(vec![42, 31]), Rule::Standard(vec![42, 11, 31])];

    rules.insert(8, r8);
    rules.insert(11, r11);
}

fn task1(file: &str) -> usize {
    let split: Vec<&str> = file.splitn(2, "\n\n").collect::<Vec<&str>>();

    let rules = split[0];
    let messages = split[1];

    let mut dequeue = VecDeque::new();
    dequeue.push_back(0);

    let map = parse(&rules);
    messages
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|v| matches(&map, &v, dequeue.clone()))
        .filter(|&x| x)
        .count()
}

fn task2(file: &str) -> usize {
    let split: Vec<&str> = file.splitn(2, "\n\n").collect::<Vec<&str>>();

    let rules = split[0];
    let messages = split[1];

    let mut dequeue = VecDeque::new();
    dequeue.push_back(0);

    let mut map = parse(&rules);
    modify(&mut map);

    messages
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|v| matches(&map, &v, dequeue.clone()))
        .filter(|&x| x)
        .count()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");

    let sum1 = task1(&file);
    let sum2 = task2(&file);

    println!("Task 1: The sum is: {}", sum1);
    println!("Task 2: The sum is: {}", sum2);
}
