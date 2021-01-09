use std::collections::{HashMap, HashSet};
use std::fs;
use std::vec::IntoIter;

#[derive(Hash, PartialEq, Eq)]
struct Tile {
    id: usize,
    matrix: Vec<Vec<char>>,
}

impl Tile {
    fn new(id: usize, matrix: Vec<Vec<char>>) -> Self {
        Tile { id, matrix }
    }

    fn edges(&self) -> IntoIter<String> {
        let len = self.matrix.len();

        let mut e1 = String::new();
        let mut e2 = String::new();
        for i in 0..len {
            e1.push(self.matrix[i][0]);
            e2.push(self.matrix[i][len - 1]);
        }

        let e3 = self.matrix[0].iter().collect();
        let e4 = self.matrix[len - 1].iter().collect();

        vec![e1, e2, e3, e4].into_iter()
    }
}

fn parse(file: &str) -> HashSet<Tile> {
    let mut set: HashSet<Tile> = HashSet::new();
    for entry in file.split("\n\n") {
        let id = entry
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let matrix = entry.lines().skip(1).map(|l| l.chars().collect()).collect();
        let tile = Tile::new(id, matrix);
        set.insert(tile);
    }

    set
}

fn task1(file: &str) -> usize {
    let mut matches = HashMap::new();
    let tiles = parse(file);
    for tile in tiles {
        for edge in tile.edges() {
            matches
                .entry(edge.clone())
                .or_insert(Vec::new())
                .push(tile.id);
            matches
                .entry(edge.chars().rev().collect())
                .or_insert(Vec::new())
                .push(tile.id);
        }
    }

    let mut count = HashMap::new();
    for tids in matches.values().filter(|tids| tids.len() == 1) {
        *count.entry(tids[0]).or_insert(0) += 1;
    }

    count
        .iter()
        .filter(|&(_, x)| *x == 4)
        .map(|(id, _)| id)
        .product()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let product = task1(&file);
    println!(
        "Task 1: The product of the four corner tiles is: {}",
        product
    );
}
