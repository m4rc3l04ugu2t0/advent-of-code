use day_3::{part1::parse_input_01, part2::parse_input_02};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    parse_input_01(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    parse_input_02(divan::black_box(include_str!("../input.txt"))).unwrap();
}
