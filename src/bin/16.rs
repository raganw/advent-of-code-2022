#![allow(unused_variables,dead_code)]
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

use std::collections::BTreeMap;

use nom::{
    IResult,
    branch::alt,
    multi::separated_list1,
    character::complete::{alpha1, newline, u32 as nom_u32},
    sequence::preceded,
    bytes::complete::tag, Parser,
};

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels_to: Vec<&'a str>,
}

fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), alpha1)(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), nom_u32)(input)?;
    let (input, tunnels_to) = alt((
        preceded(tag("; tunnel leads to valve "), alpha1.map(|v| vec![v])),
        preceded(tag("; tunnels lead to valves "), separated_list1(tag(", "), alpha1))
    ))(input)?;

    Ok((input, Valve {
        name,
        flow_rate,
        tunnels_to,
    }))
}

fn parse_input(input: &str) -> IResult<&str, BTreeMap<&str, Valve>> {
    let (input, valves) = separated_list1(newline, parse_line)(input)?;
    let mut valve_map: BTreeMap<&str, Valve> = BTreeMap::new();
    for valve in valves {
        valve_map.insert(valve.name, valve);
    }

    Ok((input, valve_map))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, valves) = parse_input(input).unwrap();
    let mut flow_stack: Vec<_> = valves.iter().map(|(_, v)| (v.flow_rate * 30, v)).collect();
    flow_stack.sort_by(|(a, _), (b, _)| b.cmp(a));
    dbg!(&flow_stack);
    // dbg!(&valves);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
