use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Move {
    quantity: u32,
    start: usize,
    end: usize,
}

fn parse_start_state(input: &str) -> Vec<VecDeque<char>> {
    let lines: Vec<&str> = input.lines().collect();

    let (number_line_idx, num_columns) = lines
        .iter()
        .enumerate()
        .find(|(_, l)| l.contains('1'))
        .map(|(idx, line)| {
            (
                idx,
                line.trim()
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .unwrap();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_columns {
        stacks.push(VecDeque::new());
    }

    for line in lines.iter().take(number_line_idx) {
        for (idx, chars) in (&line.chars().chunks(4)).into_iter().enumerate() {
            let stack = stacks.get_mut(idx).unwrap();
            let elf_crate: String = chars.collect();
            let elf_crate = elf_crate.trim().replace(['[', ']'], "").chars().last();
            if let Some(c) = elf_crate {
                stack.push_back(c);
            }
        }
    }

    stacks
}

fn parse_instructions(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let (quantity, start, end) = l
                .split(' ')
                .filter_map(|sub| sub.parse::<u32>().ok())
                .collect_tuple()
                .unwrap();
            Move {
                quantity,
                start: start as usize - 1,
                end: end as usize - 1,
            }
        })
        .collect::<Vec<Move>>()
}
fn make_move(operation: Move, stacks: &mut Vec<VecDeque<char>>) {
    dbg!(&operation, &stacks);
    for _ in 0..operation.quantity {
        let c: char;
        {
            c = stacks
                .get_mut(operation.start)
                .unwrap()
                .pop_front()
                .unwrap();
        }
        stacks.get_mut(operation.end).unwrap().push_front(c);
    }
}

fn make_move_9001(operation: Move, stacks: &mut Vec<VecDeque<char>>) {
    dbg!(&operation, &stacks);
    let mut stack: VecDeque<char> = VecDeque::new();
    for _ in 0..operation.quantity {
        let c = stacks
            .get_mut(operation.start)
            .unwrap()
            .pop_front()
            .unwrap();
        stack.push_front(c);
    }
    for c in stack {
        stacks.get_mut(operation.end).unwrap().push_front(c)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (start_stack_str, instructions_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = parse_start_state(start_stack_str);
    let instructions = parse_instructions(instructions_str);
    for instruction in instructions {
        make_move(instruction, &mut stacks);
    }
    dbg!(&stacks);

    let result: String = stacks.iter().filter_map(|c| c.front()).collect();

    // dbg!(&start_stack_str, &instructions_str);
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (start_stack_str, instructions_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = parse_start_state(start_stack_str);
    let instructions = parse_instructions(instructions_str);
    for instruction in instructions {
        make_move_9001(instruction, &mut stacks);
    }
    dbg!(&stacks);

    let result: String = stacks.iter().filter_map(|c| c.front()).collect();

    // dbg!(&start_stack_str, &instructions_str);
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
