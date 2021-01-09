#![feature(vec_remove_item)]

use std::collections::HashMap;
use std::fs;

fn task1(file: &str) -> usize {
    let parsed: Vec<Vec<usize>> = file
        .lines()
        .take(20)
        .map(|l| {
            l.chars()
                .filter(|c| c.is_whitespace() || c.is_digit(10) || *c == '-')
                .collect::<String>()
        })
        .map(|s| {
            s.split(|c| c == ' ' || c == '-')
                .filter(|n| *n != "")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    file.lines()
        .skip(25)
        .map(|l| {
            l.split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .map(|n| {
                    let mut flag = false;
                    for v in &parsed {
                        if (v[0] <= n && n <= v[1]) || (v[2] <= n && n <= v[3]) {
                            flag = true;
                            break;
                        }
                    }

                    if flag {
                        0
                    } else {
                        n
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn assign(input: &HashMap<String, Vec<usize>>) -> HashMap<String, usize> {
    let mut copy = input.clone();
    let mut copy2 = input.clone();
    let mut length = input.len();

    let mut map: HashMap<String, usize> = HashMap::with_capacity(input.len());
    let mut value = 0;

    while length > 0 {
        for (k, v) in copy2.iter() {
            if v.len() == 1 {
                value = v[0].clone();
                map.insert(k.to_string(), value);
                copy.remove(k);
                break;
            }
        }

        for (_, v) in copy.iter_mut() {
            let index = v.iter_mut().position(|x| *x == value).unwrap();
            v.remove(index);
        }
        copy2 = copy.clone();
        length -= 1;
    }
    map
}

fn task2(file: &str) -> usize {
    let parsed: Vec<(String, Vec<usize>)> = file
        .lines()
        .take(20)
        .map(|l| {
            let field = l.split(':').next().unwrap().to_owned();
            let limits = l
                .chars()
                .filter(|c| c.is_whitespace() || c.is_digit(10) || *c == '-')
                .collect::<String>();
            (field, limits)
        })
        .map(|(f, s)| {
            let v = s
                .split(|c| c == ' ' || c == '-')
                .filter(|n| *n != "")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (f, v)
        })
        .collect();

    let tickets: Vec<&str> = file
        .lines()
        .skip(25)
        .filter(|l| {
            let count = l
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .filter(|&n| {
                    let mut flag = false;
                    for v in &parsed {
                        if (v.1[0] <= n && n <= v.1[1]) || (v.1[2] <= n && n <= v.1[3]) {
                            flag = true;
                            break;
                        }
                    }
                    !flag
                })
                .count();
            count == 0
        })
        .collect();

    let my_ticket: Vec<usize> = file
        .lines()
        .nth(22)
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut matrix: Vec<Vec<usize>> = tickets
        .iter()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();
    matrix.push(my_ticket.clone());

    let mut positions: HashMap<String, Vec<usize>> = HashMap::with_capacity(parsed.len());

    for limits in parsed {
        for j in 0..matrix[0].len() {
            let mut flag = true;
            for i in 0..matrix.len() {
                if !((limits.1[0] <= matrix[i][j] && matrix[i][j] <= limits.1[1])
                    || (limits.1[2] <= matrix[i][j] && matrix[i][j] <= limits.1[3]))
                {
                    flag = false;
                }
            }

            if flag {
                if positions.contains_key(&limits.0) {
                    let mut arr = positions.get(&limits.0).unwrap().to_owned();
                    arr.push(j);
                    positions.insert(limits.0.to_string(), arr);
                } else {
                    positions.insert(limits.0.to_string(), vec![j]);
                }
            }
        }
    }

    let assigned = assign(&positions);
    assigned
        .iter()
        .filter(|(k, _)| k.contains("departure"))
        .fold(1, |acc, (_, &v)| acc * my_ticket[v])
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");

    let sum = task1(&file);
    let product = task2(&file);

    println!("Task 1: The sum is {}", sum);
    println!("Task 2: The sum is {}", product);
}
