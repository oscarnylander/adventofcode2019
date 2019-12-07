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

#[aoc(day5, part2)]
pub fn solve_2(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    execute(&mut input, 5).unwrap()
}

mod tests {
    #[allow(unused_imports)]
    use crate::intcode::execute;
    #[allow(unused_imports)]
    use super::generate_input;

    #[test]
    fn it_solves_part_2_example_1() {
        let mut input = generate_input("3,9,8,9,10,9,4,9,99,-1,8");
        let mut input_copy = input.clone();

        assert_eq!(execute(&mut input, 8).unwrap(), 1);
        assert_eq!(execute(&mut input_copy, 7).unwrap(), 0);
    }

    #[test]
    fn it_solves_part_2_example_2() {
        let mut input = generate_input("3,9,7,9,10,9,4,9,99,-1,8");
        let mut input_copy = input.clone();

        assert_eq!(execute(&mut input, 5).unwrap(), 1);
        assert_eq!(execute(&mut input_copy, 9).unwrap(), 0);
    }

    #[test]
    fn it_solves_part_2_example_3() {
        let mut input = generate_input("3,3,1108,-1,8,3,4,3,99");
        let mut input_copy = input.clone();

        assert_eq!(execute(&mut input, 8).unwrap(), 1);
        assert_eq!(execute(&mut input_copy, 9).unwrap(), 0);
    }

    #[test]
    fn it_solves_part_2_example_4() {
        let mut input = generate_input("3,3,1107,-1,8,3,4,3,99");
        let mut input_copy = input.clone();

        assert_eq!(execute(&mut input, 5).unwrap(), 1);
        assert_eq!(execute(&mut input_copy, 9).unwrap(), 0);
    }

    #[test]
    fn it_solves_part_2_example_5() {
        let mut input = generate_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut input_copy = input.clone();

        assert_eq!(execute(&mut input, 5).unwrap(), 1);
        assert_eq!(execute(&mut input_copy, 0).unwrap(), 0);
    }

    #[test]
    fn it_solves_part_2_example_6() {
        let mut input = generate_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let mut input_copy = input.clone();

        assert_eq!(execute(&mut input, 3).unwrap(), 1);
        assert_eq!(execute(&mut input_copy, 0).unwrap(), 0);
    }
}
