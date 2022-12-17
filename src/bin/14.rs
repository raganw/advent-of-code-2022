use std::collections::BTreeSet;
use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Point = (u32, u32);

pub fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(nom_u32, tag(","), nom_u32)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points) = separated_list1(tag(" -> "), parse_point)(input)?;

    let points: Vec<Point> = points.into_iter().tuple_windows().flat_map(|(a, b)| {
        if a.0 == b.0 {
            (a.1.min(b.1)..=a.1.max(b.1)).map(|y| (a.0, y)).collect::<Vec<Point>>()
        } else {
            (a.0.min(b.0)..=a.0.max(b.0)).map(|x| (x, a.1)).collect::<Vec<Point>>()
        }
    }).collect();

    Ok((input, points))
}

pub fn parse_input(input: &str) -> IResult<&str, BTreeSet<Point>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    let output: BTreeSet<Point> = lines.into_iter().flatten().collect();
    Ok((input, output))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut grid) = parse_input(input).unwrap();
    let rock_quantity = grid.len();
    let lowest_rock = *grid.iter().map(|(_, y)| y).max().unwrap();

    let mut current_sand = (500, 0);

    loop {
        if current_sand.1 > lowest_rock {
            break;
        }
        let down = (current_sand.0, current_sand.1 + 1);
        let down_left = (current_sand.0 - 1, current_sand.1 + 1);
        let down_right = (current_sand.0 + 1, current_sand.1 + 1);

        if !grid.contains(&down) {
            current_sand = down;
        } else if !grid.contains(&down_left) {
            current_sand = down_left;
        } else if !grid.contains(&down_right) {
            current_sand = down_right;
        } else {
            grid.insert(current_sand);
            current_sand = (500, 0);
        }
    }
    let result = grid.len() - rock_quantity;

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut grid) = parse_input(input).unwrap();
    let rock_quantity = grid.len();
    let lowest_rock = *grid.iter().map(|(_, y)| y).max().unwrap();
    let lowest_rock = lowest_rock + 2;

    let mut current_sand = (500, 0);

    while grid.get(&(500, 0)).is_none() {
        let down = (current_sand.0, current_sand.1 + 1);
        let down_left = (current_sand.0 - 1, current_sand.1 + 1);
        let down_right = (current_sand.0 + 1, current_sand.1 + 1);

        if down.1 == lowest_rock {
            grid.insert(current_sand);
            current_sand = (500, 0);
        } else if !grid.contains(&down) {
            current_sand = down;
        } else if !grid.contains(&down_left) {
            current_sand = down_left;
        } else if !grid.contains(&down_right) {
            current_sand = down_right;
        } else {
            grid.insert(current_sand);
            current_sand = (500, 0);
        }
    }
    let result = grid.len() - rock_quantity;

    Some(result as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
