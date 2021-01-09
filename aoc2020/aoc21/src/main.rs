use std::collections::{HashMap, HashSet};
use std::fs;

type Ingredient = String;
type Allergen = String;

#[derive(Debug, PartialEq, Eq)]
struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

impl Food {
    fn new(input: &str) -> Self {
        let mut allergens = HashSet::new();
        let split: Vec<&str> = input.split(" (contains ").collect();
        let ingredients: HashSet<Ingredient> =
            split[0].split_whitespace().map(|s| s.to_string()).collect();

        if input.contains('(') {
            let text = split[1]
                .chars()
                .filter(|c| *c != ')' && *c != ',')
                .collect::<String>();
            allergens = text
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<HashSet<Allergen>>();
        }

        Food {
            ingredients,
            allergens,
        }
    }
}

fn parse(file: &str) -> Vec<Food> {
    let mut foods = Vec::with_capacity(40);
    for line in file.lines() {
        let food = Food::new(line);
        foods.push(food);
    }
    foods
}

fn find_allergic(input: &Vec<Food>) -> HashMap<Ingredient, Allergen> {
    let mut allergen2ingredients: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    let mut allergic_ingredients: HashMap<Ingredient, Allergen> = HashMap::new();
    for food in input {
        for allergen in food.allergens.iter() {
            if let Some(ingredients) = allergen2ingredients.get_mut(allergen) {
                *ingredients = ingredients
                    .intersection(&food.ingredients)
                    .map(|x| x.to_string())
                    .collect();
            } else {
                allergen2ingredients.insert(allergen.to_string(), food.ingredients.clone());
            }
        }
    }

    while !allergen2ingredients.is_empty() {
        let copy = allergen2ingredients.clone();
        let (allergen, ingredient) = copy.iter().find(|&(_, i)| i.len() == 1).unwrap();
        let ingredient = ingredient.iter().next().unwrap();
        for ilist in allergen2ingredients.values_mut() {
            ilist.remove(ingredient);
        }
        allergen2ingredients.remove(allergen);
        allergic_ingredients.insert(ingredient.to_string(), allergen.to_string());
    }
    allergic_ingredients
}

fn task1(input: &Vec<Food>) -> usize {
    let allergic = find_allergic(input);
    input
        .iter()
        .flat_map(|v| v.ingredients.clone())
        .filter(|x| !allergic.contains_key(x))
        .count()
}

fn task2(input: &Vec<Food>) -> String {
    let allergic = find_allergic(input);
    let mut list: Vec<(Ingredient, Allergen)> = allergic.into_iter().collect();
    list.sort_by_key(|(_, a)| a.to_string());
    let list: Vec<Ingredient> = list.iter().map(|(i, _)| i.to_string()).collect();
    list.join(",")
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let input = parse(&file);

    let sum = task1(&input);
    let list = task2(&input);

    println!("Task 1: The sum of allergic item apparitions is: {}", sum);
    println!("Task 2: The list of dangerous items is: {}", list);
}
