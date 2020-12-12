use std::fs;

fn convert1<'a>(current: &'a str, direction: &'a str, angle: i64) -> &'a str {
    match direction {
        "R" => match current {
            "N" => match angle {
                90 => "E",
                180 => "S",
                270 => "W",
                _ => "",
            },
            "S" => match angle {
                90 => "W",
                180 => "N",
                270 => "E",
                _ => "",
            },
            "E" => match angle {
                90 => "S",
                180 => "W",
                270 => "N",
                _ => "",
            },
            "W" => match angle {
                90 => "N",
                180 => "E",
                270 => "S",
                _ => "",
            },
            _ => "",
        },
        "L" => match current {
            "N" => match angle {
                90 => "W",
                180 => "S",
                270 => "E",
                _ => "",
            },
            "S" => match angle {
                90 => "E",
                180 => "N",
                270 => "W",
                _ => "",
            },
            "E" => match angle {
                90 => "N",
                180 => "W",
                270 => "S",
                _ => "",
            },
            "W" => match angle {
                90 => "S",
                180 => "E",
                270 => "N",
                _ => "",
            },
            _ => "",
        },
        _ => "",
    }
}

fn task1(input: &str) -> i64 {
    let ship = input
        .lines()
        .map(|l| l.split_at(1))
        .map(|t| (t.0, t.1.parse::<i64>().unwrap()))
        .fold((0, 0, "E"), |mut ship, x| {
            match x.0 {
                "N" => {
                    ship.0 += x.1;
                }
                "S" => {
                    ship.0 -= x.1;
                }
                "E" => {
                    ship.1 += x.1;
                }
                "W" => {
                    ship.1 -= x.1;
                }
                "L" => {
                    ship.2 = convert1(ship.2, "L", x.1);
                }
                "R" => {
                    ship.2 = convert1(ship.2, "R", x.1);
                }
                "F" => match ship.2 {
                    "N" => {
                        ship.0 += x.1;
                    }
                    "S" => {
                        ship.0 -= x.1;
                    }
                    "E" => {
                        ship.1 += x.1;
                    }
                    "W" => {
                        ship.1 -= x.1;
                    }
                    _ => {}
                },
                _ => {}
            }
            (ship.0, ship.1, ship.2)
        });
    ship.0.abs() + ship.1.abs()
}

fn convert2(wayp: (i64, i64), direction: &str, angle: i64) -> Option<(i64, i64)> {
    let mut tuple = wayp.to_owned();
    let rotations = match angle {
        90 => 1,
        180 => 2,
        270 => 3,
        _ => 0,
    };

    let sign = match direction {
        "R" => (-1, 1),
        "L" => (1, -1),
        _ => (1, 1),
    };

    for _ in 0..rotations {
        tuple = (sign.0 * tuple.1, sign.1 * tuple.0);
    }

    Some(tuple)
}

fn task2(input: &str) -> i64 {
    let ship = input
        .lines()
        .map(|l| l.split_at(1))
        .map(|t| (t.0, t.1.parse::<i64>().unwrap()))
        .fold((0, 0, (1, 10)), |mut ship, x| {
            match x.0 {
                "N" => {
                    (ship.2).0 += x.1;
                }
                "S" => {
                    (ship.2).0 -= x.1;
                }
                "E" => {
                    (ship.2).1 += x.1;
                }
                "W" => {
                    (ship.2).1 -= x.1;
                }
                "R" => {
                    ship.2 = convert2(ship.2, "R", x.1).unwrap();
                }
                "L" => {
                    ship.2 = convert2(ship.2, "L", x.1).unwrap();
                }
                "F" => {
                    ship.0 += x.1 * (ship.2).0;
                    ship.1 += x.1 * (ship.2).1;
                }

                _ => {}
            }
            (ship.0, ship.1, ship.2)
        });
    ship.0.abs() + ship.1.abs()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read from file");

    let manhattan1 = task1(&file);
    let manhattan2 = task2(&file);

    println!("Task 1: Manhattan distance is {}", manhattan1);
    println!("Task 2: Manhattan distance is {}", manhattan2);
}
