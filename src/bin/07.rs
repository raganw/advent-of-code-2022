use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use advent_of_code::fs_parse::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir<'a> {
    name: &'a str,
    files: RefCell<Vec<Rc<File<'a>>>>,
    dirs: RefCell<HashMap<&'a str, Rc<Dir<'a>>>>,
}

impl<'a> Dir<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            files: RefCell::new(Vec::<Rc<File<'a>>>::new()),
            dirs: RefCell::new(HashMap::new()),
        }
    }

    fn size(&self) -> u32 {
        let total_files = self.files.borrow().iter().map(|f| f.filesize).sum::<u32>();
        let total_dirs = self
            .dirs
            .borrow()
            .iter()
            .map(|(_, d)| d.size())
            .sum::<u32>();
        total_files + total_dirs
    }

    fn sizes(&self) -> Vec<u32> {
        let mut child_sizes: Vec<u32> = self
            .dirs
            .borrow()
            .iter()
            .flat_map(|(_, d)| d.sizes())
            .collect();
        child_sizes.push(self.size());
        child_sizes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File<'a> {
    name: &'a str,
    filesize: u32,
}

impl<'a> File<'a> {
    fn new(name: &'a str, filesize: u32) -> Self {
        Self { name, filesize }
    }
}

fn build_filesystem(operations: Vec<Operation>) -> Rc<Dir> {
    let root = Rc::new(Dir::new("/"));
    let mut dir_stack: Vec<Rc<Dir>> = vec![];

    for op in operations {
        match op {
            Operation::Cd(cd_op) => match cd_op {
                Cd::Root => {
                    dir_stack.push(root.clone());
                }
                Cd::Down(dirname) => {
                    let current_dir = dir_stack.last().unwrap();
                    let new_cwd = current_dir.dirs.borrow().get(dirname).unwrap().clone();
                    dir_stack.push(new_cwd);
                }
                Cd::Up => {
                    dir_stack.pop();
                }
            },
            Operation::Ls(ls_entries) => {
                let current_dir = dir_stack.last().unwrap();
                for entry in ls_entries.iter() {
                    match entry {
                        LsEntry::Dir(dirname) => {
                            current_dir
                                .dirs
                                .borrow_mut()
                                .insert(dirname, Rc::new(Dir::new(dirname)));
                        }
                        LsEntry::File { name, size } => {
                            current_dir
                                .files
                                .borrow_mut()
                                .push(Rc::new(File::new(name, *size)));
                        }
                    }
                }
            }
        }
    }

    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let operations = parse_operations(input).unwrap().1;
    let root = build_filesystem(operations);

    let result: u32 = root.sizes().into_iter().filter(|&v| v < 100_000).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let operations = parse_operations(input).unwrap().1;
    let root = build_filesystem(operations);
    let total_disk_space = 70_000_000;
    let space_needed = 30_000_000;
    let total_used_space = root.size();
    let unused_space = total_disk_space - total_used_space;
    let space_to_free = space_needed - unused_space;
    dbg!(total_used_space, unused_space, space_to_free);
    let sizes = root.sizes();
    let candidates: Vec<&u32> = sizes.iter().filter(|&&s| s > space_to_free).collect();
    dbg!(&candidates);
    let result = candidates.iter().min().unwrap();

    Some(**result)
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
        assert_eq!(part_two(&input), Some(24933642));
    }
}
