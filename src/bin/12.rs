use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let map = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let start = map.iter().enumerate().filter_map(|(i, l)| l.iter().find_position(|&&c| c == 'S').map(|(j, _)| (i, j))).last().unwrap();
    let end = map.iter().enumerate().filter_map(|(i, l)| l.iter().find_position(|&&c| c == 'E').map(|(j, _)| (i, j))).last().unwrap();
    dbg!(&map, &start, &end);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
