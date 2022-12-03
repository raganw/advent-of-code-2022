use itertools::Itertools;

pub fn find_priority(item_code: char) -> u32 {
    let normalizer = if item_code.is_lowercase() { 'a' as u32 } else { 'A' as u32 };
    (item_code as u32) - normalizer + (if item_code.is_lowercase() { 1 } else { 27 })
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input.lines().map(|sack| {
        let (compartment_a, compartment_b) = sack.split_at(sack.len() / 2);
        let common_item = compartment_a.chars().find(|&c| compartment_b.contains(c)).unwrap();
        find_priority(common_item)
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<&str> = input.lines().collect();
    dbg!(&rucksacks);
    let mut badge_ids: Vec<char> = Vec::new();

    for chunk in &rucksacks.into_iter().chunks(3) {
        let (a, b, c) = chunk.collect_tuple().unwrap();
        let common_char = a.chars().find(|&ch| b.contains(ch) && c.contains(ch)).unwrap();
        badge_ids.push(common_char);
    }
    let result: u32 = badge_ids.iter().map(|c| find_priority(*c)).sum();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
