use std::collections::HashSet;

fn find_signal(input: &str, window_size: usize) -> Option<u32> {
    let mut result: Option<u32> = None;
    for i in 0..(input.len() - window_size) {
        let window = &input[i..i + window_size];
        let set: HashSet<char> = HashSet::from_iter(window.chars());
        if set.len() == window_size {
            result = Some((i + window_size) as u32);
            break;
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    find_signal(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_signal(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
