use itertools::Itertools;
use std::{collections::VecDeque, cell::RefCell};

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::newline,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
struct Operation<'a>(&'a str);

impl <'a> Operation<'a> {
    fn perform_operation(&self, old: u64) -> u64 {
        match self.0 {
            "old * 19" => old * 19,
            "old + 6" => old + 6,
            "old * old" => old * old,
            "old + 3" => old + 3,
            "old * 2" => old * 2,
            "old + 2" => old + 2,
            "old * 11" => old * 11,
            "old + 7" => old + 7,
            "old + 1" => old + 1,
            "old + 5" => old + 5,
            _ => unimplemented!("The operation '{}' is unimplemented", self.0),
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
struct Monkey<'a> {
    items: RefCell<VecDeque<u64>>,
    operation: Operation<'a>,
    test: Test,
    true_target: usize,
    false_target: usize,
    inspection_count: u64,
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), nom::character::complete::u32, tag(":\n"))(input)?;
    let (input, items) = delimited(tag("  Starting items: "), separated_list1(tag(", "), nom::character::complete::u64), newline)(input)?;
    let (input, operation) = delimited(tag("  Operation: new = "), take_until1("\n"), newline)(input)?;
    let (input, test) = delimited(tag("  Test: divisible by "), nom::character::complete::u64, newline)(input)?;
    let (input, true_target) = delimited(tag("    If true: throw to monkey "), nom::character::complete::u32, newline)(input)?;
    let (input, false_target) = delimited(tag("    If false: throw to monkey "), nom::character::complete::u32, newline)(input)?;

    let items: RefCell<VecDeque<u64>> = RefCell::new(items.into_iter().collect());

    let monkey = Monkey {
        items,
        test: Test(test),
        operation: Operation(operation),
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

/// Part One
pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut monkeys) = parse_input(input).unwrap();

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let mut target_monkey_and_items: Vec<(usize, u64)> = Vec::new();
            if let Some(monkey) = monkeys.get_mut(monkey_idx) {
                while let Some(item) = monkey.items.borrow_mut().pop_front() {
                    monkey.inspection_count += 1;
                    let new_value = monkey.operation.perform_operation(item) / 3;
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
    let result = a * b;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut monkeys) = parse_input(input).unwrap();
    let scaling_factor = monkeys.iter().fold(1, |acc, monkey| lcm(acc, monkey.test.0));

    for _ in 0..10_000 {
        for monkey_idx in 0..monkeys.len() {
            let mut target_monkey_and_items: Vec<(usize, u64)> = Vec::new();
            if let Some(monkey) = monkeys.get_mut(monkey_idx) {
                while let Some(item) = monkey.items.borrow_mut().pop_front() {
                    monkey.inspection_count += 1;
                    let new_value = monkey.operation.perform_operation(item);
                    let new_value = new_value % scaling_factor;
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
    let result = a * b;

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
