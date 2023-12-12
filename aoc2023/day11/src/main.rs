use itertools::Itertools;
use num::{BigInt, FromPrimitive};
use std::fs;

type Pos = (BigInt, BigInt);

fn manhattan_distance(p1: Pos, p2: Pos) -> BigInt {
    let x_delta = if p1.0 > p2.0 {
        p1.0 - p2.0
    } else {
        p2.0 - p1.0
    };
    let y_delta = if p1.1 > p2.1 {
        p1.1 - p2.1
    } else {
        p2.1 - p1.1
    };
    x_delta + y_delta
}

fn adjust_coordinate(v: BigInt, double: &Vec<usize>, replacement_count: usize) -> BigInt {
    let empty_before = BigInt::from_usize(
        double
            .iter()
            .filter(|&&d| BigInt::from_usize(d).unwrap() < v)
            .count(),
    )
    .unwrap();

    v + (replacement_count - 1) * empty_before
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let double_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let double_cols = (0..cols)
        .map(|i| (0..rows).map(|j| grid[j][i]).collect::<Vec<char>>())
        .enumerate()
        .filter(|(_, col)| !col.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let galaxies = (0..rows)
        .flat_map(|i| (0..cols).map(move |j| (i, j)))
        .filter(|(i, j)| grid[*i][*j] == '#')
        .map(|(i, j)| {
            (
                BigInt::from_usize(i).unwrap(),
                BigInt::from_usize(j).unwrap(),
            )
        });

    let calculate_distances = |replace_with| -> BigInt {
        galaxies
            .clone()
            .map(|g| {
                (
                    adjust_coordinate(g.0, &double_rows, replace_with),
                    adjust_coordinate(g.1, &double_cols, replace_with),
                )
            })
            .combinations(2)
            .map(|v| manhattan_distance(v[0].clone(), v[1].clone()))
            .sum()
    };

    let part1 = calculate_distances(2);
    let part2 = calculate_distances(1000000);

    println!("Day 11 part 1 is {part1}");
    println!("Day 11 part 2 is {part2}");
}
