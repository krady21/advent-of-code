use std::collections::{VecDeque, HashMap};
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Token {
    Add,
    Mul,
    LeftP,
    Value(usize),
}

fn process(token: Token, stack: &mut Vec<Token>, dequeue: &mut VecDeque<Token>, order: u8) {
    let mut order_map = HashMap::new();
    if order == 1 {
        order_map.insert(Token::Add, 1);
        order_map.insert(Token::Mul, 1);
        order_map.insert(Token::LeftP, 0);
    } else if order == 2 {
        order_map.insert(Token::Add, 1);
        order_map.insert(Token::Mul, 0);
        order_map.insert(Token::LeftP, 0);
    }

    let order = order_map.get(&token).unwrap();
    while let Some(token) = stack.last() {
        match token {
            Token::Add | Token::Mul if order <= order_map.get(token).unwrap() => dequeue.push_back(stack.pop().unwrap()),
            _ => break,
        }
    }
    stack.push(token);
}

fn task(file: &str, order: u8) -> usize {
    file.lines().map(|l| {
        let tokens = l.chars().filter(|c| !c.is_whitespace());
        let mut stack: Vec<Token> = Vec::new();
        let mut dequeue: VecDeque<Token> = VecDeque::new();
        for token in tokens {
            match token {
                '+' => process(Token::Add, &mut stack, &mut dequeue, order),
                '*' => process(Token::Mul, &mut stack, &mut dequeue, order),
                '(' => stack.push(Token::LeftP),
                ')' => loop {
                    match stack.pop() {
                        Some(Token::LeftP) | None => break,
                        Some(last) => dequeue.push_back(last),
                    }
                },
                num => dequeue.push_back(Token::Value(num.to_digit(10).unwrap() as usize)),
            }
        }

        while let Some(last) = stack.pop() {
            dequeue.push_back(last)
        }

        while let Some(front) = dequeue.pop_front() {
            match front {
                Token::Add => {
                    if let (Some(Token::Value(lhs)), Some(Token::Value(rhs))) = (stack.pop(), stack.pop()) {
                        stack.push(Token::Value(lhs + rhs));
                    }
                }
                Token::Mul => {
                    if let (Some(Token::Value(lhs)), Some(Token::Value(rhs))) = (stack.pop(), stack.pop()) {
                        stack.push(Token::Value(lhs * rhs));
                    }
                }
                val => stack.push(val),
            }
        }

        if let Some(Token::Value(result)) = stack.pop() {
            result
        } else {
            0
        }

    }).sum()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");

    let sum1 = task(&file, 1);
    let sum2 = task(&file, 2);

    println!("Task 1: The sum is: {}", sum1);
    println!("Task 2: The sum is: {}", sum2);
}
