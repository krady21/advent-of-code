use std::collections::HashMap;
use std::fs;

fn contains_gold(bag: &str, map: &HashMap<String, Vec<(String, u32)>>) -> bool {
    if let Some(bags) = map.get(bag) {
        for item in bags.iter() {
            if item.0.eq("shiny gold") || contains_gold(&item.0, map) {
                return true;
            }
        }
    }
    false
}

fn task1(map: &HashMap<String, Vec<(String, u32)>>) -> usize {
    map.keys().filter(|x| contains_gold(x, map)).count()
}

fn task2(bag: &str, map: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let mut sum = 0;
    match map.get(bag) {
        Some(bags) => sum = bags.iter().map(|(x, y)| y * task2(x, map) + y).sum::<u32>(),
        None => sum += 1,
    }

    sum
}

fn parse(input: &str) -> HashMap<String, Vec<(String, u32)>> {
    let mut map: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let mut kvvec: Vec<(String, u32)> = Vec::new();

    for line in input.lines() {
        let split = line.splitn(5, ' ').collect::<Vec<&str>>();
        let key = split[0..2].join(" ");
        let bags = split.last().unwrap();

        for bag in bags.split(|c| c == ',' || c == '\n') {
            let data = bag.split_whitespace().collect::<Vec<&str>>();
            if let Ok(num) = data[0].trim().parse() {
                let value: (String, u32) = (data[1..3].join(" "), num);
                kvvec.push(value);
            }
        }
        map.insert(key.clone(), kvvec.clone());
        kvvec.clear();
    }
    map
}

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let map = parse(&file);

    let bags1 = task1(&map);
    let bags2 = task2("shiny gold", &map);

    println!("Task 1: The number of bags is: {}", bags1);
    println!("Task 2: The number of bags is: {}", bags2);

    Ok(())
}
