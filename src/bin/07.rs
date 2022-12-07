use std::str::FromStr;

use nom::{IResult, character::complete::newline, bytes::complete::tag, multi::{separated_list1, separated_list0}};

#[derive(Debug, Clone, PartialEq, Eq)]
enum FsNode {
    File {
        name: String,
        filesize: u32
    },
    Dir {
        name: String,
        nodes: Vec<FsNode>,
    }
}

impl FromStr for FsNode {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, last) = s.split_once(' ').unwrap();
        if first == "dir" {
            Ok(FsNode::Dir { name: last.to_owned(), nodes: vec![] })
        } else {
            let filesize: u32 = first.parse().unwrap();
            Ok(FsNode::File { name: last.to_owned(), filesize })
        }
    }
}

impl FsNode {
    fn size(&self) -> u32 {
        match self {
            FsNode::File { filesize, name: _ } => *filesize,
            FsNode::Dir { name: _, nodes } => {
                nodes.iter().map(|node| {
                    match node {
                        FsNode::File { name: _, filesize } => *filesize,
                        FsNode::Dir { name: _, nodes: _ } => node.size(),
                    }
                }).sum::<u32>()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    ChangeDir(String),
    ChangeRootDir,
    UpDir,
    Ls,
}

impl FromStr for Command {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ls" {
            Ok(Command::Ls)
        }  else {
            let (_, cd_operation) = s.split_once(' ').unwrap();
            match cd_operation {
                "/" => Ok(Command::ChangeRootDir),
                ".." => Ok(Command::UpDir),
                directory_name => Ok(Command::ChangeDir(directory_name.to_owned())),
            }
        }
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    // let (input, _) = tag("$ ")(input)?;
    // let (input, command) = 
    Ok((input, Command::Ls))
}

fn parse_line(input: &str) -> IResult<&str, &str> {
    dbg!(&input);
    Ok(("", "hello"))
}

pub fn read_puzzle_input(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, lines) = separated_list0(newline, parse_line)(input)?;
    Ok((input, lines))
}

pub fn part_one(input: &str) -> Option<u32> {
    // let (_, lines) = 
    let (_, lines) = read_puzzle_input(input).unwrap();
    dbg!(lines);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_root_dir() {
        let input = String::from("$ cd /");
        // assert_eq!(parse_command(input), Command::ChangeRootDir);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
