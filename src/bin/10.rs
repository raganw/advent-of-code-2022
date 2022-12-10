use std::{fmt, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::preceded,
    IResult, Finish,
};

#[derive(Debug)]
enum Operation {
    Noop,
    AddX(i32),
}

#[derive(Debug)]
enum ReturnType {
    MidCycle,
    EndOfCycle,
}

#[derive(Debug)]
struct Computer {
    register: i32,
    operations_stack: Vec<Operation>,
    operation_counter: u8,
    return_type: ReturnType,
}

impl Computer {
    fn new(mut operations: Vec<Operation>) -> Self {
        operations.reverse();
        Self {
            register: 1,
            operation_counter: 0,
            operations_stack: operations,
            return_type: ReturnType::MidCycle,
        }
    }
}

impl Iterator for Computer {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let register = self.register;
        if self.operations_stack.is_empty() {
            return None;
        }
        if let Some(operation) = self.operations_stack.last() {
            match operation {
                Operation::Noop => {
                    if self.operation_counter == 1 {
                        self.operations_stack.pop();
                        self.operation_counter = 0;
                    }
                },
                Operation::AddX(value) => {
                    if self.operation_counter == 2 {
                        self.register += value;
                        self.operations_stack.pop();
                        self.operation_counter = 0;
                    }
                }
            }
        }
        self.operation_counter += 1;

        match self.return_type {
            ReturnType::MidCycle => Some(register),
            ReturnType::EndOfCycle => Some(self.register)
        } 
    }
}

#[derive(Debug)]
struct Crt {
    screen: [[char; 40]; 6],
    current_row: usize,
    current_col: usize,
}

impl Crt {
    fn new() -> Self {
        Self {
            screen: [['.'; 40]; 6],
            current_row: 0,
            current_col: 0,
        }
    }

    fn set_pixel(&mut self, row: usize, col: usize) {
        self.screen[row][col] = '#';
    }
}

impl Iterator for Crt {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current_row == 5) && (self.current_col == 39) {
            return None;
        }
        let output = (self.current_row, self.current_col);
        self.current_col += 1;
        if self.current_col >= 40 {
            self.current_col = 0;
            self.current_row += 1;
        }

        Some(output)
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.screen {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

fn addx(input: &str) -> IResult<&str, Operation> {
    let (input, value) = preceded(tag("addx "), nom::character::complete::i32)(input)?;

    Ok((input, Operation::AddX(value)))
}

fn noop(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Operation::Noop))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(newline, alt((noop, addx)))(input)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, operations) = parse_input(input).finish().unwrap();
    let computer = Computer::new(operations);
    let output: Vec<(usize, i32)> = computer.enumerate().collect();
    let samples = vec![output[20], output[60], output[100], output[140], output[180], output[220]];
    let result: i32 = samples.iter().map(|(a, b)| *a as i32 * b).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, operations) = parse_input(input).finish().unwrap();
    let mut computer = Computer::new(operations);
    computer.return_type = ReturnType::EndOfCycle;
    let mut screen = Crt::new();
    for (pixel, register) in Crt::new().zip(computer) {
        let sprite_start = register - 1;
        let sprite_end = register + 1;
        if sprite_start <= pixel.1 as i32 && pixel.1 as i32 <= sprite_end {
            screen.set_pixel(pixel.0, pixel.1);
        }
    }
    println!("{}", screen);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
