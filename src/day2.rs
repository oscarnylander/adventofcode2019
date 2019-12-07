use crate::intcode::execute;

#[aoc_generator(day2)]
pub fn generate_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|n| n.parse::<_>().unwrap())
        .collect()
}

fn set_verb_and_noun(input: &mut Vec<i32>, verb: i32, noun: i32) {
    input[1] = verb;
    input[2] = noun;
}

#[aoc(day2, part1)]
pub fn solve_1(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    set_verb_and_noun(&mut input, 12, 2);
    execute(&mut input, 0).unwrap();
    input[0]
}

#[aoc(day2, part2)]
pub fn solve_2(input: &[i32]) -> i32 {
    let mut buffer = input.to_vec();

    for x in 1..99 {
        for y in 1..99 {
            set_verb_and_noun(&mut buffer, x, y);
            match execute(&mut buffer, 0) {
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
        execute(&mut input, 0).unwrap();

        assert_eq!(input, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn it_solves_problem_1_example_2() {
        let mut input = generate_input("1,0,0,0,99");
        execute(&mut input, 0).unwrap();

        assert_eq!(input, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn it_solves_problem_1_example_3() {
        let mut input = generate_input("2,3,0,3,99");
        execute(&mut input, 0).unwrap();

        assert_eq!(input, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn it_solves_problem_1_example_4() {
        let mut input = generate_input("2,4,4,5,99,0");
        execute(&mut input, 0).unwrap();

        assert_eq!(input, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn it_solves_problem_1_example_5() {
        let mut input = generate_input("1,1,1,4,99,5,6,0,99");
        execute(&mut input, 0).unwrap();

        assert_eq!(input, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
