use std::fs;

// TODO: Refactor this mess

fn change_seat1(input: &Vec<Vec<char>>, i: usize, j: usize) -> Option<char> {
    match input[i][j] {
        'L' => {
            if !input[i - 1][j - 1].eq(&'#')
                && !input[i - 1][j].eq(&'#')
                && !input[i - 1][j + 1].eq(&'#')
                && !input[i][j - 1].eq(&'#')
                && !input[i][j + 1].eq(&'#')
                && !input[i + 1][j - 1].eq(&'#')
                && !input[i + 1][j].eq(&'#')
                && !input[i + 1][j + 1].eq(&'#')
            {
                Some('#')
            } else {
                Some('L')
            }
        }
        '#' => {
            let mut count = 0;
            if input[i - 1][j - 1].eq(&'#') {
                count += 1;
            }

            if input[i - 1][j].eq(&'#') {
                count += 1;
            }

            if input[i - 1][j + 1].eq(&'#') {
                count += 1;
            }

            if input[i][j - 1].eq(&'#') {
                count += 1;
            }

            if input[i][j + 1].eq(&'#') {
                count += 1;
            }

            if input[i + 1][j - 1].eq(&'#') {
                count += 1;
            }

            if input[i + 1][j].eq(&'#') {
                count += 1;
            }

            if input[i + 1][j + 1].eq(&'#') {
                count += 1;
            }

            if count >= 4 {
                Some('L')
            } else {
                Some('#')
            }
        }
        '.' => Some('.'),
        _ => None,
    }
}

fn change_seat2(input: &Vec<Vec<char>>, i: usize, j: usize) -> Option<char> {
    let rows: isize = input.len() as isize;
    let columns: isize = input[0].len() as isize;

    match input[i][j] {
        'L' => {
            let mut count = 0;
            let mut x = (i - 1) as isize;
            let mut y = (j - 1) as isize;
            while x >= 0 && y >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (x == 0 || y == 0) {
                    count += 1;
                    break;
                }
                x -= 1;
                y -= 1;
            }

            let mut x = (i - 1) as isize;
            let y = j as isize;
            while x >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (x == 0) {
                    count += 1;
                    break;
                }
                x -= 1;
            }

            let mut x = (i - 1) as isize;
            let mut y = (j + 1) as isize;
            while x >= 0 && y <= columns - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (x == 0 || y == columns - 1) {
                    count += 1;
                    break;
                }
                x -= 1;
                y += 1;
            }

            let x = i as isize;
            let mut y = (j + 1) as isize;
            while y <= columns - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (y == columns - 1) {
                    count += 1;
                    break;
                }
                y += 1;
            }

            let mut x = (i + 1) as isize;
            let mut y = (j + 1) as isize;
            while x <= rows - 1 && y <= columns - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (x == rows - 1 || y == columns - 1) {
                    count += 1;
                    break;
                }
                x += 1;
                y += 1;
            }

            let mut x = (i + 1) as isize;
            let y = j as isize;
            while x <= rows - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (x == rows - 1) {
                    count += 1;
                    break;
                }
                x += 1;
            }

            let mut x = (i + 1) as isize;
            let mut y = (j - 1) as isize;
            while x <= rows - 1 && y >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (x == rows - 1 || y == 0) {
                    count += 1;
                    break;
                }
                if y == 0 {
                    break;
                }
                x += 1;
                y -= 1;
            }

            let x = i as isize;
            let mut y = (j - 1) as isize;
            while y >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    return Some('L');
                }
                if input[x as usize][y as usize].eq(&'L') || (y == 0) {
                    count += 1;
                    break;
                }
                if y == 0 {
                    break;
                }
                y -= 1;
            }

            if count == 8 {
                return Some('#');
            } else {
                return Some('L');
            }
        }
        '#' => {
            let mut count = 0;
            let mut x = (i - 1) as isize;
            let mut y = (j - 1) as isize;
            while x >= 0 && y >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                x -= 1;
                y -= 1;
            }

            let mut x = (i - 1) as isize;
            let y = j as isize;
            while x >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                x -= 1;
            }

            let mut x = (i - 1) as isize;
            let mut y = (j + 1) as isize;
            while x >= 0 && y <= columns - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                x -= 1;
                y += 1;
            }

            let x = i as isize;
            let mut y = (j + 1) as isize;
            while y <= columns - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                y += 1;
            }

            let mut x = (i + 1) as isize;
            let mut y = (j + 1) as isize;
            while x <= rows - 1 && y <= columns - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                x += 1;
                y += 1;
            }

            let mut x = (i + 1) as isize;
            let y = j as isize;
            while x <= rows - 1 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                x += 1;
            }

            let mut x = (i + 1) as isize;
            let mut y = (j - 1) as isize;
            while x <= rows - 1 && y >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                x += 1;
                y -= 1;
            }

            let x = i as isize;
            let mut y = (j - 1) as isize;
            while y >= 0 {
                if input[x as usize][y as usize].eq(&'#') {
                    count += 1;
                    break;
                }
                if input[x as usize][y as usize].eq(&'L') {
                    break;
                }
                y -= 1;
            }
            if count >= 5 {
                return Some('L');
            } else {
                return Some('#');
            }
        }
        '.' => Some('.'),
        _ => None,
    }
}
fn count_occupied(input: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for vector in input {
        for chr in vector {
            if chr.eq(&'#') {
                sum += 1;
            }
        }
    }
    sum
}

fn task1(input: &Vec<Vec<char>>) -> usize {
    let mut copy = input.clone();
    let mut copy2 = input.clone();
    loop {
        let mut flag = false;
        for i in 0..copy.len() {
            for j in 0..copy[0].len() {
                if let Some(c) = change_seat1(&copy, i, j) {
                    if !c.eq(&copy[i][j]) {
                        flag = true;
                    }
                    copy2[i][j] = c;
                }
            }
        }
        copy = copy2.clone();

        if !flag {
            return count_occupied(&copy);
        }
    }
}

fn task2(input: &Vec<Vec<char>>) -> usize {
    let mut copy = input.clone();
    let mut copy2 = input.clone();
    loop {
        let mut flag = false;
        for i in 1..copy.len() - 1 {
            for j in 1..copy[0].len() - 1 {
                if let Some(c) = change_seat2(&copy, i, j) {
                    if !c.eq(&copy[i][j]) {
                        flag = true;
                    }
                    copy2[i][j] = c;
                }
            }
        }
        copy = copy2.clone();

        if !flag {
            return count_occupied(&copy);
        }
    }
}

fn preprocess(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = input
        .clone()
        .iter()
        .map(|v| {
            let mut v = v.clone();
            v.insert(0, '.');
            v.push('.');
            v
        })
        .collect();

    let empty_line = vec!['.'; input[0].len() + 2];
    matrix.insert(0, empty_line.clone());
    matrix.push(empty_line);

    matrix
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let data: Vec<Vec<char>> = file
        .lines()
        .map(|l| l.chars().collect())
        .into_iter()
        .collect();
    let input = preprocess(&data);

    let occupied1 = task1(&input);
    let occupied2 = task2(&input);

    println!("Task 1: The number of occupied seats is: {}", occupied1);
    println!("Task 2: The number of occupied seats is: {}", occupied2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input2() {
        let data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let input = data
            .lines()
            .map(|l| l.chars().collect())
            .into_iter()
            .collect();

        let input = preprocess(&input);
        assert_eq!(26, task2(&input));
    }
}
