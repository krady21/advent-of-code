use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type Pos = (usize, usize);

#[derive(Copy, Clone)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

fn move_rock(input: &Vec<Vec<char>>, pos: Pos, direction: Cardinal) -> Pos {
    let nrows = input.len();
    let ncols = input[0].len();

    let range: Box<dyn Iterator<Item = Pos>> = match direction {
        Cardinal::North => Box::new((0..pos.0).rev().map(|i| (i, pos.1))),
        Cardinal::South => Box::new((pos.0 + 1..nrows).map(|i| (i, pos.1))),
        Cardinal::West => Box::new((0..pos.1).rev().map(|j| (pos.0, j))),
        Cardinal::East => Box::new((pos.1 + 1..ncols).map(|j| (pos.0, j))),
    };

    range
        .take_while(|&(i, j)| input[i][j] == '.')
        .last()
        .unwrap_or(pos)
}

fn spin(mut input: Vec<Vec<char>>, direction: Cardinal) -> Vec<Vec<char>> {
    let nrows = input.len();
    let ncols = input[0].len();

    let indexes: Vec<Pos> = match direction {
        Cardinal::North => (0..nrows)
            .flat_map(|i| (0..ncols).map(move |j| (i, j)))
            .collect(),
        Cardinal::South => (0..nrows)
            .rev()
            .flat_map(|i| (0..ncols).map(move |j| (i, j)))
            .collect(),
        Cardinal::East => (0..ncols)
            .rev()
            .flat_map(|j| (0..nrows).map(move |i| (i, j)))
            .collect(),
        Cardinal::West => (0..ncols)
            .flat_map(|j| (0..nrows).map(move |i| (i, j)))
            .collect(),
    };

    for (i, j) in indexes {
        if input[i][j] == 'O' {
            let (newi, newj) = move_rock(&input, (i, j), direction);
            input[i][j] = '.';
            input[newi][newj] = 'O';
        }
    }
    input
}

fn cycle(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north = spin(input, Cardinal::North);
    let west = spin(north, Cardinal::West);
    let south = spin(west, Cardinal::South);
    let east = spin(south, Cardinal::East);
    east
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut input: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();

    let p1 = spin(input.clone(), Cardinal::North);

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    let mut set: HashMap<u64, usize> = HashMap::new();

    let cycles = 1000000000;
    for i in 1..=cycles {
        input = cycle(input);
        let hash = calculate_hash(&input);

        if !set.contains_key(&hash) {
            set.insert(hash, i);
        } else if (cycles - i) % ((set[&hash] as isize - i as isize).abs() as usize) == 0 {
            break;
        }
    }

    let nrows = input.len();
    let part1: usize = p1
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (nrows - i))
        .sum();

    let part2: usize = input
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (nrows - i))
        .sum();

    println!("Day 14 part 1 is {part1}");
    println!("Day 14 part 2 is {part2}");
}
