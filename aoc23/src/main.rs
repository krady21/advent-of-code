fn task1(input: &str, moves: usize) -> usize {
    let mut input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).map(|i| i as usize).unwrap())
        .collect();

    let min = input.iter().min().unwrap().to_owned();
    let max = input.iter().max().unwrap().to_owned();
    let len = input.len();

    for i in 0..moves {
        let mut pickup = Vec::with_capacity(3);
        let mut destination = input[i % input.len()];

        let mut copy = Vec::with_capacity(3);
        for j in 1..4 {
            copy.push(input[(i + j) % len]);
        }

        for num in copy {
            pickup.push(input.remove(input.iter().position(|&d| d == num).unwrap()));
        }

        let mut found = false;
        while !found {
            if destination == min {
                destination = max;
            } else {
                destination -= 1;
            }

            if !pickup.contains(&destination) {
                found = true;
            }
        }

        let dindex = input.iter().position(|&d| d == destination).unwrap();
        for digit in pickup.iter().rev() {
            input.insert(dindex + 1, *digit);
        }

        if dindex < i % len {
            if len - 1 - i % len < 3 {
                input.rotate_left(len - 1 - i % len);
            } else {
                input.rotate_left(3);
            }
        }
    }

    let pos = input.iter().position(|&x| x == 1).unwrap();
    input.remove(pos);
    input.rotate_left(pos);

    input
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn task2(input: &str, moves: usize) -> usize {
    let mut input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).map(|i| i as usize).unwrap())
        .collect();

    input.extend(10..=1_000_000);
    let min = input.iter().min().unwrap().to_owned();
    let max = input.iter().max().unwrap().to_owned();

    let mut next = vec![0; input.len() + 1];
    let mut curr = input[0];
    for i in 0..(input.len() - 1) {
        next[input[i]] = input[i + 1];
    }
    next[input[input.len() - 1]] = input[0];

    for _ in 0..moves {
        let pickup = (next[curr], next[next[curr]], next[next[next[curr]]]);

        let mut destination = curr;
        let mut found = false;
        while !found {
            if destination == min {
                destination = max;
            } else {
                destination -= 1;
            }

            if destination != pickup.0 && destination != pickup.1 && destination != pickup.2 {
                found = true;
            }
        }

        next[curr] = next[pickup.2];
        next[pickup.2] = next[destination];
        next[destination] = pickup.0;

        curr = next[curr];
    }

    next[1] * next[next[1]]
}

fn main() {
    let input = "963275481";
    let label = task1(&input, 100);
    let product = task2(&input, 10_000_000);

    println!("Task 1: The labels are: {}", label);
    println!("Task 2: The product is: {}", product);
}
