use std::{collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    dirs: HashMap<&'a str, Dir<'a>>,
}

impl <'a> Dir<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            files: Vec::<File<'a>>::new(),
            dirs: HashMap::new(),
        }
    }

    fn size(&self) -> u32 {
        let total_files = self.files.iter().map(|f| f.filesize).sum::<u32>();
        let total_dirs = self.dirs.iter().map(|(_, d)| d.size()).sum::<u32>();
        total_files + total_dirs
    }
    
    fn set_content(&mut self, files: Vec::<File<'a>>, dirs: HashMap<&str, Dir<'a>>) {
        for file in files {
            self.files.push(File::new(file.name, file.filesize));
        }
        for (name, dir) in dirs {
            self.dirs.insert(dir.name, Dir::new(dir.name));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File<'a> {
    name: &'a str,
    filesize: u32
}

impl <'a> File <'a> {
    fn new(name: &'a str, filesize: u32) -> Self {
        Self {
            name,
            filesize
        }
    }
}

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<LsEntry<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

#[derive(Debug)]
enum LsEntry<'a> {
    File { name: &'a str, size: u32 },
    Dir(&'a str),
}

fn dir(input: &str) -> IResult<&str, LsEntry> {
    let (input, dir_name) = preceded(tag("dir "), alpha1)(input)?;
    Ok((input, LsEntry::Dir(dir_name)))
}

fn filename(input: &str) -> IResult<&str, &str> {
    is_a("qwertyuiopasdfghjklzxcvbnm.")(input)
}

fn file(input: &str) -> IResult<&str, LsEntry> {
    let (input, (size, name)) =
        separated_pair(nom::character::complete::u32, tag(" "), filename)(input)?;

    Ok((input, LsEntry::File { name, size }))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, ls_entries) = separated_list1(newline, alt((dir, file)))(input)?;
    dbg!(&ls_entries);

    Ok((input, Operation::Ls(ls_entries)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, cd) = alt((tag(".."), tag("/"), alpha1))(input)?;
    let op = match cd {
        ".." => Operation::Cd(Cd::Up),
        "/" => Operation::Cd(Cd::Root),
        dir => Operation::Cd(Cd::Down(dir)),
    };
    Ok((input, op))
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, operations) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, operations))
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    dbg!(&lines);
    let operations = parse_operations(input).unwrap().1;
    let mut fs_nodes: HashMap<String, Vec<LsEntry>> = HashMap::new();
    let mut cwd: Vec<&str> = vec![];

    let root = Dir::new("/");
    let mut dir_stack: Vec<&Dir> = vec![];
    dir_stack.push(&root);

    for op in operations {
        // let current_dir: &Dir<'_> = dir_stack.last().unwrap();
        dbg!(&op);
        match op {
            Operation::Cd(cd_op) => {
                match cd_op {
                    Cd::Root => {
                        cwd.push("/");
                    },
                    Cd::Down(dirname) => {
                        cwd.push(dirname);
                        // let new_dir = current_dir.dirs.get(dirname).unwrap();
                        // dir_stack.push(new_dir);
                    },
                    Cd::Up => {
                        cwd.pop();
                        // dir_stack.pop();
                    }
                }
            },
            Operation::Ls(ls_entries) => {
                let cwd = cwd.join("/");
                let mut files: Vec<File> = vec![];
                let mut dirs: HashMap<&str, Dir> = HashMap::new();
                for entry in ls_entries.iter() {
                    match entry {
                        LsEntry::Dir(dirname) => {
                            dirs.insert(dirname, Dir::new(dirname));
                        }
                        LsEntry::File { name, size } => {
                            files.push(File::new(name, *size));
                        }
                    }
                }
                // current_dir.set_content(files, dirs);
                fs_nodes.insert(cwd, ls_entries);
            }
        }
    }

    dbg!(fs_nodes);

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
