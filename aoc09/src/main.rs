use std::collections::HashSet;
use std::fs;

fn task1(input: &Vec<i64>) -> Option<i64> {
    let mut map: HashSet<i64> = HashSet::with_capacity(25);

    input
        .iter()
        .enumerate()
        .skip(25)
        .map(|(index, num)| {
            let is_sum = ((index - 25)..index)
                .map(|i| {
                    map.insert(input[i]);
                    map.contains(&(num - input[i]))
                })
                .any(|x| x);
            map.clear();
            (index, is_sum)
        })
        .filter(|(_, s)| !s)
        .map(|(i, _)| input[i])
        .next()
}

fn task2(input: &Vec<i64>, num: i64) -> Option<i64> {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let sum = input[i..j].iter().sum::<i64>();
            let min = input[i..j].iter().min().unwrap();
            let max = input[i..j].iter().max().unwrap();

            if sum == num {
                return Some(min + max);
            } else if sum > num {
                break;
            }
        }
    }
    None
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let input: Vec<i64> = file.lines().map(|x| x.parse::<i64>().unwrap()).collect();

    let num = task1(&input);
    let sum = task2(&input, num.unwrap());

    println!("Task 1: The first value is: {}", num.unwrap());
    println!("Task 2: The sum is: {}", sum.unwrap());
}
