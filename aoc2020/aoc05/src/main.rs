use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn task1(seats: &Vec<(u32, u32)>) -> Option<u32> {
    seats.iter().map(|(row, column)| row * 8 + column).max()
}

fn task2(seats: &Vec<(u32, u32)>) -> Option<u32> {
    let mut map: HashSet<u32> = HashSet::new();
    for seat in seats {
        let sid = seat.0 * 8 + seat.1;
        map.insert(sid);
    }

    for i in 1..=map.iter().max().unwrap().clone() {
        if !map.contains(&i) && map.contains(&(i - 1)) && map.contains(&(i + 1)) {
            return Some(i);
        }
    }

    None
}

fn util(input: &Vec<String>) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = Vec::with_capacity(1000);
    for seat in input {
        let mut _left = 0;
        let mut _right = 127;
        let mut mid = 0;

        for chr in seat[0..=6].chars() {
            mid = (_left + _right) / 2;
            if chr.eq(&'F') {
                _right = mid;
            } else if chr.eq(&'B') {
                mid += 1;
                _left = mid;
            }
        }

        let row = _right;

        _left = 0;
        _right = 7;

        for chr in seat[7..=9].chars() {
            mid = (_left + _right) / 2;
            if chr.eq(&'L') {
                _right = mid;
            } else if chr.eq(&'R') {
                mid += 1;
                _left = mid;
            }
        }

        let column = mid;
        vec.push((row, column));
    }

    vec
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(&file);

    let input: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    let seats = util(&input);

    let sid1 = task1(&seats);
    let sid2 = task2(&seats);

    println!("Task 1: Highest seat id is {}", sid1.unwrap());
    println!("Task 2: My seat id is {}", sid2.unwrap());

    Ok(())
}
