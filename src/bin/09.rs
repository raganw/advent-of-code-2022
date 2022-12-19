use advent_of_code::point::Point;
use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Finish,
};

#[derive(Debug)]
enum Operation {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Knot {
    point: Point,
    visited: HashSet<Point>,
}

impl Default for Knot {
    fn default() -> Self {
        let mut v = Self {
            point: Point::default(),
            visited: HashSet::new(),
        };
        v.visited.insert(Point::default());
        v
    }
}

impl Knot {
    fn move_knot(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.point.y += 1,
            Direction::Down => self.point.y -= 1,
            Direction::Left => self.point.x -= 1,
            Direction::Right => self.point.x += 1,
        };
    }

    fn is_adjacent_or_diagonal(&self, knot_b: &Self) -> bool {
        self.point.distance(&knot_b.point) < 2_f32
    }

    fn move_towards(&mut self, knot_b: &Self) {
        if self.is_adjacent_or_diagonal(knot_b) {
            return;
        }
        // on the x axis
        match self.point.x.cmp(&knot_b.point.x) {
            std::cmp::Ordering::Less => {
                self.move_knot(Direction::Right)
            },
            std::cmp::Ordering::Greater => {
                self.move_knot(Direction::Left)
            },
            std::cmp::Ordering::Equal => {},
        }
        // on the y axis
        match self.point.y.cmp(&knot_b.point.y) {
            std::cmp::Ordering::Less => {
                self.move_knot(Direction::Up)
            },
            std::cmp::Ordering::Greater => {
                self.move_knot(Direction::Down)
            },
            std::cmp::Ordering::Equal => {},
        }

        self.visited.insert(self.point);
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(num_knots: u32) -> Self {
        Self {
            knots: (0..num_knots).into_iter().map(|_| Knot::default()).collect_vec(),
        }
    }
    fn make_move(&mut self, direction: Direction) {
        self.knots.first_mut().unwrap().move_knot(direction);

        for i in 0..(self.knots.len() - 1) {
            if let Some(knot_to_move_to) = self.knots.get(i).cloned() {
                if let Some(knot_to_move) = self.knots.get_mut(i + 1) {
                    knot_to_move.move_towards(&knot_to_move_to);
                }
            }
        }
    }

    fn num_visited(&self, index: usize) -> u32 {
        self.knots.get(index).unwrap().visited.len() as u32
    }
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, (op, quantity)) =
        separated_pair(alt((tag("L"), tag("R"), tag("U"), tag("D"))), tag(" "), nom::character::complete::u32)(input)?;

    let operation = match op {
        "L" => Operation::Left(quantity),
        "R" => Operation::Right(quantity),
        "U" => Operation::Up(quantity),
        "D" => Operation::Down(quantity),
        _ => unreachable!("invalid direction")
    };
    Ok((input, operation))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(newline, operation)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, operations) = parse_input(input).finish().unwrap();
    let mut rope = Rope::new(2);

    for operation in operations.iter() {
        match operation {
            Operation::Left(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Left);
                }
            },
            Operation::Right(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Right);
                }
            },
            Operation::Up(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Up);
                }
            },
            Operation::Down(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Down);
                }
            },
        }
    }

    let num_visited = rope.num_visited(1);
    Some(num_visited)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, operations) = parse_input(input).finish().unwrap();
    let mut rope = Rope::new(10);

    for operation in operations.iter() {
        match operation {
            Operation::Left(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Left);
                }
            },
            Operation::Right(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Right);
                }
            },
            Operation::Up(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Up);
                }
            },
            Operation::Down(i) => {
                for _ in 0..*i {
                    rope.make_move(Direction::Down);
                }
            },
        }
    }

    let num_visited = rope.num_visited(9);
    Some(num_visited)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
