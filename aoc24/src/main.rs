use std::collections::HashSet;
use std::fs;

fn parse(file: String) -> Vec<Vec<usize>> {
    let file = file.replace("se", "1");
    let file = file.replace("sw", "2");
    let file = file.replace("nw", "4");
    let file = file.replace("ne", "5");
    let file = file.replace("e", "0");
    let file = file.replace("w", "3");

    file.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn task1(input: &Vec<Vec<usize>>) -> usize {
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    for line in input {
        let mut pos = (0, 0);
        for dir in line {
            pos = match *dir {
                0 => (pos.0 + 1, pos.1),
                1 => (pos.0, pos.1 + 1),
                2 => (pos.0 - 1, pos.1 + 1),
                3 => (pos.0 - 1, pos.1),
                4 => (pos.0, pos.1 - 1),
                5 => (pos.0 + 1, pos.1 - 1),
                _ => (pos.0, pos.1),
            };
        }

        if !set.remove(&pos) {
            set.insert(pos);
        }
    }
    set.len()
}

fn neighbors(pos: &(isize, isize)) -> Vec<(isize, isize)> {
    let mut v = Vec::new();
    v.push((pos.0 + 1, pos.1));
    v.push((pos.0, pos.1 + 1));
    v.push((pos.0 - 1, pos.1 + 1));
    v.push((pos.0 - 1, pos.1));
    v.push((pos.0, pos.1 - 1));
    v.push((pos.0 + 1, pos.1 - 1));
    v
}

fn task2(input: &Vec<Vec<usize>>, days: usize) -> usize {
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    for line in input {
        let mut pos = (0, 0);
        for dir in line {
            pos = match *dir {
                0 => (pos.0 + 1, pos.1),
                1 => (pos.0, pos.1 + 1),
                2 => (pos.0 - 1, pos.1 + 1),
                3 => (pos.0 - 1, pos.1),
                4 => (pos.0, pos.1 - 1),
                5 => (pos.0 + 1, pos.1 - 1),
                _ => (pos.0, pos.1),
            };
        }

        if !set.remove(&pos) {
            set.insert(pos);
        }
    }

    for _ in 1..=days {
        let still_black: HashSet<(isize, isize)> = set
            .iter()
            .filter(|&p| neighbors(p).iter().filter(|q| set.contains(q)).count() == 1)
            .map(|x| x.clone())
            .collect();

        let turned_black: HashSet<(isize, isize)> = set
            .iter()
            .flat_map(|x| neighbors(x))
            .filter(|y| !still_black.contains(y))
            .filter(|z| neighbors(z).iter().filter(|p| set.contains(p)).count() == 2)
            .collect();

        set = still_black
            .iter()
            .chain(turned_black.iter())
            .map(|x| *x)
            .collect();
    }

    set.len()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");
    let input = parse(file);
    let black1 = task1(&input);
    let black2 = task2(&input, 100);

    println!("Task 1: The number of black squares is: {}", black1);
    println!("Task 2: The number of black squares is: {}", black2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from(
            "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
        );
        let input = parse(s);
        let black = task1(&input);
        assert_eq!(black, 10);
    }

    #[test]
    fn test2() {
        let s = String::from(
            "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
        );
        let input = parse(s);
        let black = task2(&input, 100);
        assert_eq!(black, 2208);
    }
}
