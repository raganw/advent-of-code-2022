#![allow(unused_variables, dead_code)]

use std::collections::{HashMap, VecDeque, HashSet};

use nom::{
    *,
    IResult,
    branch::alt,
    multi::separated_list1,
    character::complete::{newline, anychar, alpha1, i64 as nom_i64},
    sequence::{tuple, separated_pair},
    bytes::complete::tag,
};

#[derive(Debug, Clone, Copy)]
enum Operation<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mult(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl <'a> Operation<'a> {
    fn get_listeners(&self) -> (&'a str, &'a str) {
        match self {
            Self::Add(a, b) => (a, b),
            Self::Sub(a, b) => (a, b),
            Self::Mult(a, b) => (a, b),
            Self::Div(a, b) => (a, b),
        }
    }
    fn run(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add(_, _) => a + b,
            Self::Sub(_, _) => a - b,
            Self::Mult(_, _) => a * b,
            Self::Div(_, _) => a / b,
        }
    }
    fn run_inverse(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add(_, _) => a - b,
            Self::Sub(_, _) => a + b,
            Self::Mult(_, _) => a / b,
            Self::Div(_, _) => a * b,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Job<'a> {
    Operation(Operation<'a>),
    Yell(i64),
}

#[derive(Debug, Clone, Copy)]
struct Monkey<'a> {
    name: &'a str,
    job: Job<'a>
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (monkey_1, _, op, _, monkey_2)) = tuple((alpha1, tag(" "), anychar, tag(" "), alpha1))(input)?;
    let operation = match op {
        '+' => Operation::Add(monkey_1, monkey_2),
        '-' => Operation::Sub(monkey_1, monkey_2),
        '*' => Operation::Mult(monkey_1, monkey_2),
        '/' => Operation::Div(monkey_1, monkey_2),
        _ => unreachable!("invalid operation")
    };

    Ok((input, operation))
}

fn parse_job(input: &str) -> IResult<&str, Job> {
    alt((parse_operation.map(Job::Operation), nom_i64.map(Job::Yell)))(input)
}

fn parse_line(input: &str) -> IResult<&str, Monkey> {
    let (input, (name, job)) = separated_pair(alpha1, tag(": "), parse_job)(input)?;

    Ok((input, Monkey { name, job }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_line)(input)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, monkeys) = parse_input(input).unwrap();
    let mut monkey_yells: HashMap<&str, i64> = HashMap::new();
    let mut monkey_listeners: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut monkey_workers: VecDeque<Monkey> = VecDeque::new();

    for monkey in monkeys {
        match monkey.job {
            Job::Yell(yell) => { monkey_yells.insert(monkey.name, yell); },
            Job::Operation(op) => {
                let (a, b) = op.get_listeners();
                monkey_listeners.entry(a).and_modify(|v| v.push(monkey.name)).or_insert(vec![monkey.name]);
                monkey_listeners.entry(b).and_modify(|v| v.push(monkey.name)).or_insert(vec![monkey.name]);
                monkey_workers.push_back(monkey);
            }
        }
    }

    while let Some(monkey) = monkey_workers.pop_front() {
        match monkey.job {
            Job::Operation(op) => {
                let (a, b) = op.get_listeners();
                if let Some((a, b)) = monkey_yells.get(a).and_then(|a| monkey_yells.get(b).map(|b| (a, b))) {
                    let yell = op.run(*a, *b);
                    monkey_yells.insert(monkey.name, yell);
                    if monkey.name == "root" {
                        break;
                    }
                } else {
                    monkey_workers.push_back(monkey);
                }
            }
            _ => { unreachable!("yell in workers"); }
        }
    }
    monkey_yells.get("root").map(|v| *v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, monkeys) = parse_input(input).unwrap();
    let mut monkey_yells: HashMap<&str, i64> = HashMap::new();
    let mut monkey_listeners: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut monkey_workers: VecDeque<Monkey> = VecDeque::new();

    let mut branch_1: HashSet<&str> = HashSet::new();
    let mut branch_2: HashSet<&str> = HashSet::new();

    let (root_1, root_2) = monkeys.iter().find(|m| m.name == "root").and_then(|m| {
        if let Job::Operation(op) = m.job {
            Some(op.get_listeners())
        } else {
            None
        }
    }).unwrap();

    let mut to_check: VecDeque<&str> = vec![root_1].into();
    while let Some(current) = to_check.pop_front() {
        branch_1.insert(current);
        if let Some(m) = monkeys.iter().find(|m| m.name == current) {
            if let Job::Operation(op) = m.job {
                let (a, b) = op.get_listeners();
                if branch_1.get(a).is_none() {
                    to_check.push_back(a);
                }
                if branch_1.get(b).is_none() {
                    to_check.push_back(b);
                }
            }
        }
    }
    let mut to_check: VecDeque<&str> = vec![root_2].into();
    while let Some(current) = to_check.pop_front() {
        if let Some(m) = monkeys.iter().find(|m| m.name == current) {
            branch_2.insert(current);
            if let Job::Operation(op) = m.job {
                let (a, b) = op.get_listeners();
                if branch_2.get(a).is_none() {
                    to_check.push_back(a);
                }
                if branch_2.get(b).is_none() {
                    to_check.push_back(b);
                }
            }
        }
    }
    let non_human_root = if branch_1.contains("humn") { root_1 } else { root_2 };
    dbg!(root_1, root_2, non_human_root);

    for monkey in monkeys.clone() {
        match monkey.job {
            Job::Yell(yell) => { monkey_yells.insert(monkey.name, yell); },
            Job::Operation(op) => {
                let (a, b) = op.get_listeners();
                monkey_listeners.entry(a).and_modify(|v| v.push(monkey.name)).or_insert(vec![monkey.name]);
                monkey_listeners.entry(b).and_modify(|v| v.push(monkey.name)).or_insert(vec![monkey.name]);
                monkey_workers.push_back(monkey);
            }
        }
    }

    while let Some(monkey) = monkey_workers.pop_front() {
        match monkey.job {
            Job::Operation(op) => {
                let (a, b) = op.get_listeners();
                if let Some((a, b)) = monkey_yells.get(a).and_then(|a| monkey_yells.get(b).map(|b| (a, b))) {
                    let yell = op.run(*a, *b);
                    monkey_yells.insert(monkey.name, yell);
                    if monkey.name == non_human_root {
                        break;
                    }
                } else {
                    monkey_workers.push_back(monkey);
                }
            }
            _ => { unreachable!("yell in workers"); }
        }
    }
    let mut current_value = monkey_yells.get(non_human_root).map(|v| *v).unwrap();

    let mut to_check: VecDeque<&str> = vec![non_human_root].into();
    let mut monkey_yells: HashMap<&str, i64> = HashMap::new();
    while let Some(monkey) = to_check.pop_front() {
        let monkey = monkeys.iter().find(|m| m.name == "root").unwrap();
        dbg!(monkey, current_value);
        match monkey.job {
            Job::Operation(op) => {
                let (a, b) = op.get_listeners();
                let a_yell = monkey_yells.get(a);
                let b_yell = monkey_yells.get(b);
                dbg!(a_yell, b_yell);
            },
            Job::Yell(yell) => {
                monkey_yells.insert(monkey.name, yell);
            }
        }
    }

    dbg!(monkey_yells.get("humn"));

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
