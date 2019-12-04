#[aoc_generator(day4)]
pub fn generate_input(input: &str) -> (u32, u32) {
    let (first, second) = input.split_at(6);
    (
        first.parse::<u32>().unwrap(),
        second.replace('-', "").parse::<u32>().unwrap(),
    )
}

#[aoc(day4, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn solve_1(input: &(u32, u32)) -> u32 {
    let mut matches = 0;
    for i in input.0..input.1 {
        if matches_criteria_1(&i.to_string()) {
            matches += 1;
        }
    }
    matches
}

fn matches_criteria_1(input: &str) -> bool {
    let chars = input.chars().collect::<Vec<_>>();

    let mut has_seen_pair = false;

    for i in 0..5 {
        let a = chars[i];
        let b = chars[i + 1];
        let a_digit = a.to_digit(10).unwrap();
        let b_digit = b.to_digit(10).unwrap();
        if b_digit < a_digit {
            return false;
        }
        if a == b {
            has_seen_pair = true;
        }
    }

    has_seen_pair
}

#[aoc(day4, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn solve_2(input: &(u32, u32)) -> u32 {
    let mut matches = 0;
    for i in input.0..input.1 {
        if matches_criteria_2(&i.to_string()) {
            matches += 1;
        }
    }
    matches
}

fn matches_criteria_2(input: &str) -> bool {
    let chars = input.chars().collect::<Vec<_>>();

    let mut has_seen_pair_without_additional = false;

    for i in 0..5 {
        let a = chars[i];
        let b = chars[i + 1];
        let a_digit = a.to_digit(10).unwrap();
        let b_digit = b.to_digit(10).unwrap();
        if b_digit < a_digit {
            return false;
        }
        if a == b {
            match i {
                0 => {
                    let c = chars[i + 2];
                    if b != c {
                        has_seen_pair_without_additional = true;
                    }
                }
                1..=3 => {
                    let c = chars[i + 2];
                    let d = chars[i - 1];

                    if a != d && c != b {
                        has_seen_pair_without_additional = true;
                    }
                }
                4 => {
                    let c = chars[i - 1];

                    if a != c {
                        has_seen_pair_without_additional = true;
                    }
                }
                _ => {}
            }
        }
    }

    if has_seen_pair_without_additional {
        println!("Considering {} a match", input);
    }

    has_seen_pair_without_additional
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_classifies_part_1_example_1() {
        assert_eq!(matches_criteria_1("111111"), true);
    }

    #[test]
    fn it_classifies_part_1_example_2() {
        assert_eq!(matches_criteria_1("223450"), false);
    }

    #[test]
    fn it_classifies_part_1_example_3() {
        assert_eq!(matches_criteria_1("123789"), false);
    }

    #[test]
    fn it_classifies_400000_correctly() {
        assert_eq!(matches_criteria_1("400000"), false);
    }

    #[test]
    fn it_classifies_part_2_example_1() {
        assert_eq!(matches_criteria_2("112233"), true);
    }

    #[test]
    fn it_classifies_part_2_example_2() {
        assert_eq!(matches_criteria_2("123444"), false);
    }

    #[test]
    fn it_classifies_part_2_example_3() {
        assert_eq!(matches_criteria_2("111122"), true);
    }

    #[test]
    fn it_classifies_388889_correctly() {
        assert_eq!(matches_criteria_2("388889"), false);
    }

    #[test]
    fn it_classifies_889999_correctly() {
        assert_eq!(matches_criteria_2("889999"), true);
    }
}
