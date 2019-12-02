use std::convert::TryFrom;

enum IntCode {
    Add,
    Multiply,
    Halt,
}

impl TryFrom<u32> for IntCode {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(IntCode::Add),
            2 => Ok(IntCode::Multiply),
            99 => Ok(IntCode::Halt),
            _ => Err(format!("Invalid IntCode {}", value)),
        }
    }
}

#[aoc_generator(day2)]
pub fn generate_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

#[derive(Debug)]
struct ExecutionError;

impl From<std::num::TryFromIntError> for ExecutionError {
    fn from(_err: std::num::TryFromIntError) -> ExecutionError {
        ExecutionError
    }
}

impl From<()> for ExecutionError {
    fn from(_err: ()) -> ExecutionError {
        ExecutionError
    }
}

fn execute(input: &mut Vec<u32>) -> Result<(), ExecutionError> {
    let mut idx = 0;

    loop {
        match IntCode::try_from(input[idx]).unwrap() {
            IntCode::Add => {
                let a_loc = usize::try_from(*input.get(idx + 1).ok_or(())?)?;
                let b_loc = usize::try_from(*input.get(idx + 2).ok_or(())?)?;
                let c_loc = usize::try_from(*input.get(idx + 3).ok_or(())?)?;
                let a = input[a_loc];
                let b = input[b_loc];
                input[c_loc] = a + b;
            }
            IntCode::Multiply => {
                let a_loc = usize::try_from(*input.get(idx + 1).ok_or(())?)?;
                let b_loc = usize::try_from(*input.get(idx + 2).ok_or(())?)?;
                let c_loc = usize::try_from(*input.get(idx + 3).ok_or(())?)?;
                let a = input[a_loc];
                let b = input[b_loc];
                input[c_loc] = a * b;
            }
            IntCode::Halt => return Ok(()),
        }

        idx += 4;
    }
}

fn set_verb_and_noun(input: &mut Vec<u32>, verb: u32, noun: u32) {
    input[1] = verb;
    input[2] = noun;
}

#[aoc(day2, part1)]
pub fn solve_1(input: &[u32]) -> u32 {
    let mut input = input.to_vec();
    set_verb_and_noun(&mut input, 12, 2);
    execute(&mut input).unwrap();
    input[0]
}

#[aoc(day2, part2)]
pub fn solve_2(input: &[u32]) -> u32 {
    let mut buffer = input.to_vec();

    for x in 1..99 {
        for y in 1..99 {
            set_verb_and_noun(&mut buffer, x, y);
            match execute(&mut buffer) {
                Ok(_) => {
                    if buffer[0] == 19_690_720 {
                        return (x * 100) + y;
                    }
                    buffer = input.to_vec();
                }
                Err(_) => {
                    buffer = input.to_vec();
                }
            }
        }
    }
    panic!("Unable to find a match")
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_solves_problem_1_example_1() {
        let mut input = generate_input("1,9,10,3,2,3,11,0,99,30,40,50");
        execute(&mut input).unwrap();

        assert_eq!(input, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn it_solves_problem_1_example_2() {
        let mut input = generate_input("1,0,0,0,99");
        execute(&mut input).unwrap();

        assert_eq!(input, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn it_solves_problem_1_example_3() {
        let mut input = generate_input("2,3,0,3,99");
        execute(&mut input).unwrap();

        assert_eq!(input, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn it_solves_problem_1_example_4() {
        let mut input = generate_input("2,4,4,5,99,0");
        execute(&mut input).unwrap();

        assert_eq!(input, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn it_solves_problem_1_example_5() {
        let mut input = generate_input("1,1,1,4,99,5,6,0,99");
        execute(&mut input).unwrap();

        assert_eq!(input, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
