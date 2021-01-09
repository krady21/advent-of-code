use std::collections::HashMap;
use std::fs;

fn task1(input: &Vec<usize>) -> usize {
    let d1 = input.windows(2).filter(|&w| w[1] - w[0] == 1).count();
    let d3 = input.windows(2).filter(|&w| w[1] - w[0] == 3).count();

    d1 * d3
}

fn task2(input: &Vec<usize>, index: usize, map: &mut HashMap<usize, usize>) -> usize {
    if index == input.len() - 1 {
        return 1;
    }

    let total = input
        .iter()
        .enumerate()
        .skip(index + 1)
        .take_while(|(_, x)| (1..=3).contains(&(*x - input[index])))
        .map(|(i, _)| {
            map.get(&i)
                .copied()
                .unwrap_or_else(|| task2(&input, i, map))
        })
        .sum();
    map.insert(index, total);
    total
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let mut adapters: Vec<usize> = file.lines().filter_map(|x| x.parse().ok()).collect();
    adapters.sort();

    let mut input = vec![0];
    input.extend(adapters);
    input.push(input.last().unwrap() + 3);

    let magic = task1(&input);
    let arrangements = task2(&input, 0, &mut HashMap::new());

    println!("Task 1: The value is {}", magic);
    println!("Task 2: The number of arrangements is {}", arrangements);
}
