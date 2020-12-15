fn task1(input: &Vec<usize>, turns: usize) -> usize {
    let mut vector: Vec<(usize, usize)> = vec![(usize::MAX, usize::MAX); turns];
    let mut next = input.last().unwrap().to_owned();

    for (i, n) in input.iter().map(|x| x.to_owned()).enumerate() {
        vector[n].1 = i;
    }

    for i in input.len()..turns {
        let (first, second) = vector[next];
        next = if first != usize::MAX && second != usize::MAX {
            second - first
        } else {
            0
        };
        vector[next] = (vector[next].1, i);
    }
    next
}

fn main() {
    let input: Vec<usize> = vec![1, 0, 15, 2, 10, 13];

    let number1 = task1(&input, 2020);
    let number2 = task1(&input, 30000000);

    println!("Task 1: The 2020th number spoken is {}", number1);
    println!("Task 2: The 30000000th number spoken is {}", number2);
}
