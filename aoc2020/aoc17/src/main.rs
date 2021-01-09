use std::collections::HashSet;
use std::fs;

#[macro_use]
extern crate itertools;

fn active_neighbors(map: &HashSet<(isize, isize, isize)>, pos: (isize, isize, isize)) -> usize {
    iproduct!(
        (pos.0 - 1)..=(pos.0 + 1),
        (pos.1 - 1)..=(pos.1 + 1),
        (pos.2 - 1)..=(pos.2 + 1)
    )
    .filter(|&p| p != pos)
    .filter(|(i, j, k)| map.contains(&(*i, *j, *k)))
    .count()
}

fn active_neighbors2(map: &HashSet<(isize, isize, isize, isize)>, pos: (isize, isize, isize, isize)) -> usize {
    iproduct!(
        (pos.0 - 1)..=(pos.0 + 1),
        (pos.1 - 1)..=(pos.1 + 1),
        (pos.2 - 1)..=(pos.2 + 1),
        (pos.3 - 1)..=(pos.3 + 1)
    )
    .filter(|&p| p != pos)
    .filter(|(i, j, k, l)| map.contains(&(*i, *j, *k, *l)))
    .count()
}

fn task1(input: &str, cycles: usize) -> usize {
    let mut map: HashSet<(isize, isize, isize)> = HashSet::new();
    let n: isize = input.lines().count() as isize;

    for (x, l) in input.lines().rev().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == '#' {
                map.insert((x as isize, y as isize, 0));
            }
        }
    }

    for i in 1..=cycles {
        let mut changed: Vec<((isize, isize, isize), bool)> = Vec::new();
        let cubes = iproduct!(
            (0 - i as isize)..=(n + i as isize),
            (0 - i as isize)..=(n + i as isize),
            (0 - i as isize)..=(0 + i as isize)
        )
        .map(|(i, j, k)| (i as isize, j as isize, k as isize));

        for cube in cubes {
            let active = active_neighbors(&map, cube);
            if map.contains(&cube) {
                if active != 2 && active != 3 {
                    changed.push((cube, false));
                }
            } else {
                if active == 3 {
                    changed.push((cube, true));
                }
            }
        }

        for entry in changed {
            if entry.1 {
                map.insert(entry.0);
            } else {
                map.remove(&entry.0);
            }
        }
    }

    map.len()
}

fn task2(input: &str, cycles: usize) -> usize {
    let mut map: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let n: isize = input.lines().count() as isize;

    for (x, l) in input.lines().rev().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == '#' {
                map.insert((x as isize, y as isize, 0, 0));
            }
        }
    }

    for i in 1..=cycles {
        let mut changed: Vec<((isize, isize, isize, isize), bool)> = Vec::new();
        let cubes = iproduct!(
            (0 - i as isize)..=(n + i as isize),
            (0 - i as isize)..=(n + i as isize),
            (0 - i as isize)..=(0 + i as isize),
            (0 - i as isize)..=(0 + i as isize)
        )
        .map(|(i, j, k, l)| (i as isize, j as isize, k as isize, l as isize));

        for cube in cubes {
            let active = active_neighbors2(&map, cube);
            if map.contains(&cube) {
                if active != 2 && active != 3 {
                    changed.push((cube, false));
                }
            } else {
                if active == 3 {
                    changed.push((cube, true));
                }
            }
        }

        for entry in changed {
            if entry.1 {
                map.insert(entry.0);
            } else {
                map.remove(&entry.0);
            }
        }
    }

    map.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read from file");
    let active1 = task1(&input, 6);
    let active2 = task2(&input, 6);

    println!("Task 1: Cubes left in the active state: {}", active1);
    println!("Task 2: Cubes left in the active state: {}", active2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1() {
        let input = ".#.
..#
###";

        assert_eq!(112, task1(&input, 6));
    }

}
