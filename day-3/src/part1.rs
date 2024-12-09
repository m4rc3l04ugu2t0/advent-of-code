fn mul(fts_left: i32, fts_right: i32) -> i32 {
    fts_left * fts_right
}

pub fn parse_input_01(input: &str) -> i32 {
    let binding = input.split_terminator("mul(").collect::<Vec<&str>>();
    let mut vec_l = Vec::new();
    let mut vec_r = Vec::new();
    let mut enable = true;
    for v in binding.iter() {
        if !enable {
            if v.contains("do()") {
                enable = true;
                continue;
            } else {
                continue;
            }
        }
        let (a, b) = v.split_once(",").unwrap_or(("", ""));
        let mut nuns = String::new();

        for c in b.chars() {
            match c {
                ')' => {
                    if let Ok(g) = a.parse::<i32>() {
                        vec_l.push(g);
                        vec_r.push(nuns.parse::<i32>().unwrap());
                    }
                    if v.contains("don't()") {
                        enable = false;
                    }
                    break;
                }
                '0'..='9' => nuns.push(c),
                _ => {
                    break;
                }
            }
        }
    }

    vec_l.into_iter().zip(vec_r).map(|(a, b)| mul(a, b)).sum()
}

#[test]
fn test_day_3() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(parse_input_01(input), 48);
}
