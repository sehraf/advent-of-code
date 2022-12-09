use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, alphanumeric1, anychar},
        streaming::space1,
    },
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair},
};
use std::{collections::HashMap, ops::AddAssign, path::PathBuf};

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, usize> {
    parse(input)
}

#[aoc(day7, part1)]
pub fn part1(directories: &HashMap<String, usize>) -> usize {
    const LIMIT: usize = 100000;

    directories
        .into_iter()
        .filter_map(|(_, size)| if size <= &LIMIT { Some(size) } else { None })
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, usize>) -> usize {
    const FS_SIZE: usize = 70_000_000;
    const FS_REQUIRED: usize = 30_000_000;

    let occupied = input.get("/").unwrap();
    let free = FS_SIZE - occupied;
    let to_be_freed = FS_REQUIRED - free;

    input
        .iter()
        .filter_map(
            |(_, &size)| {
                if size > to_be_freed {
                    Some(size)
                } else {
                    None
                }
            },
        )
        .min()
        .unwrap()
}

fn parse_ls_file(line: &str) -> nom::IResult<&str, (String, usize)> {
    map(
        separated_pair(alphanumeric1::<&str, _>, space1, many1(anychar)),
        |(size, name)| {
            (
                String::from_iter(name.into_iter()),
                size.parse().expect("failed to parse file size"),
            )
        },
    )(line)
}
// fn parse_ls_dir(line: &str) -> nom::IResult<&str, &str> {
//     preceded(tag("dir "), alpha1::<&str, _>)(line)
// }

fn parse_command(line: &str) -> nom::IResult<&str, Cmd> {
    preceded(
        tag("$ "),
        alt((
            map(tag("ls"), |_| Cmd::Ls),
            map(
                preceded(tag("cd "), alt((tag(".."), tag("/"), alpha1))),
                |dir: &str| Cmd::Cd(dir.to_owned()),
            ),
        )),
    )(line)
}

fn parse(input: &str) -> HashMap<String, usize> {
    let mut iter = input.lines();

    // start with "cd /" command
    let (rem, cmd) = parse_command(iter.next().unwrap()).unwrap();
    assert!(rem.is_empty());
    assert!(matches!(cmd, Cmd::Cd(dir) if &dir == "/"));

    // Structure:
    // First, go over all file information and `cd` commands.
    // The `cd` commands update `current_dir` and any file command
    // adds an entry to `files` (which is a list of <path>, <name>, <size> tuples)
    //
    // Second, convert the "all files" list to a dictionary that maps each
    // folder to its size (accumulative)

    let mut current_dir = PathBuf::from("/");
    let mut files = vec![];

    while let Some(line) = iter.next() {
        match line {
            cmd if cmd.starts_with("$") => {
                let (rem, cmd) = parse_command(cmd).unwrap();
                assert!(rem.is_empty());
                match cmd {
                    Cmd::Cd(path) => match path.as_ref() {
                        ".." => _ = current_dir.pop(),
                        "/" => unreachable!(), // just in case
                        path => current_dir.push(path),
                    },
                    Cmd::Ls => {}
                }
            }
            dir if dir.starts_with("dir") => {}
            file => {
                let (rem, (name, size)) = parse_ls_file(file).unwrap();
                assert!(rem.is_empty());
                files.push((current_dir.to_str().unwrap().to_owned(), name, size));
            }
        }
    }

    // collect sizes
    let mut directories: HashMap<String, usize> = HashMap::new();
    for (path, _, size) in files.into_iter() {
        for path in PathBuf::from(path).ancestors() {
            directories
                .entry(path.to_str().unwrap().to_string())
                .or_default()
                .add_assign(size);
        }
    }
    directories
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 95437);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 24933642);
    }
}
