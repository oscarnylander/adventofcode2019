use std::convert::From;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

const CENTER_POINT: Point = Point { x: 0, y: 0 };

#[derive(Debug)]
pub struct Wire(Vec<Point>);

enum DirectionVector {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl From<&str> for DirectionVector {
    fn from(value: &str) -> Self {
        let first = value.chars().next().unwrap();
        let mut rest = value.chars();
        rest.next();
        let rest = rest.collect::<String>();
        let length = rest.parse::<i32>().unwrap();
        match first {
            'U' => DirectionVector::Up(length),
            'R' => DirectionVector::Right(length),
            'D' => DirectionVector::Down(length),
            'L' => DirectionVector::Left(length),
            _ => panic!("Unrecognized direction {}", first),
        }
    }
}

#[aoc_generator(day3)]
pub fn generate_input(input: &str) -> (Wire, Wire) {
    let mut wires = input
        .lines()
        .map(|l| {
            let mut x = 0;
            let mut y = 0;

            let mut res = Vec::<Point>::new();
            for dv in l.split(',').map(DirectionVector::from) {
                match dv {
                    DirectionVector::Up(l) => {
                        for _ in 0..l {
                            y += 1;
                            res.push(Point { x, y });
                        }
                    }
                    DirectionVector::Right(l) => {
                        for _ in 0..l {
                            x += 1;
                            res.push(Point { x, y });
                        }
                    }
                    DirectionVector::Down(l) => {
                        for _ in 0..l {
                            y -= 1;
                            res.push(Point { x, y });
                        }
                    }
                    DirectionVector::Left(l) => {
                        for _ in 0..l {
                            x -= 1;
                            res.push(Point { x, y });
                        }
                    }
                }
            }
            Wire(res)
        })
        .collect::<Vec<_>>();

    let w2 = wires.remove(1);
    let w1 = wires.remove(0);
    (w1, w2)
}

fn manhattan_distance(p1: &Point, p2: &Point) -> u32 {
    ((p2.x - p1.x).abs() + (p2.y - p1.y).abs())
        .try_into()
        .unwrap()
}

#[aoc(day3, part1)]
pub fn solve_1(input: &(Wire, Wire)) -> u32 {
    let (wire1, wire2) = input;

    let mut current_record = std::u32::MAX;

    for p1 in &wire1.0 {
        for p2 in &wire2.0 {
            if p1 == p2 {
                let res = manhattan_distance(&CENTER_POINT, p1);
                if res < current_record {
                    current_record = res;
                }
            }
        }
    }
    current_record
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_solves_part_1_example_1() {
        let wires = generate_input("R8,U5,L5,D3\nU7,R6,D4,L4");
        assert_eq!(solve_1(&wires), 6);
    }

    #[test]
    fn it_solves_part_1_example_2() {
        let wires =
            generate_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(solve_1(&wires), 159);
    }

    #[test]
    fn it_solves_part_1_example_3() {
        let wires = generate_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        assert_eq!(solve_1(&wires), 135);
    }
}
