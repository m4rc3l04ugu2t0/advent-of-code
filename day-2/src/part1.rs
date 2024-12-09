// error
pub fn parse_input_01(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .into_iter()
        .map(|s| {
            let mut nums = s
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut count = 0;
            let mut previews_number = nums.remove(0);
            let state = previews_number > nums[0];

            for v in nums.into_iter() {
                if state {
                    if (previews_number - v).abs() > 3
                        || previews_number <= v
                        || previews_number == v
                    {
                        count = 0;
                        break;
                    } else {
                        count = 1;
                        previews_number = v;
                    }
                } else {
                    if previews_number <= v || previews_number == v {
                        count = 0;
                        break;
                    }
                    if (previews_number - v).abs() > 3 {
                        count = 0;
                        break;
                    } else {
                        count = 1;
                        previews_number = v;
                    }
                }
            }
            count
        })
        .sum()
}

#[test]
fn test_day_2() {
    let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
    ";
    assert_eq!(parse_input_01(input), 2);
}
