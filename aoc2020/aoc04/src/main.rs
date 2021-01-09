use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn task1(input: &Vec<HashMap<String, String>>) -> usize {
    let mut sum = 0;
    for dict in input {
        if dict.contains_key("byr")
            && dict.contains_key("iyr")
            && dict.contains_key("eyr")
            && dict.contains_key("hgt")
            && dict.contains_key("hcl")
            && dict.contains_key("ecl")
            && dict.contains_key("pid")
        {
            sum += 1;
        }
    }
    sum
}

fn task2(input: &Vec<HashMap<String, String>>) -> usize {
    let mut sum = 0;
    for dict in input {
        if let Some(byear) = dict.get("byr") {
            if byear.parse::<i64>().unwrap() >= 1920 && byear.parse::<i64>().unwrap() <= 2002 {
                if let Some(iyear) = dict.get("iyr") {
                    if iyear.parse::<i64>().unwrap() >= 2010
                        && iyear.parse::<i64>().unwrap() <= 2020
                    {
                        if let Some(eyear) = dict.get("eyr") {
                            if eyear.parse::<i64>().unwrap() >= 2020
                                && eyear.parse::<i64>().unwrap() <= 2030
                            {
                                if let Some(height) = dict.get("hgt") {
                                    let len = height.len();
                                    let num = height[0..len - 2].parse::<i64>().unwrap();
                                    if (height.chars().nth(len - 2).unwrap() == 'c'
                                        && height.chars().nth(len - 1).unwrap() == 'm'
                                        && num >= 150
                                        && num <= 193)
                                        || (height.chars().nth(len - 2).unwrap() == 'i'
                                            && height.chars().nth(len - 1).unwrap() == 'n'
                                            && num >= 59
                                            && num <= 76)
                                    {
                                        if let Some(hcolor) = dict.get("hcl") {
                                            if hcolor.chars().nth(0).unwrap() == '#'
                                                && hcolor[1..]
                                                    .chars()
                                                    .any(|c| matches!(c, '0'..='9' | 'a'..='f'))
                                                && hcolor.len() == 7
                                            {
                                                if let Some(ecolor) = dict.get("ecl") {
                                                    if ecolor.eq("amb")
                                                        || ecolor.eq("blu")
                                                        || ecolor.eq("brn")
                                                        || ecolor.eq("gry")
                                                        || ecolor.eq("grn")
                                                        || ecolor.eq("hzl")
                                                        || ecolor.eq("oth")
                                                    {
                                                        if let Some(pid) = dict.get("pid") {
                                                            if pid.len() == 9 {
                                                                sum += 1;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(&file);

    let mut i = 0;
    let mut input: Vec<HashMap<String, String>> = Vec::with_capacity(1000);
    for _ in 0..input.capacity() {
        let kv = HashMap::new();
        input.push(kv);
    }

    for line in file.lines().map(|x| x.unwrap()) {
        if !line.eq("") {
            for kv in line.split(' ') {
                let kv: Vec<String> = kv.split(':').map(|s| String::from(s)).collect();
                let key = kv[0].clone();
                let value = kv[1].clone();

                input[i].insert(key, value);
            }
        } else {
            i += 1;
        }
    }

    let passports1 = task1(&input);
    let passports2 = task2(&input);

    println!("Task 1: Number of valid passports: {}", passports1);
    println!("Task 2: Number of valid passports: {}", passports2);

    Ok(())
}
