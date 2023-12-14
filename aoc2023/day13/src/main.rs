use std::fs;

fn reflection(v: &Vec<String>) -> Option<usize> {
    let n = v.len();

    (0..n - 1)
        .find(|&i| {
            let before = v[0..=i].iter().rev();
            let after = v[i + 1..n].iter();
            before.zip(after).all(|(v1, v2)| v1.eq(v2))
        })
        .map(|i| i + 1)
}

fn reflection2(v: &Vec<String>) -> Option<usize> {
    let n = v.len();

    (0..n - 1)
        .find(|&i| {
            let before = v[0..=i].iter().rev();
            let after = v[i + 1..n].iter();

            // find the split for which only one line is different
            let different_vecs: Vec<(&String, &String)> =
                before.zip(after).filter(|&(v1, v2)| *v1 != *v2).collect();

            let different_lines_nr = different_vecs.len();
            if different_lines_nr == 0 || different_lines_nr > 1 {
                return false;
            }

            // check if the two lines differ only by one char
            let (s1, s2) = different_vecs[0];
            let different_chars = s1
                .chars()
                .zip(s2.chars())
                .filter(|(c1, c2)| *c1 != *c2)
                .count();

            different_chars == 1
        })
        .map(|i| i + 1)
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let input = file.split("\n\n").map(|block| {
        let lines = block
            .trim()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let nrows = lines.len();
        let ncols = lines[0].len();

        let cols: Vec<String> = (0..ncols)
            .map(|j| {
                (0..nrows)
                    .map(|i| lines[i].chars().nth(j).unwrap())
                    .collect::<String>()
            })
            .collect();

        (lines, cols)
    });

    let part1: usize = input
        .clone()
        .map(|(lines, cols)| {
            let line_reflection = reflection(&lines);
            let col_reflection = reflection(&cols);

            line_reflection
                .map(|r| r * 100)
                .unwrap_or(col_reflection.unwrap_or(0))
        })
        .sum();

    let part2: usize = input
        .map(|(lines, cols)| {
            let line_reflection = reflection2(&lines);
            let col_reflection = reflection2(&cols);

            line_reflection
                .map(|r| r * 100)
                .unwrap_or(col_reflection.unwrap_or(0))
        })
        .sum();

    println!("Day 13 part1 is {part1}");
    println!("Day 13 part2 is {part2}");
}
