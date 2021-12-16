use std::{ops::ShlAssign, path::PathBuf};

use nom::{bits::complete::take, IResult};

use crate::AdventOfCode;

const DAY: &str = "day16";

#[derive(Debug)]
enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Default for Packet {
    fn default() -> Self {
        Packet::Literal(0, 0)
    }
}

impl Packet {
    fn get_sum(&self) -> u64 {
        match self {
            Packet::Literal(ver, _val) => *ver as u64,
            Packet::Operator(ver, _ty, next) => {
                next.iter().fold(*ver as u64, |acc, p| acc + p.get_sum())
            }
        }
    }

    fn process(&self) -> u64 {
        match self {
            &Packet::Literal(_ver, val) => val,
            Packet::Operator(_ver, ty, next) => {
                match ty {
                    0 => {
                        // sum
                        next.iter().fold(0, |acc, p| acc + p.process())
                    }
                    1 => {
                        // product
                        next.iter().fold(1, |acc, p| acc * p.process())
                    }
                    2 => {
                        // minimun
                        next.iter().map(|p| p.process()).min().unwrap()
                    }
                    3 => {
                        // maximum
                        next.iter().map(|p| p.process()).max().unwrap()
                    }
                    5 => {
                        // greater than
                        assert_eq!(next.len(), 2);
                        if next.first().unwrap().process() > next.last().unwrap().process() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        // less then
                        assert_eq!(next.len(), 2);
                        if next.first().unwrap().process() < next.last().unwrap().process() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        // equal
                        assert_eq!(next.len(), 2);
                        if next.first().unwrap().process() == next.last().unwrap().process() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: Vec<u8>,
    offset: usize,
    packets: Packet,
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) {
        self.load(base_dir, String::from(DAY) + ".txt");
        println!("{}, puzzle 1: {}", DAY, self.puzzle1());

        self.load(base_dir, String::from(DAY) + ".txt");
        println!("{}, puzzle 2: {}", DAY, self.puzzle2());
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: String) {
        let input_file = base_dir.join(test_input);
        assert!(
            input_file.exists(),
            "input file {} does not exist",
            input_file.to_string_lossy()
        );
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let mut lines = input.lines();
        let line = lines.next().unwrap();
        self.input = (0..line.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&line[i..i + 2], 16).unwrap())
            .collect();

        self.parse_packets();
    }

    fn puzzle1(&mut self) -> u64 {
        self.packets.get_sum()
    }

    fn puzzle2(&mut self) -> u64 {
        self.packets.process()
    }

    fn parse_packets(&mut self) {
        let ptr = (self.input.as_slice(), self.offset);
        let (_ptr, p) = parse_packet(ptr).unwrap();
        self.packets = p;

        let copy = ptr.0.to_vec();
        self.offset = ptr.1;
        self.input = copy;
    }
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let mut ptr_g = input;

    // header
    let (ptr, version) = eat_bits(ptr_g, 3).unwrap();
    let (ptr, ty) = eat_bits(ptr, 3).unwrap();

    let p = match ty {
        4 => {
            // literal
            let mut num = 0u64;
            ptr_g = ptr;
            loop {
                let (ptr, last) = eat_bits(ptr_g, 1).unwrap();

                let (ptr, part) = eat_bits(ptr, 4).unwrap();
                num.shl_assign(4);
                num += part as u64;

                ptr_g = ptr;
                if last == 0 {
                    break;
                }
            }
            Packet::Literal(version, num)
        }
        _ => {
            // operator
            let (ptr, i) = eat_bits(ptr, 1).unwrap();
            let mut counter = 0usize;
            ptr_g = ptr;

            if i == 0 {
                // length in bits
                // split 15 bit read in two chunks
                let (ptr, x) = eat_bits(ptr_g, 8).unwrap();
                counter.shl_assign(8);
                counter += x as usize;
                let (ptr, x) = eat_bits(ptr, 7).unwrap();
                counter.shl_assign(7);
                counter += x as usize;
                ptr_g = ptr;
            } else {
                // length in packets
                // split 11 bit read in two chunks
                let (ptr, x) = eat_bits(ptr_g, 8).unwrap();
                counter.shl_assign(8);
                counter += x as usize;
                let (ptr, x) = eat_bits(ptr, 3).unwrap();
                counter.shl_assign(3);
                counter += x as usize;
                ptr_g = ptr;
            }

            let mut packets = vec![];
            while counter > 0 {
                let (ptr, p) = parse_packet(ptr_g).unwrap();
                packets.push(p);

                if i == 0 {
                    // length in bits
                    let consumed = (ptr_g.0.len() - ptr.0.len()) * 8 + ptr.1 - ptr_g.1;
                    counter = counter.saturating_sub(consumed);
                } else {
                    // length in packets
                    counter -= 1;
                }

                ptr_g = ptr;
            }

            Packet::Operator(version, ty, packets)
        }
    };

    Ok((ptr_g, p))
}

fn eat_bits(input: (&[u8], usize), count: usize) -> IResult<(&[u8], usize), u8> {
    assert!(count <= 8);

    take(count)(input)
}

#[cfg(test)]
mod day1 {
    use std::env;
    use std::path::PathBuf;

    use super::{Data, DAY};

    #[test]
    fn puzzle1() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle1(), 20);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt");
        assert_eq!(data.puzzle2(), 1);
    }
}
