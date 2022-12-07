
use std::{str::FromStr, error::Error, fmt::Display, collections::HashMap};
#[derive(Debug)]
struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for ParseError {

}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FsNode {
    File {
        name: String,
        filesize: u32
    },
    Dir {
        name: String,
        nodes: HashMap<&'static String, &'static FsNode>,
    }
}

impl FromStr for FsNode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, last) = s.split_once(' ').unwrap();
        if first == "dir" {
            Ok(FsNode::new_dir(last))
        } else {
            first.parse::<u32>().map(|filesize| {
                FsNode::File { name: last.to_owned(), filesize }
            }).map_err(|_| ParseError)
        }
    }
}

impl FsNode {
    fn new_dir(name: &str) -> Self {
        FsNode::Dir {
            name: name.to_owned(),
            nodes: HashMap::new()
        }
    }

    fn name(&self) -> &String {
        match self {
            FsNode::File { filesize: _, name} => name,
            FsNode::Dir { name, nodes: _ } => name
        }
    }

    fn size(&self) -> u32 {
        match self {
            FsNode::File { filesize, name: _ } => *filesize,
            FsNode::Dir { name: _, nodes } => {
                nodes.iter().map(|(_, node)| {
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
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("$") {
            return Err(ParseError);
        }
        let s = &s[2..];
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

#[derive(Debug)]
enum LineItem {
    Command(Command),
    FsNode(FsNode),
}

impl FromStr for LineItem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maybe_command = s.parse::<Command>().map(LineItem::Command);
        let maybe_fs_node = s.parse::<FsNode>().map(LineItem::FsNode);
        maybe_command.or(maybe_fs_node)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines: Vec<LineItem> = input.lines().map(|line| line.parse::<LineItem>().unwrap()).collect();
    let fs_root = FsNode::new_dir("/");
    let mut fs_stack: Vec<&FsNode> = vec![&fs_root];

    for mut line_item in lines.iter_mut().skip(1) {
        match line_item {
            LineItem::Command(Command::UpDir) => { fs_stack.pop(); },
            LineItem::Command(Command::ChangeDir(new_dir)) => {
                let new_dir = fs_stack.last().map(|dir| {
                    match dir {
                        FsNode::Dir { name: _, nodes } => nodes.get(new_dir).expect("directory not found"),
                        _ => unreachable!("not a directory!?")
                    }
                }).unwrap();
                fs_stack.push(new_dir);
            }
            LineItem::Command(Command::ChangeRootDir) => { fs_stack.truncate(1); }
            LineItem::FsNode(ref node) => {
                let name = node.name();
                let cwd = fs_stack.last_mut().map(|dir| {
                    match dir {
                        FsNode::Dir { name: _, ref mut nodes } => nodes,
                        _ => unreachable!("not a directory!?")
                    }
                }).unwrap();
                cwd.insert(name, node);
            }
            _ => {}
        }
    }

    dbg!(&fs_root);

    // dbg!(&lines);
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
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
