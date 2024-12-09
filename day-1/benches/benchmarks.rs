use day_1::{part_1::parse_input1, part_2::parse_input2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    parse_input1(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    parse_input2(divan::black_box(include_str!("../input.txt")));
}
