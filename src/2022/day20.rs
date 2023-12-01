use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::tuple,
};

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

fn do_round(input: &[i32], key: i64, rounds: i32) -> Vec<i64> {
    // create value index pairs
    let mut results = input
        .iter()
        .map(|&n| n as i64 * key)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..rounds {
        for number_pos_pair in input.iter().map(|&n| n as i64 * key).enumerate() {
            // remove element
            let current_index = results.iter().position(|x| x == &number_pos_pair).unwrap();
            results.remove(current_index);

            // insert element
            let new_index = ((current_index as i64 + number_pos_pair.1)
                .rem_euclid(results.len() as i64)) as usize;
            results.insert(new_index, number_pos_pair);

            // println!("moving {} from {index} to {index_new}", e);
            // println!("{data:?}");
        }
    }
    results.iter().map(|r| r.1).collect::<Vec<_>>()
}

#[aoc(day20, part1)]
pub fn part1(input: &[i32]) -> i64 {
    let v = do_round(input, 1, 1);

    let zero = v.iter().position(|i| *i == 0).unwrap();
    v[(zero + 1000) % v.len()] + v[(zero + 2000) % v.len()] + v[(zero + 3000) % v.len()]
}

#[aoc(day20, part2)]
pub fn part2(input: &[i32]) -> i64 {
    let v = do_round(input, 811589153, 10);

    let zero = v.iter().position(|i| *i == 0).unwrap();
    v[(zero + 1000) % v.len()] + v[(zero + 2000) % v.len()] + v[(zero + 3000) % v.len()]
}

fn parse(input: &str) -> nom::IResult<&str, Vec<i32>> {
    separated_list1(
        line_ending,
        map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 1623178306);
    }
}
