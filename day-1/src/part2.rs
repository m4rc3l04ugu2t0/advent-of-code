pub fn parse_input2(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        if let [l, r] = numbers[..] {
            left.push(l);
            right.push(r);
        }
    }

    left.iter()
        .map(|v| {
            if right.contains(&v) {
                let mutt = right.iter().fold(0, |acc, n| {
                    if n == v {
                        return acc + 1;
                    }

                    acc
                });

                return v * mutt;
            }
            0
        })
        .sum()
}

#[test]
fn test() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
    ";
    assert_eq!(parse_input2(input), 31);
}
