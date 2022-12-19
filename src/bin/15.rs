use std::{collections::{BTreeMap, BTreeSet, HashSet}, ops::RangeInclusive};

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
use advent_of_code::point::Point;

use nom::{IResult, multi::separated_list1, character::complete::{i32 as nom_i32, newline}, sequence::{delimited, preceded, separated_pair}, bytes::complete::tag};

type Sensor = Point;
type Beacon = Point;

// x=2, y=18
fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(preceded(tag("x="), nom_i32), tag(", y="), nom_i32)(input)?;
    Ok((input, Point { x, y }))
}

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, (sensor, beacon)) = preceded(tag("Sensor at "), separated_pair(parse_point, tag(": closest beacon is at "), parse_point))(input)?;

    Ok((input, (sensor, beacon)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Sensor, Beacon)>> {
    separated_list1(newline, parse_line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: BTreeMap<i32, Vec<RangeInclusive<i32>>> = BTreeMap::new();
    let mut beacon_y_count: BTreeMap<i32, HashSet<Point>> = BTreeMap::new();
    let (_, sensors) = parse_input(input).unwrap();

    for (sensor, beacon) in sensors.iter() {
        let distance = sensor.manhattan_distance(&beacon) as i32;
        beacon_y_count.entry(beacon.y).and_modify(|v| { v.insert(beacon.clone()); }).or_default();
        for delta_y in -distance..=distance {
            let current_y = sensor.y + delta_y;
            let half_x_range = distance - delta_y.abs();
            let range = (sensor.x - half_x_range)..=(sensor.x + half_x_range);
            grid.entry(current_y).and_modify(|v| v.push(range.clone()) ).or_default();
        }
    }

    // dbg!(&grid);
    // let interesting_row = 2000000;
    let interesting_row = 10;

    let row = grid.get(&interesting_row).unwrap();
    let beacons = beacon_y_count.get(&interesting_row).map(|v| v.len()).unwrap_or(0_usize);
    dbg!(&beacons);
    let result = row.into_iter().fold(BTreeSet::new(), |mut acc, v| {
        acc.extend(v.clone().into_iter());
        acc
    }).len() - beacons;
    dbg!(&result);

    // dbg!(&sensors);
    Some(result as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
