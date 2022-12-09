use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty());
    ret
}

#[aoc(day8, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    // should be a square
    assert_eq!(input.len(), input[0].len());
    let size = input.len();

    let mut visible: HashMap<(usize, usize), bool> = HashMap::new();

    // add edges
    for i in 0..size {
        *visible.entry((i, 0)).or_default() |= true;
        *visible.entry((0, i)).or_default() |= true;
        *visible.entry((i, size - 1)).or_default() |= true;
        *visible.entry((size - 1, i)).or_default() |= true;
    }

    // X
    for x in 0..size {
        // skip edges
        if x == 0 || x == size - 1 {
            continue;
        }

        let res = do_comparison(input[x].iter());
        for (y, v) in res.into_iter().enumerate() {
            *visible.entry((x, y)).or_default() |= v;
        }
    }
    for x in 0..size {
        // skip edges
        if x == 0 || x == size - 1 {
            continue;
        }

        let res = do_comparison(input[x].iter().rev());
        for (y, v) in res.into_iter().rev().enumerate() {
            *visible.entry((x, y)).or_default() |= v;
        }
    }

    // Y
    for y in 0..size {
        // skip edges
        if y == 0 || y == size - 1 {
            continue;
        }

        let res = do_comparison(input.iter().map(|row| &row[y]));
        for (x, v) in res.into_iter().enumerate() {
            *visible.entry((x, y)).or_default() |= v;
        }
    }
    for y in 0..size {
        // skip edges
        if y == 0 || y == size - 1 {
            continue;
        }

        let res = do_comparison(input.iter().map(|row| &row[y]).rev());
        for (x, v) in res.into_iter().rev().enumerate() {
            *visible.entry((x, y)).or_default() |= v;
        }
    }

    // pretty_print(&visible, size);

    visible.iter().filter(|&(_, val)| *val).count()
}

#[allow(dead_code)]
fn pretty_print(map: &HashMap<(usize, usize), bool>, size: usize) {
    for y in 0..size {
        for x in 0..size {
            match map.get(&(x, y)).unwrap() {
                true => print!("#"),
                false => print!("."),
            }
        }
        print!("\n");
    }
}

fn do_comparison<'a>(trees: impl Iterator<Item = &'a u32>) -> Vec<bool> {
    let mut max_hight = -1;
    let mut visible = vec![];

    for tree in trees {
        // initialize max_hight with first value
        if max_hight == -1 {
            max_hight = *tree as i32;

            // this tree is always visible
            visible.push(true);
            continue;
        }

        if *tree as i32 > max_hight {
            max_hight = *tree as i32;
            visible.push(true);
        } else {
            visible.push(false);
        }
    }

    visible
}

#[aoc(day8, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    let size = input.len();

    let mut max_score = 0;

    for y in 0..size {
        for x in 0..size {
            let s = calc_score(input, x, y);
            max_score = max_score.max(s);
            // print!("{s}");
        }
        // println!("");
    }
    max_score
}

fn calc_score(map: &[Vec<u32>], start_x: usize, start_y: usize) -> usize {
    let size = map.len();

    let mut l = 0;
    let mut r = 0;
    let mut u = 0;
    let mut d = 0;

    let base = map[start_x][start_y];

    // left
    for (count, x) in (0..start_x).rev().enumerate() {
        if map[x][start_y] >= base || x == 0 {
            l = count + 1;
            break;
        }
    }

    // right
    for (count, x) in (start_x + 1..size).enumerate() {
        if map[x][start_y] >= base || x == size - 1 {
            r = count + 1;
            break;
        }
    }

    // up
    for (count, y) in (0..start_y).rev().enumerate() {
        if map[start_x][y] >= base || y == 0 {
            u = count + 1;
            break;
        }
    }

    // down
    for (count, y) in (start_y + 1..size).enumerate() {
        if map[start_x][y] >= base || y == size - 1 {
            d = count + 1;
            break;
        }
    }

    l * r * u * d
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        map(digit1, |a: &str| {
            a.chars().map(|c| c.to_digit(10).unwrap()).collect()
        }),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 8);
    }
}
