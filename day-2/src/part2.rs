use itertools::Itertools;

enum Direction {
    Increasing,
    Descreasing,
}

type Report = Vec<i32>;

fn check_safety(report: &Report) -> Result<(), ()> {
    let mut direction: Option<Direction> = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(());
                }
                Some(Direction::Descreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(());
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(());
                    } else {
                        direction = Some(Direction::Descreasing);
                        continue;
                    }
                }
            },
            1 => match direction {
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(());
                    } else {
                        continue;
                    }
                }
                Some(Direction::Descreasing) => {
                    return Err(());
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(());
                    } else {
                        direction = Some(Direction::Increasing);
                        continue;
                    }
                }
            },
            0 => {
                return Err(());
            }
            _ => panic!(),
        }
    }
    Ok(())
}

pub fn parse_input_02(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|r| {
            if check_safety(r).is_err() {
                for index in 0..r.len() {
                    let mut new_report = (*r).clone();
                    new_report.remove(index);
                    if check_safety(&new_report).is_ok() {
                        return true;
                    } else {
                        continue;
                    }
                }
                false
            } else {
                true
            }
        })
        .count()
        .to_string()
}

#[test]
fn test_day_2() {
    let input = "
10 40 9 8 7
    ";
    let result = parse_input_02(input);
    assert_eq!(result, "2");
}
