use aoc_runner_derive::{aoc, aoc_generator};

type T = Challenge;

#[derive(Debug, Default)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, Default)]
struct Number {
    val: u32,
    pos_begin: u32,
    pos_end: Pos,
}

#[derive(Debug, Default)]
struct Symbol {
    val: char,
    pos: Pos,
}
#[derive(Debug, Default)]
pub struct Challenge {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> T {
    let mut challenge = Challenge::default();

    let mut x;
    let mut y = 0;
    let mut num_buf = String::new();

    // helper function
    let convert_number = |challenge: &mut Challenge, num_buf: &mut String, x, y| {
        assert!(num_buf.len() <= 3);
        challenge.numbers.push(Number {
            val: num_buf.parse().unwrap(),
            // x is right after the number
            pos_begin: x - num_buf.len() as u32,
            pos_end: Pos { x: x - 1, y },
        });
        num_buf.clear();
    };

    for line in input.lines() {
        x = 0;
        num_buf.clear();

        for char in line.chars() {
            match char {
                '0'..='9' => {
                    // add char to buffer
                    num_buf.push(char);
                }
                _ => {
                    // can be a symbol or a dot
                    if !num_buf.is_empty() {
                        convert_number(&mut challenge, &mut num_buf, x, y);
                    }
                    if char != '.' {
                        challenge.symbols.push(Symbol {
                            val: char,
                            pos: Pos { x, y },
                        });
                    }
                }
            }

            x += 1;
        }

        if !num_buf.is_empty() {
            convert_number(&mut challenge, &mut num_buf, x, y);
        }

        y += 1;
    }

    // dbg!(&challenge);

    challenge
}

#[aoc(day3, part1)]
pub fn part1(input: &T) -> u32 {
    input
        .numbers
        .iter()
        .filter(|&number| {
            let x_range = number.pos_begin as i32 - 1..=number.pos_end.x as i32 + 1;
            let y_range = number.pos_end.y as i32 - 1..=number.pos_end.y as i32 + 1;
            input
                .symbols
                .iter()
                .find(|&symbol| {
                    x_range.contains(&(symbol.pos.x as i32))
                        && y_range.contains(&(symbol.pos.y as i32))
                })
                .is_some()
        })
        .map(|number| number.val)
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &T) -> u32 {
    // this is exactly like part1 ... just different!
    input
        .symbols
        .iter()
        .filter(|symbol| symbol.val == '*') // this is not necessary for my input?!
        .filter_map(|symbol| {
            let x_range = symbol.pos.x as i32 - 1..=symbol.pos.x as i32 + 1;
            let y_range = symbol.pos.y as i32 - 1..=symbol.pos.y as i32 + 1;
            let adjacent = input.numbers.iter().filter(|&number| {
                y_range.contains(&(number.pos_end.y as i32))
                    && (x_range.contains(&(number.pos_begin as i32))
                        || x_range.contains(&(number.pos_end.x as i32)))
            });

            let res = adjacent.fold((1, 0), |(acc, r), e| (acc * e.val, r + 1));

            if res.1 == 2 {
                Some(res.0)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 4361);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 467835);
    }
}
