pub fn parse_input1(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().lines().collect();

    // Inicializar listas
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Dividir cada linha e extrair valores
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

    // Ordenar as listas
    left.sort_unstable();
    right.sort_unstable();

    // Calcular a soma das diferenças absolutas
    let total_distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    total_distance
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
    assert_eq!(parse_input1(input), 11);
}
