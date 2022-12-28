use itertools::Itertools;
use nom::{*, multi::many1, branch::alt, bytes::complete::tag};

#[cfg(test)]
use rstest_reuse;

use std::{str::FromStr, iter::Sum, fmt::Display, fs::write};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Snafu(i64);

fn snafu_digit(input: &str) -> IResult<&str, i8> {
    alt((
        tag("0").map(|_| 0_i8),
        tag("1").map(|_| 1_i8),
        tag("2").map(|_| 2_i8),
        tag("-").map(|_| -1_i8),
        tag("=").map(|_| -2_i8),
    ))(input)
}

impl FromStr for Snafu {
    type Err = core::fmt::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, digits) = many1(snafu_digit)(input).unwrap();
        let val = digits.iter().rev().enumerate().fold(0_i64, |acc, (i, v)| {
            acc + (*v as i64 * 5_i64.pow(i as u32))
        });

        Ok(Snafu(val))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars: Vec<_> = itertools::unfold(self.0, |val| {
            let mut rem = *val % 5;
            *val = *val / 5;
            if *val == 0  && rem == 0 {
                return None;
            }
            if rem > 2 {
                rem -= 5;
                *val += 1;
            }
            let c = match rem {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => panic!("uh oh")
            };

            Some(c)
        }).collect();
        let str: String = chars.iter().rev().collect();
        write!(f, "{}", str)?;
        Ok(())
    }
}

impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> Self {
        snafu.0
    }
}

impl <'a> Sum<&'a Snafu> for i64 {
    fn sum<I: Iterator<Item = &'a Snafu>>(iter: I) -> Self {
        iter.fold(0, |acc, s| acc + s.0)
    }
}

impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        iter.fold(Snafu(0), |Snafu(acc), s| Snafu(acc + s.0))
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let sum: Snafu = input.lines().map(|v| v.parse().unwrap()).sum();
    Some(sum.to_string())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[template]
    #[rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("1=", 3)]
    #[case("1-", 4)]
    #[case("10", 5)]
    #[case("11", 6)]
    #[case("12", 7)]
    #[case("2=", 8)]
    #[case("2-", 9)]
    #[case("20", 10)]
    #[case("1=0", 15)]
    #[case("1-0", 20)]
    #[case("1=11-2", 2022)]
    #[case("1-0---0", 12345)]
    #[case("1121-1110-1=0", 314159265)]
    #[case("1=-0-2", 1747)]
    #[case("12111", 906)]
    #[case("2=0=", 198)]
    #[case("21", 11)]
    #[case("2=01", 201)]
    #[case("111", 31)]
    #[case("20012", 1257)]
    #[case("112", 32)]
    #[case("1=-1=", 353)]
    #[case("1-12", 107)]
    #[case("12", 7)]
    #[case("1=", 3)]
    #[case("122", 37)]
    fn test_input(#[case] input: &str, #[case] expected: i64) {}

    #[apply(test_input)]
    fn test_string_to_snafu(#[case] input: &str, #[case] expected: i64) {
        let snafu: Snafu = input.parse().unwrap();
        let value: i64 = snafu.into();
        assert_eq!(value, expected);
    }

    #[apply(test_input)]
    fn test_snafu_to_string(#[case] expected: &str, #[case] input: i64) {
        let value: Snafu = Snafu(input);
        assert_eq!(value.to_string(), expected.to_owned());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
