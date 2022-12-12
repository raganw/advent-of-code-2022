use itertools::Itertools;
use std::{collections::VecDeque, cell::RefCell};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
enum Operation {
    Mult(u64),
    Add(u64),
    Square,
}

impl Operation {
    fn perform_operation(&self, old: u64) -> u64 {
        match self {
            Operation::Square => old * old,
            Operation::Add(value) => old + value,
            Operation::Mult(value) => old * value,
        }
    }
}

#[derive(Debug)]
struct Test(u64);

impl Test {
    fn test(&self, value: u64) -> bool {
        value % self.0 == 0
    }
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    test: Test,
    true_target: usize,
    false_target: usize,
    inspection_count: u64,
}

/// Operation: new = old * 2
fn op_square(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("new = old * old")(input)?;
    Ok((input, Operation::Square))
}

fn op_add(input: &str) -> IResult<&str, Operation> {
    let (input, value) = preceded(tag("new = old + "), nom::character::complete::u64)(input)?;
    Ok((input, Operation::Add(value)))
}
fn op_mult(input: &str) -> IResult<&str, Operation> {
    let (input, value) = preceded(tag("new = old * "), nom::character::complete::u64)(input)?;
    Ok((input, Operation::Mult(value)))
}
fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) = alt((op_square, op_add, op_mult))(input)?;
    Ok((input, operation))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), nom::character::complete::u32, tag(":\n"))(input)?;
    let (input, items) = delimited(tag("  Starting items: "), separated_list1(tag(", "), nom::character::complete::u64), newline)(input)?;
    let (input, operation) = delimited(tag("  Operation: "), operation, newline)(input)?;
    let (input, test) = delimited(tag("  Test: divisible by "), nom::character::complete::u64, newline)(input)?;
    let (input, true_target) = delimited(tag("    If true: throw to monkey "), nom::character::complete::u32, newline)(input)?;
    let (input, false_target) = delimited(tag("    If false: throw to monkey "), nom::character::complete::u32, newline)(input)?;

    let items: RefCell<VecDeque<u64>> = RefCell::new(items.into_iter().collect());

    let monkey = Monkey {
        items,
        test: Test(test),
        operation,
        true_target: true_target as usize,
        false_target: false_target as usize,
        inspection_count: 0,
    };

    Ok((input, monkey))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(newline, monkey)(input)?;
    Ok((input, monkeys))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != b {
        let orig_a = a;
        a = a.abs_diff(b);
        b = orig_a.min(b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn monkey_business<F: Fn(u64) -> u64>(mut monkeys: Vec<Monkey>, rounds: u32, scaling_operation: F) -> u64 {
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            let mut target_monkey_and_items: Vec<(usize, u64)> = Vec::new();
            if let Some(monkey) = monkeys.get_mut(monkey_idx) {
                while let Some(item) = monkey.items.borrow_mut().pop_front() {
                    monkey.inspection_count += 1;
                    let new_value = scaling_operation(monkey.operation.perform_operation(item));
                    let target = if monkey.test.test(new_value) {
                        monkey.true_target
                    } else {
                        monkey.false_target
                    };
                    target_monkey_and_items.push((target, new_value));
                }
            }
            for (target, value) in target_monkey_and_items {
                if let Some(monkey) = monkeys.get_mut(target) {
                    monkey.items.borrow_mut().push_back(value);
                }
            }
        }
    }

    let mut result: Vec<u64> = monkeys.iter().map(|m| m.inspection_count).collect();
    result.sort();
    let (a, b) = result.iter().rev().take(2).collect_tuple().unwrap();
     a * b
}

/// Part One
pub fn part_one(input: &str) -> Option<u64> {
    let (_, monkeys) = parse_input(input).unwrap();
    let result = monkey_business(monkeys, 20, |val| val / 3);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, monkeys) = parse_input(input).unwrap();
    let scaling_factor = monkeys.iter().fold(1, |acc, monkey| lcm(acc, monkey.test.0));

    let result = monkey_business(monkeys, 10_000, |val| val % scaling_factor);
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }
}
