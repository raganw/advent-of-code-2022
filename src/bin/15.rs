use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    ops::{ControlFlow, RangeInclusive},
};

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
use advent_of_code::point::Point;

use nom::{
    bytes::complete::tag,
    character::complete::{i32 as nom_i32, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

type Sensor = Point;
type Beacon = Point;

// x=2, y=18
fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) =
        separated_pair(preceded(tag("x="), nom_i32), tag(", y="), nom_i32)(input)?;
    Ok((input, Point { x, y }))
}

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, (sensor, beacon)) = preceded(
        tag("Sensor at "),
        separated_pair(parse_point, tag(": closest beacon is at "), parse_point),
    )(input)?;

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
        let distance = sensor.manhattan_distance(beacon) as i32;
        beacon_y_count
            .entry(beacon.y)
            .and_modify(|v| {
                v.insert(*beacon);
            })
            .or_default();
        for delta_y in -distance..=distance {
            let current_y = sensor.y + delta_y;
            let half_x_range = distance - delta_y.abs();
            let range = (sensor.x - half_x_range)..=(sensor.x + half_x_range);
            grid.entry(current_y)
                .and_modify(|v| v.push(range.clone()))
                .or_default();
        }
    }

    // dbg!(&grid);
    // let interesting_row = 2000000;
    let interesting_row = 10;

    let row = grid.get(&interesting_row).unwrap();
    let beacons = beacon_y_count
        .get(&interesting_row)
        .map(|v| v.len())
        .unwrap_or(0_usize);
    dbg!(&beacons);
    let result = row
        .iter()
        .fold(BTreeSet::new(), |mut acc, v| {
            acc.extend(v.clone().into_iter());
            acc
        })
        .len()
        - beacons;
    dbg!(&result);

    // dbg!(&sensors);
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    // let max_range = 4_000_000;
    let max_range = 20;
    let mut grid: BTreeMap<i32, Vec<RangeInclusive<i32>>> = BTreeMap::new();
    let (_, sensors) = parse_input(input).unwrap();

    for (sensor, beacon) in sensors.iter() {
        let distance = sensor.manhattan_distance(beacon) as i32;
        for delta_y in -distance..=distance {
            let current_y = sensor.y + delta_y;
            if current_y < 0 || current_y > max_range {
                continue;
            }
            let half_x_range = distance - delta_y.abs();
            let range = 0.max(sensor.x - half_x_range)..=max_range.min(sensor.x + half_x_range);
            grid.entry(current_y)
                .and_modify(|v| {
                    v.push(range.clone());
                    v.sort_by(|a, b| a.start().cmp(b.start()));
                })
                .or_insert_with(|| vec![range.clone()]);
        }
    }

    let distress_beacon_location: Point = grid
        .into_iter()
        .filter_map(|(y, v)| {
            match v.iter().try_fold(0..=0, |acc, r| {
                if r.start() <= &(*acc.end() + 1) {
                    ControlFlow::Continue(*acc.start()..=*r.end().max(acc.end()))
                } else {
                    // println!("{:?} - {:?} - {:?}", &y, &acc, &r);
                    ControlFlow::Break((acc.end() + 1)..=(r.start() - 1))
                }
            }) {
                ControlFlow::Continue(_) => None,
                ControlFlow::Break(r) => Some((*r.start(), y)),
            }
        })
        .map(|(x, y)| Point { x, y })
        .last()
        .unwrap();

    let result = 4_000_000 * distress_beacon_location.x as u64 + distress_beacon_location.y as u64;
    Some(result)
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
        assert_eq!(part_two(&input), Some(56000011));
    }
}
