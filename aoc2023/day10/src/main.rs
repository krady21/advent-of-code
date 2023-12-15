use std::fs;

type Pos = (usize, usize);

fn find_start(grid: &Vec<Vec<char>>) -> Option<Pos> {
    for (i, _) in grid.iter().enumerate() {
        for (j, _) in grid[i].iter().enumerate() {
            if grid[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_one_connected(grid: &Vec<Vec<char>>, curr: Pos) -> Pos {
    if (curr.0 > 0) && ['|', '7', 'F'].contains(&grid[curr.0 - 1][curr.1]) {
        (curr.0 - 1, curr.1)
    } else if (curr.0 < grid.len() - 1) && ['|', 'J', 'L'].contains(&grid[curr.0 + 1][curr.1]) {
        (curr.0 + 1, curr.1)
    } else if (curr.1 < grid[0].len() - 1) && ['-', 'J', '7'].contains(&grid[curr.0][curr.1 + 1]) {
        (curr.0, curr.1 + 1)
    } else if (curr.1 > 0) && ['-', 'F', 'L'].contains(&grid[curr.0][curr.1 - 1]) {
        (curr.0, curr.1 + 1)
    } else {
        panic!("wut")
    }
}

fn find_next(grid: &Vec<Vec<char>>, curr: Pos, prev: Pos) -> Pos {
    match grid[curr.0][curr.1] {
        '|' => {
            if prev.0 < curr.0 {
                (curr.0 + 1, curr.1)
            } else {
                (curr.0 - 1, curr.1)
            }
        }
        '-' => {
            if prev.1 < curr.1 {
                (curr.0, curr.1 + 1)
            } else {
                (curr.0, curr.1 - 1)
            }
        }
        'L' => {
            if curr.1 < prev.1 {
                (curr.0 - 1, curr.1)
            } else {
                (curr.0, curr.1 + 1)
            }
        }
        'J' => {
            if prev.1 < curr.1 {
                (curr.0 - 1, curr.1)
            } else {
                (curr.0, curr.1 - 1)
            }
        }
        '7' => {
            if prev.1 < curr.1 {
                (curr.0 + 1, curr.1)
            } else {
                (curr.0, curr.1 - 1)
            }
        }
        'F' => {
            if curr.1 < prev.1 {
                (curr.0 + 1, curr.1)
            } else {
                (curr.0, curr.1 + 1)
            }
        }
        'S' => curr,
        _ => panic!("wut"),
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();

    let start = find_start(&grid).unwrap();

    let mut iter = find_one_connected(&grid, start);
    let mut prev = start;
    let mut count = 0;

    let mut loop_indexes = Vec::new();
    loop_indexes.push(start);
    loop_indexes.push(iter);

    loop {
        count += 1;

        let next = find_next(&grid, iter, prev);
        if next.0 == start.0 && next.1 == start.1 {
            break;
        }
        loop_indexes.push(next);

        prev = iter;
        iter = next;
    }

    // shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
    // determine the area of a simple polygon whose vertices are described by their Cartesian coordinates in the plane
    let mut windows = loop_indexes
        .windows(2)
        .map(|w| (w[0], w[1]))
        .collect::<Vec<(Pos, Pos)>>();

    // add (xn, x0) tuple
    windows.push((loop_indexes[loop_indexes.len() - 1], loop_indexes[0]));

    // sum of the determinants of each pair of cartesian coordinates.
    let area: f64 = (windows
        .iter()
        .map(|(a, b)| (a.0 as f64 * b.1 as f64) - (a.1 as f64 * b.0 as f64))
        .sum::<f64>()
        / 2_f64)
        .abs();

    let boundary_points = (count + 1) as f64;

    // Picks theorem: i = A - b/2 + h - 1 where i = number of points inside loop, b number of
    // points on the boundary, h = number of holes (0)
    let inner_points = area - boundary_points / 2_f64 + 1_f64;

    let part1 = (count + 1) / 2;
    let part2 = inner_points;

    println!("Day 10 part 1 is {part1}");
    println!("Day 10 part 1 is {part2}");
}
