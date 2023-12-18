use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let histories = file.lines().map(|l| {
        l.trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<isize>().ok())
            .collect::<Vec<isize>>()
    });

    let seqs_iter = histories.map(|h| {
        let mut finished = false;
        let mut sequences = vec![h.clone()];
        let mut iter = h;

        while !finished {
            let seq = iter.windows(2).map(|w| w[1] - w[0]).collect::<Vec<isize>>();
            finished = seq.iter().all(|&x| x == 0);
            iter = seq.clone();
            sequences.push(seq);
        }
        sequences
    });

    let part1: isize = seqs_iter
        .clone()
        .map(|seqs| {
            seqs.iter()
                .rev()
                .scan(0, |state, acc| {
                    let &last = acc.last().unwrap();
                    *state = last + *state;
                    Some(*state)
                })
                .last()
                .unwrap()
        })
        .sum();

    let part2: isize = seqs_iter
        .map(|seqs| {
            seqs.iter()
                .rev()
                .scan(0, |state, acc| {
                    let &first = acc.first().unwrap();
                    *state = first - *state;
                    Some(*state)
                })
                .last()
                .unwrap()
        })
        .sum();

    println!("Day 9 part1 is {part1}");
    println!("Day 9 part2 is {part2}");
}
