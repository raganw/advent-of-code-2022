pub fn part_one(input: &str) -> Option<i32> {
    let input = input.lines().map(|a| a.parse::<i32>().unwrap()).enumerate().collect::<Vec<(usize, i32)>>();

    let mut output = input.clone();

    for (idx, delta) in input {
        let current_idx = output.iter().position(|(i, _)| *i == idx).unwrap();
        let value = output.remove(current_idx);
        let new_idx = (current_idx as i32 + delta).rem_euclid(output.len() as i32);
        output.insert(new_idx as usize, value);
    }

    let zero_position = output.iter().position(|(_, v)| *v == 0).unwrap();

    let cycle = output.iter().map(|(_, v)| v).cycle().skip(zero_position);
    let mut cycle = cycle.skip(1000);
    let value_1 = cycle.next().unwrap();
    let mut cycle = cycle.skip(999);
    let value_2 = cycle.next().unwrap();
    let mut cycle = cycle.skip(999);
    let value_3 = cycle.next().unwrap();

    let result = value_1 + value_2 + value_3;

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let decryption_key = 811589153;
    let input = input.lines().map(|a| a.parse::<i64>().unwrap() * decryption_key).enumerate().collect::<Vec<(usize, i64)>>();

    let mut output = input.clone();

    for _ in 0..10 {
        for (idx, delta) in &input {
            let current_idx = output.iter().position(|(i, _)| *i == *idx).unwrap();
            let value = output.remove(current_idx);
            let new_idx = (current_idx as i64 + delta).rem_euclid(output.len() as i64);
            output.insert(new_idx as usize, value);
        }
    }

    let zero_position = output.iter().position(|(_, v)| *v == 0).unwrap();

    let cycle = output.iter().map(|(_, v)| v).cycle().skip(zero_position);
    let mut cycle = cycle.skip(1000);
    let value_1 = cycle.next().unwrap();
    let mut cycle = cycle.skip(999);
    let value_2 = cycle.next().unwrap();
    let mut cycle = cycle.skip(999);
    let value_3 = cycle.next().unwrap();

    let result = value_1 + value_2 + value_3;

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
