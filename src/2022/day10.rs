use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::preceded,
};

pub enum Instruction {
    Noop,
    Addx(i32),
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let (rem, instr) = parse(input).unwrap();
    assert!(rem.is_empty());
    instr
}

#[aoc(day10, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut reg = 1;
    let mut cycle = 0;
    let mut counter = 0;

    let check_reg = |cycle: &i32, reg: &i32, counter: &mut i32| match cycle {
        c if (c - 20) % 40 == 0 => {
            dbg!(c, reg, reg * c);
            *counter += c * reg
        }
        _ => {}
    };

    for inst in input {
        match inst {
            Instruction::Noop => {
                cycle += 1;
                check_reg(&cycle, &reg, &mut counter);
            }
            Instruction::Addx(i) => {
                cycle += 1;
                check_reg(&cycle, &reg, &mut counter);

                cycle += 1;
                check_reg(&cycle, &reg, &mut counter);

                reg += i;
            }
        }
    }

    counter
}

#[aoc(day10, part2)]
pub fn part2(input: &[Instruction]) -> u32 {
    let mut reg = 1;
    let mut cycle = 0;

    let render = |cycle: &i32, reg: &i32| {
        let x = (cycle-1) % 40;
        let visible = match x - reg {
            -1|0|1 => true,
            _ => false,
        };

        match visible {
            true => print!("#"),
            false => print!(" ")
        }

        if x == 39 {
            println!("")
        }
    };

    for inst in input {
        match inst {
            Instruction::Noop => {
                cycle += 1;
                render(&cycle, &reg);
            }
            Instruction::Addx(i) => {
                cycle += 1;
                render(&cycle, &reg);

                cycle += 1;
                render(&cycle, &reg);

                reg += i;
            }
        }
    }

    0
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(
        line_ending,
        alt((
            map(tag("noop"), |_| Instruction::Noop),
            map(preceded(tag("addx "), many1(one_of("0123456789-"))), |c| {
                let i = String::from_iter(c.into_iter()).parse().unwrap();
                Instruction::Addx(i)
            }),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 13140);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 0);
    }
}
