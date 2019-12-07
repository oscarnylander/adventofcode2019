use crate::intcode::execute;

#[aoc_generator(day5)]
pub fn generate_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|n| n.parse::<_>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_1(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    execute(&mut input, 1).unwrap()
}
