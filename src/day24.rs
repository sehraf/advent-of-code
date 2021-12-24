use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::PathBuf,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, one_of, space1},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair},
    Finish,
};

use crate::AdventOfCode;

const DAY: &str = "day24";

#[derive(Debug, Clone, Copy)]

enum Register {
    W,
    X,
    Y,
    Z,
    Literal(i64),
}

impl Register {}

impl TryFrom<char> for Register {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Register::W),
            'x' => Ok(Register::X),
            'y' => Ok(Register::Y),
            'z' => Ok(Register::Z),
            _ => unreachable!("invalid register"),
        }
    }
}

#[derive(Debug, Clone, Copy)]

enum OpCode {
    Inp(Register),
    Add(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    Mod(Register, Register),
    Eql(Register, Register),
}

impl OpCode {
    fn try_from(code: &str, r1: Register, r2: Register) -> Result<Self, ()> {
        match code {
            "add" => Ok(OpCode::Add(r1, r2)),
            "mul" => Ok(OpCode::Mul(r1, r2)),
            "div" => Ok(OpCode::Div(r1, r2)),
            "mod" => Ok(OpCode::Mod(r1, r2)),
            "eql" => Ok(OpCode::Eql(r1, r2)),

            _ => Err(()),
        }
    }
}

struct State {
    regs: [i64; 4],
}

impl State {
    fn process(&mut self, op: &OpCode) {
        match op {
            OpCode::Add(a, b) => *self.reg_mut(a) = self.reg(a) + self.reg(b),
            OpCode::Mul(a, b) => *self.reg_mut(a) = self.reg(a) * self.reg(b),
            OpCode::Div(a, b) => *self.reg_mut(a) = self.reg(a) / self.reg(b),
            OpCode::Mod(a, b) => *self.reg_mut(a) = self.reg(a) % self.reg(b),
            OpCode::Eql(a, b) => *self.reg_mut(a) = (self.reg(a) == self.reg(b)) as i64,

            // OpCode::Inp(a) => *self.get_reg_mut(&a) = self.input.pop_front().unwrap() as i64, // self.input is no more
            OpCode::Inp(_) => unreachable!(),
        }
    }

    fn reg<'a>(&'a self, reg: &'a Register) -> &'a i64 {
        match reg {
            Register::Literal(a) => a,
            Register::W => &self.regs[0],
            Register::X => &self.regs[1],
            Register::Y => &self.regs[2],
            Register::Z => &self.regs[3],
        }
    }

    fn reg_mut<'a>(&'a mut self, reg: &'a Register) -> &'a mut i64 {
        match reg {
            Register::Literal(_) => unreachable!(),
            Register::W => &mut self.regs[0],
            Register::X => &mut self.regs[1],
            Register::Y => &mut self.regs[2],
            Register::Z => &mut self.regs[3],
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "regs:")?;
        for reg in self.regs {
            write!(f, " {}", reg)?;
        }
        writeln!(f, "")
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<Vec<OpCode>>,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt");
        let a = self.puzzle1();

        // self.load(base_dir, String::from(DAY) + ".txt");
        let b = self.puzzle2();

        (a, b)
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: String) {
        let input_file = base_dir.join(test_input);
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let lines = input.lines();
        let insts: Vec<OpCode> = lines
            .map(|line| parse(line).finish().map(|(_, x)| x).unwrap())
            .collect();
        // split on each `inp` instruction and remove it
        let blocks = insts
            .chunks(18)
            .map(|c| c.iter().skip(1).copied().collect())
            .collect::<Vec<_>>();
        self.input = blocks;
    }

    fn puzzle1(&mut self) -> u64 {
        self.find(&[9, 8, 7, 6, 5, 4, 3, 2, 1]) as u64
    }

    fn puzzle2(&mut self) -> u64 {
        self.find(&[1, 2, 3, 4, 5, 6, 7, 8, 9]) as u64
    }

    fn find(&self, range: &[i64; 9]) -> i64 {
        // (block, z) -> num
        let mut cache: HashMap<(usize, i64), Option<i64>> = HashMap::new();
        let res = find_recursive(&mut cache, &self.input, 0, 0, range).unwrap();
        res.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

fn find_recursive(
    cache: &mut HashMap<(usize, i64), Option<i64>>,
    blocks: &Vec<Vec<OpCode>>,
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    if let Some(&res) = cache.get(&(block, z)) {
        return res;
    }

    for &num in range {
        // execute step
        let mut state = State {
            regs: [num, 0, 0, z],
        };
        for op in &blocks[block] {
            state.process(op);
        }
        let z = state.reg(&Register::Z).to_owned();

        // are we there yet?
        if block == blocks.len() - 1 {
            if z == 0 {
                cache.insert((block, z), Some(num));
                return Some(num);
            }
            continue;
        }

        // we need to go deeper!
        if let Some(res) = find_recursive(cache, blocks, block + 1, z, range) {
            cache.insert((block, z), Some(res * 10 + num));
            return Some(res * 10 + num);
        }
    }

    // feels bad man
    cache.insert((block, z), None);
    None
}

fn parse_register(line: &str) -> nom::IResult<&str, Register> {
    map_res(one_of("wxyz"), |op| Register::try_from(op))(line)
}

fn parse_literal(line: &str) -> nom::IResult<&str, Register> {
    map(i64, |num| Register::Literal(num))(line)
}

fn parse_reg_or_lit(line: &str) -> nom::IResult<&str, Register> {
    alt((parse_register, parse_literal))(line)
}

fn parse_operants(line: &str) -> nom::IResult<&str, (Register, Register)> {
    separated_pair(parse_register, space1, parse_reg_or_lit)(line)
}

fn parse_op(line: &str) -> nom::IResult<&str, OpCode> {
    map_res(
        separated_pair(nom::character::complete::alpha1, space1, parse_operants),
        |(code, (r1, r2))| OpCode::try_from(code, r1, r2),
    )(line)
}

fn parse(line: &str) -> nom::IResult<&str, OpCode> {
    alt((
        map(preceded(tag("inp "), parse_register), |reg| {
            OpCode::Inp(reg)
        }),
        parse_op,
    ))(line)
}

#[cfg(test)]
mod day1 {
    use std::env;
    use std::path::PathBuf;

    use super::{Data, State, DAY};

    #[test]
    fn puzzle1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");

        let mut state = State { regs: [7, 0, 0, 0] };
        for op in &data.input[0] {
            state.process(op);
        }
        assert_eq!(state.regs, [0, 1, 1, 1]);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");

        let mut state = State { regs: [8, 0, 0, 0] };
        for op in &data.input[0] {
            state.process(op);
        }
        assert_eq!(state.regs, [1, 0, 0, 0]);    }
}
