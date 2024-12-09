use day_3::part1::parse_input_01;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    parse_input_01(divan::black_box(include_str!("../input.txt")));
}
