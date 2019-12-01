use std::convert::TryInto;

#[aoc_generator(day1)]
pub fn generate_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_1(input: &[u32]) -> u32 {
    input
        .iter()
        .map(|i| (i / 3) - 2)
        .sum()
}

fn get_fuel_by_mass(mass: u32) -> u32 {
    let possibly_negative_mass: i32 = mass.try_into().unwrap();
    let fuel = (possibly_negative_mass / 3) - 2;
    if fuel <= 0 {
        0
    } else {
        fuel.try_into().unwrap()
    }
}

fn get_total_fuel_for_module(mass: u32) -> u32 {
    let mut total_fuel = 0;

    let mut current_fuel = get_fuel_by_mass(mass);
    while current_fuel != 0 {
        total_fuel += current_fuel;
        current_fuel = get_fuel_by_mass(current_fuel);
    }

    total_fuel
}

#[aoc(day1, part2)]
pub fn solve_2(input: &[u32]) -> u32 {
    input
        .iter()
        .map(|i| get_total_fuel_for_module(*i))
        .sum()
}
