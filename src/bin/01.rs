fn parse_input(input: &str) -> Vec<u32> {
    let elf_inventories: Vec<&str> = input.split("\n\n").collect();
    dbg!(&elf_inventories.len());
    elf_inventories
        .into_iter()
        .map(|inv_str| {
            inv_str
                .trim()
                .lines()
                .map(|cal_str| cal_str.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_input(input).into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_calories: Vec<u32> = parse_input(input);
    elf_calories.sort();
    elf_calories.reverse();
    let top_3_sum: u32 = elf_calories.into_iter().take(3).sum();
    Some(top_3_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
