use rayon::prelude::*;

use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Ray = (usize, usize, Direction);

fn simulate(grid: &Vec<Vec<char>>, ray: Ray) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut ennergized: Vec<Vec<HashSet<Direction>>> = vec![vec![HashSet::new(); m]; n];

    let update_ray_pos = |r: Ray| -> Option<Ray> {
        let dir = r.2;
        let (coordx, coordy) = match dir {
            Direction::Up => (r.0.checked_sub(1), Some(r.1)),
            Direction::Down => ((r.0.checked_add(1)), Some(r.1)),
            Direction::Left => (Some(r.0), r.1.checked_sub(1)),
            Direction::Right => (Some(r.0), r.1.checked_add(1)),
        };

        if let (Some(i), Some(j)) = (coordx, coordy) {
            if i < n && j < m {
                Some((i, j, dir))
            } else {
                None
            }
        } else {
            None
        }
    };

    let mut rays = vec![ray];
    while rays.len() != 0 {
        let mut new_rays = vec![];
        for ray in rays.iter() {
            ennergized[ray.0][ray.1].insert(ray.2);
        }

        for ray in rays.iter() {
            let (i, j) = (ray.0, ray.1);
            let encounters = grid[i][j];
            let dir = ray.2;

            if encounters == '.' {
                new_rays.push(update_ray_pos((i, j, dir)));
            } else if encounters == '/' {
                match dir {
                    Direction::Up => {
                        new_rays.push(update_ray_pos((i, j, Direction::Right)));
                    }
                    Direction::Down => {
                        new_rays.push(update_ray_pos((i, j, Direction::Left)));
                    }
                    Direction::Right => {
                        new_rays.push(update_ray_pos((i, j, Direction::Up)));
                    }
                    Direction::Left => {
                        new_rays.push(update_ray_pos((i, j, Direction::Down)));
                    }
                }
            } else if encounters == '\\' {
                match dir {
                    Direction::Up => {
                        new_rays.push(update_ray_pos((i, j, Direction::Left)));
                    }
                    Direction::Down => {
                        new_rays.push(update_ray_pos((i, j, Direction::Right)));
                    }
                    Direction::Right => {
                        new_rays.push(update_ray_pos((i, j, Direction::Down)));
                    }
                    Direction::Left => {
                        new_rays.push(update_ray_pos((i, j, Direction::Up)));
                    }
                }
            } else if encounters == '|' {
                match dir {
                    Direction::Up | Direction::Down => {
                        new_rays.push(update_ray_pos((i, j, dir)));
                    }
                    Direction::Left | Direction::Right => {
                        new_rays.push(update_ray_pos((i, j, Direction::Up)));
                        new_rays.push(update_ray_pos((i, j, Direction::Down)));
                    }
                }
            } else if encounters == '-' {
                match dir {
                    Direction::Left | Direction::Right => {
                        new_rays.push(update_ray_pos((i, j, dir)));
                    }
                    Direction::Up | Direction::Down => {
                        new_rays.push(update_ray_pos((i, j, Direction::Left)));
                        new_rays.push(update_ray_pos((i, j, Direction::Right)));
                    }
                }
            }
        }

        rays = new_rays
            .iter()
            .filter_map(|&x| x)
            .filter(|&x| !ennergized[x.0][x.1].contains(&x.2))
            .collect();
    }

    ennergized
        .iter()
        .flat_map(|l| l.iter().clone())
        .filter(|enn| !enn.is_empty())
        .count()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = file.trim().lines().map(|l| l.chars().collect()).collect();
    let n = grid.len();
    let m = grid[0].len();

    let ray_start1 = (0, 0, Direction::Right);

    let part1 = simulate(&grid, ray_start1);

    let first_row = (0..m).map(|j| (0, j, Direction::Down));
    let last_row = (0..m).map(|j| (n - 1, j, Direction::Up));
    let first_col = (0..n - 1).map(|i| (i, 0, Direction::Right));
    let last_col = (0..n - 1).map(|i| (i, m - 1, Direction::Left));

    let input = first_row
        .chain(last_row)
        .chain(first_col)
        .chain(last_col)
        .collect::<Vec<Ray>>();

    let part2: usize = input
        .par_iter()
        .map(|&ray| simulate(&grid, ray))
        .max()
        .unwrap();

    println!("Day 16 part 1 is {part1}");
    println!("Day 16 part 2 is {part2}");
}
