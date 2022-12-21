#![allow(unused_variables, dead_code)]
use std::{fmt::Display, iter::Cycle};
use std::ops::Deref;

use nom::{
    branch::alt,
    multi::many1,
    character::complete::char as nom_char,
    IResult,
    *
};

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Jet>> {
    many1(alt((
        nom_char('<').map(|_| Jet::Left),
        nom_char('>').map(|_| Jet::Right),
    )))(input)
}

const _ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

#[derive(Debug, Clone, Copy)]
struct RockFormation([u8;4]);

impl RockFormation {
    const FORMATIONS: [[u8;4];5] = [
        [
            0b00000000,
            0b00000000,
            0b00000000,
            0b00111100
        ],
        [
            0b00000000,
            0b00010000,
            0b00111000,
            0b00010000
        ],
        [
            0b00000000,
            0b00001000,
            0b00001000,
            0b00111000,

        ],
        [
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000
        ],
        [
            0b00000000,
            0b00000000,
            0b00110000,
            0b00110000
        ]
    ];
}

fn write_row(row: &u8, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut bitmask = 0b10000000;
    for i in 0..7 {
        if bitmask & row == bitmask {
            write!(f, "#")?;
        } else {
            write!(f, ".")?;
        }
        bitmask >>= 1;
    }
    Ok(())
}

impl Display for RockFormation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            write_row(&row, f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Chamber {
    rocks: Vec<u8>,
    jets: Vec<Jet>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rocks.iter().rev() {
            write!(f, "|")?;
            write_row(&row, f)?;
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}

impl Chamber {
    fn new(jets: Vec<Jet>) -> Self {
        Self {
            rocks: vec![],
            jets, 
        }
    }

    fn insert_formation(&mut self, formation: RockFormation) {
        let height = self.rock_height();
        let insert_height = height + 3;
        let insert_rows = insert_height.checked_sub(self.rocks.len()).unwrap_or(0) + 4;
        for _ in 0..insert_rows {
            self.rocks.push(0);
        }
    }

    fn rock_height(&self) -> usize {
        self.rocks.iter().enumerate().rev().find(|(_, &row)| row != 0).map(|(v, _)| v).unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, jets) = parse_input(input).unwrap();
    let mut chamber = Chamber::new(jets);

    for formation in RockFormation::FORMATIONS {
        let formation = RockFormation(formation);
        chamber.insert_formation(formation);
        println!("{}", formation);
    }
    println!("{}", chamber);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
