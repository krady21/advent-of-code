use std::fs;

// TODO: Refactor task2 and (maybe) task 1

fn task1(input: &Vec<(String, i64)>) -> i64 {
    let mut i = 0;
    let mut acc: i64 = 0;
    let mut visited = vec![false; input.len()];

    while !visited[i] && i < input.len() {
        visited[i] = true;
        match input[i].0.as_str() {
            "acc" => {
                acc += input[i].1;
                i += 1;
            }
            "jmp" => {
                i = ((i as i64) + input[i].1) as usize;
            }
            "nop" => {
                i += 1;
            }
            _ => {}
        }
    }
    acc
}

fn task2(input: &mut Vec<(String, i64)>) -> i64 {
    let mut i = 0;
    let mut acc: i64 = 0;
    let mut visited = vec![false; input.len()];
    let mut finished = false;

    while !finished {
        match input[i].0.as_str() {
            "acc" => {
                acc += input[i].1;
                visited[i] = true;
                i += 1;
            }
            "jmp" => {
                input[i].0 = String::from("nop");
                let mut j = i;
                let mut acc2 = 0;
                let mut visited2 = visited.clone();

                while j != input.len() && !visited2[j] {
                    visited2[j] = true;
                    match input[j].0.as_str() {
                        "acc" => {
                            acc2 += input[j].1;
                            j += 1;
                        }
                        "jmp" => {
                            j = ((j as i64) + input[j].1) as usize;
                        }
                        "nop" => {
                            j += 1;
                        }
                        _ => {}
                    }
                }

                if j == input.len() {
                    acc += acc2;
                    finished = true;
                } else {
                    input[i].0 = String::from("jmp");
                    visited[i] = true;
                    i = ((i as i64) + input[i].1) as usize;
                }
            }
            "nop" => {
                input[i].0 = String::from("jmp");
                let mut j = i;
                let mut acc2 = 0;
                let mut visited2 = visited.clone();

                while j != input.len() && !visited2[j] {
                    visited2[j] = true;
                    match input[j].0.as_str() {
                        "acc" => {
                            acc2 += input[j].1;
                            j += 1;
                        }
                        "jmp" => {
                            j = ((j as i64) + input[j].1) as usize;
                        }
                        "nop" => {
                            j += 1;
                        }
                        _ => {}
                    }
                }

                if j == input.len() {
                    acc += acc2;
                    finished = true;
                } else {
                    input[i].0 = String::from("nop");
                    visited[i] = true;
                    i += 1;
                }
            }
            _ => {}
        }
    }
    acc
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");

    let mut input: Vec<(String, i64)> = file
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().to_owned(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    let acc1 = task1(&input);
    let acc2 = task2(&mut input);

    println!("Task 1: The accumulator value is: {}", acc1);
    println!("Task 2: The accumulator value is: {}", acc2);
}
