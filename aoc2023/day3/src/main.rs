use std::collections::HashMap;
use std::fs;

fn get_neighbors(input: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<&char> {
    let i_range = if i > 0 { i - 1..=i + 1 } else { 0..=1 };
    let j_range = if j > 0 { j - 1..=j + 1 } else { 0..=1 };

    let neighbors: Vec<&char> = i_range
        .filter_map(|i| input.get(i))
        .flat_map(|row| j_range.clone().filter_map(|j| row.get(j)))
        .collect();

    neighbors
}

fn find_neighbor_star(input: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let i_range = if i > 0 { i - 1..=i + 1 } else { 0..=1 };
    let j_range = if j > 0 { j - 1..=j + 1 } else { 0..=1 };

    for i in i_range {
        if let Some(row) = input.get(i) {
            for j in j_range.clone() {
                if let Some(val) = row.get(j) {
                    if *val == '*' {
                        return Some((i, j));
                    }
                }
            }
        }
    }

    None
}

fn has_symbol_adjacent(input: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let neighbors = get_neighbors(input, i, j);

    neighbors.iter().any(|&n| !n.is_digit(10) && *n != '.')
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for (i, _) in input.iter().enumerate() {
        let mut acc = 0;
        let mut is_part = false;

        for (j, _) in input[i].iter().enumerate() {
            let c = input[i][j];

            if c.is_digit(10) {
                acc = acc * 10 + c.to_digit(10).unwrap();
                is_part = is_part || has_symbol_adjacent(&input, i, j);
            }

            if !c.is_digit(10) || j == input[0].len() - 1 {
                if is_part {
                    sum += acc;
                }
                acc = 0;
                is_part = false;
            }
        }
    }
    sum
}

fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (i, _) in input.iter().enumerate() {
        let mut acc = 0;
        let mut star: Option<(usize, usize)> = None;
        for (j, _) in input[i].iter().enumerate() {
            let c = input[i][j];

            if c.is_digit(10) {
                acc = acc * 10 + c.to_digit(10).unwrap();
                star = star.or(find_neighbor_star(&input, i, j));
            }

            if !c.is_digit(10) || j == input[0].len() - 1 {
                if let Some(star_pos) = star {
                    let mut v: Vec<usize> = match map.get(&star_pos) {
                        Some(arr) => arr.clone(),
                        None => Vec::new(),
                    };
                    v.push(acc as usize);
                    map.insert(star_pos, v);
                }
                acc = 0;
                star = None
            }
        }
    }

    return map
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let input: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Day 3 part 1 is {p1}");
    println!("Day 3 part 2 is {p2}");
}
