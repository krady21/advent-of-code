use std::io::Result;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn parse(file: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();
    let mut players = file.split("\n\n");

    let cards = players
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|s| s.parse::<usize>().ok());
    p1.extend(cards);
    let cards = players
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|s| s.parse::<usize>().ok());
    p2.extend(cards);
    (p1, p2)
}

fn play_game(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> Result<bool> {
    let mut winner;
    let (mut h1, mut h2) = (HashSet::new(), HashSet::new());

    loop {
        if !h1.insert(p1.clone()) && !h2.insert(p2.clone()) {
            return Ok(true);
        }

        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        winner = if c1 <= p1.len() && c2 <= p2.len() {
            let mut p1 = p1.iter().take(c1).cloned().collect();
            let mut p2 = p2.iter().take(c2).cloned().collect();
            play_game(&mut p1, &mut p2)?
        } else {
            if c1 > c2 { true } else { false }
        };

        if winner {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }

        if p1.len() == 0 {
            winner = false;
            break;
        }

        if p2.len() == 0 {
            winner = true;
            break;
        }
    }
    Ok(winner)
}

fn task1(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    loop {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }

        if p1.len() == 0 {
            p1.extend(p2);
            break;
        }

        if p2.len() == 0 {
            break;
        }
    }

    p1.iter()
        .zip((1..=p1.len()).rev())
        .map(|(x, y)| x * y)
        .sum()
}


fn task2(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    if !play_game(&mut p1, &mut p2).unwrap(){
        p1 = p2;
    }

    p1.iter()
        .zip((1..=p1.len()).rev())
        .map(|(x, y)| x * y)
        .sum()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let (p1, p2) = parse(&file);
    let score1 = task1(p1.clone(), p2.clone());
    let score2 = task2(p1.clone(), p2.clone());

    println!("Task 1: The winning player's score is: {}", score1);
    println!("Task 2: The winning player's score is: {}", score2);
}
