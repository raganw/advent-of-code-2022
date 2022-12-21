use std::collections::{HashSet, VecDeque};

use nom::{
    *,
    bytes::complete::tag,
    IResult, multi::separated_list1, character::complete::{i32 as nom_i32, newline}};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point3(i32, i32, i32);

impl Point3 {
    fn neighbors(&self) -> Vec<Self> {
        let directions = vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
        directions.iter().map(|&d| {
            let (x2, y2, z2) = d;
            Self(self.0 + x2, self.1 + y2, self.2 + z2)
        }).collect()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Point3>> {
    separated_list1(newline, separated_list1(tag(","), nom_i32).map(|v| {
        let (x, y, z) = if let [x, y, z] = v[0..3] { (x, y, z) } else { unreachable!() };
        Point3(x,y,z)
    }))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();
    let all_points: HashSet<_> = input.iter().collect();
    let result: usize = input.iter().map(|p| {
        p.neighbors().iter().filter(|&n| all_points.get(n).is_none()).count()
    }).sum();
    Some(result as u32)
}

fn bounding_box(points: &Vec<Point3>) -> (Point3, Point3) {
    // create bounding box
    let (mut x_min, mut x_max) = (i32::MAX, i32::MIN);
    let (mut y_min, mut y_max) = (i32::MAX, i32::MIN);
    let (mut z_min, mut z_max) = (i32::MAX, i32::MIN);
    for p in points {
        let (x, y, z) = (p.0, p.1, p.2);
        (x_min, x_max) = (x.min(x_min), x.max(x_max));
        (y_min, y_max) = (y.min(y_min), y.max(y_max));
        (z_min, z_max) = (z.min(z_min), z.max(z_max));
    }
    (x_min, x_max) = (x_min - 1, x_max + 1);
    (y_min, y_max) = (y_min - 1, y_max + 1);
    (z_min, z_max) = (z_min - 1, z_max + 1);
    (Point3(x_min, y_min, z_min), Point3(x_max, y_max, z_max))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();
    let all_points: HashSet<_> = input.iter().collect();

    let (min_point, max_point) = bounding_box(&input);
    // bfs starting in corner of bounding box, not traversing points in all_points
    let mut seen: HashSet<Point3> = HashSet::new();
    let mut to_visit: VecDeque<Point3> = vec![min_point].into();
    let mut surface_area: u32 = 0;

    while let Some(point) = to_visit.pop_front() {
        for neighbor in point.neighbors() {
            let (x, y, z) = (neighbor.0, neighbor.1, neighbor.2);

            // remove out of bounds
            if x < min_point.0 || x > max_point.0 ||
               y < min_point.1 || y > max_point.1 ||
               z < min_point.2 || z > max_point.2 {
                continue;
            }

            // if lava, increment count, 
            if all_points.get(&neighbor).is_some() {
                surface_area += 1;
            } else if seen.get(&neighbor).is_some() {
                continue;
            } else {
                seen.insert(neighbor);
                to_visit.push_back(neighbor);
            }
        }
    }
    Some(surface_area)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
