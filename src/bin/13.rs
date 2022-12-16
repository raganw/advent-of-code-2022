use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
enum PacketElement {
    Value(u32),
    List(Vec<PacketElement>),
}

impl PartialEq for PacketElement {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketElement { }

impl PartialOrd for PacketElement { 
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketElement::Value(left_value), PacketElement::Value(right_value)) => left_value.cmp(right_value),
            (PacketElement::Value(left_value), PacketElement::List(_)) => PacketElement::List(vec![PacketElement::Value(*left_value)]).cmp(other),
            (PacketElement::List(_), PacketElement::Value(right_value)) => self.cmp(&PacketElement::List(vec![PacketElement::Value(*right_value)])),
            (PacketElement::List(left_list), PacketElement::List(right_list)) => {
                for (a, b) in left_list.iter().zip(right_list) {
                    match a.cmp(b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }
                left_list.len().cmp(&right_list.len())
            }
        }
    }
}

fn parse_value(input: &str) -> IResult<&str, PacketElement> {
    let (input, value) = nom::character::complete::u32(input)?;

    Ok((input, PacketElement::Value(value)))
}

fn parse_list(input: &str) -> IResult<&str, PacketElement> {
    let (input, value) = delimited(tag("["), separated_list0(tag(","), parse_packet_element), tag("]"))(input)?;

    Ok((input, PacketElement::List(value)))
}

fn parse_packet_element(input: &str) -> IResult<&str, PacketElement> {
    alt((parse_value, parse_list))(input)
}

fn parse_packet(input: &str) -> IResult<&str, (PacketElement, PacketElement)> {
    separated_pair(parse_packet_element, newline, parse_packet_element)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(PacketElement, PacketElement)>> {
     separated_list1(tag("\n\n"), parse_packet)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, packets) = parse_input(input).unwrap();
    let mut valid_packets: Vec<u32> = vec![];
    for (idx, (left, right)) in packets.iter().enumerate() {
        let idx = idx as u32 + 1;
        if left.cmp(right) == Ordering::Less {
            valid_packets.push(idx);
        }
    }
    let result: u32 = valid_packets.iter().sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, packets) = parse_input(input).unwrap();
    let mut packets: Vec<_> = packets.into_iter().flat_map(|(a, b)| vec![a, b]).collect();
    let packet_1 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Value(2)])]);
    let packet_2 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Value(6)])]);
    packets.push(packet_1.clone());
    packets.push(packet_2.clone());
    packets.sort();
    let result = packets.iter().enumerate().filter_map(|(i, p)| {
        if *p == packet_1 || *p == packet_2 {
            Some(i as u32 + 1)
        } else {
            None
        }
    }).product();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
