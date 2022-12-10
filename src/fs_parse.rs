use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<LsEntry<'a>>),
}

#[derive(Debug)]
pub enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

#[derive(Debug)]
pub enum LsEntry<'a> {
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

pub fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, operations) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, operations))
}
