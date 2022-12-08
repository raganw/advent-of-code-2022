use std::ops::Range;

use itertools::Itertools;

fn expand_range(input: &str) -> Range<u32> {
    let (start, end) = input
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();
    Range {
        start,
        end: end + 1,
    }
}

fn range_fully_contains_range(range_a: &Range<u32>, range_b: &Range<u32>) -> bool {
    range_a.contains(&range_b.start) && range_a.contains(&(range_b.end - 1))
}

fn range_contains_range(range_a: &Range<u32>, range_b: &Range<u32>) -> bool {
    range_a.contains(&range_b.start) || range_a.contains(&(range_b.end - 1))
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: usize = input
        .lines()
        .map(|pair| {
            let (range_a, range_b) = pair.split(',').map(expand_range).collect_tuple().unwrap();
            range_fully_contains_range(&range_a, &range_b)
                || range_fully_contains_range(&range_b, &range_a)
        })
        .filter(|&v| v)
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: usize = input
        .lines()
        .map(|pair| {
            let (range_a, range_b) = pair.split(',').map(expand_range).collect_tuple().unwrap();
            range_contains_range(&range_a, &range_b) || range_contains_range(&range_b, &range_a)
        })
        .filter(|&v| v)
        .count();
    dbg!(&result);

    Some(result as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
