fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day_1::part1::parse_input1(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    day_1::part2::parse_input2(divan::black_box(include_str!("../input.txt")));
}
