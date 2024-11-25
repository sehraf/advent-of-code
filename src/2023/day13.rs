use aoc_runner_derive::{aoc, aoc_generator};
use tracing::info;

#[derive(Debug)]
pub struct Input {
    rows: Vec<String>,
    columns: Vec<String>,
    size: (usize, usize),
}

type T = Input;

#[aoc_generator(day13)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> Vec<T> {
    input
        .split("\n\n")
        .map(|block| {
            let rows: Vec<String> = block.lines().map(|line| line.to_string()).collect();
            let x = rows.first().unwrap().len();
            let y = rows.len();

            let mut columns = vec![];
            for idx in 0..x {
                let mut column = String::new();
                for row in &rows {
                    column += &row.chars().nth(idx).unwrap().to_string();
                }
                columns.push(column);
            }

            Input {
                rows,
                columns,
                size: (x, y),
            }
        })
        .collect()
}

#[tracing::instrument]
fn locate_potential_smudge(a: &str, b: &str) -> Option<usize> {
    let diffs: Vec<_> = a
        .chars()
        .zip(b.chars())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .collect();

    if diffs.len() == 1 {
        Some(diffs.first().unwrap().0)
    } else {
        None
    }
}

#[tracing::instrument(skip(data))]
fn valid_mirror(
    range: usize,
    mirror_pos: usize,
    data: &Vec<String>,
) -> (bool, Option<(usize, usize)>) {
    let mut valid = None;
    let mut smudge = None;

    for test in 0..range - 1 {
        if mirror_pos < test {
            break;
        }
        let a = data.get(mirror_pos - test);
        let b = data.get(mirror_pos + test + 1);

        match (a, b) {
            // normale case, both lines exist
            (Some(a), Some(b)) => {
                match (a, b, valid) {
                    // first successful test
                    (a, b, None) if a == b => valid = Some(true),
                    // first unsuccessful test
                    (a, b, None) if a != b => {
                        if let Some(s) = locate_potential_smudge(a, b) {
                            smudge = Some((mirror_pos - test, s));
                        }
                        break;
                    }

                    // everything is fine
                    (a, b, Some(true)) if a == b => {}
                    // failed
                    (a, b, Some(true)) if a != b => {
                        if let Some(s) = locate_potential_smudge(a, b) {
                            smudge = Some((mirror_pos - test, s));
                        }
                        valid = Some(false);
                        break;
                    }

                    (_, _, _) => unreachable!(),
                }
            }

            // end of area
            (Some(_), None) => break,
            (None, Some(_)) => break,
            (None, None) => break,
        }
    }

    info!("result: {valid:?}, {smudge:?}");
    (valid.unwrap_or_default(), smudge)
}

#[aoc(day13, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &[T]) -> u32 {
    input
        .iter()
        .map(|input| {
            let mut score = 0;

            // horizontal mirror
            for row in 0..input.size.1 {
                if valid_mirror(input.size.1, row, &input.rows).0 {
                    score += (row + 1) * 100;
                }
            }

            // vertical mirror
            for col in 0..input.size.0 {
                if valid_mirror(input.size.0, col, &input.columns).0 {
                    score += col + 1;
                }
            }

            score as u32
        })
        .sum()
}

#[aoc(day13, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &[T]) -> u32 {
    let clone_and_replace = |input: &Vec<String>, pos_a: usize, pos_b: usize| {
        let old = input.iter().nth(pos_a).unwrap().chars().nth(pos_b).unwrap();
        let new = match old {
            '.' => '#',
            '#' => '.',
            _ => unreachable!(),
        };
        let old_value = input.iter().nth(pos_a).unwrap();
        let mut new_values = input.to_owned();
        *new_values.iter_mut().nth(pos_a).unwrap() = old_value
            .chars()
            .enumerate()
            .map(|(idx, c)| if idx == pos_b { new } else { c })
            .collect();
        new_values
    };

    input
        .iter()
        .map(|input| {
            let mut score = 0;
            // horizontal mirror
            for row in 0..input.size.1 {
                let (_, smudge) = valid_mirror(input.size.1, row, &input.rows);

                if let Some((test, pos)) = smudge {
                    info!("smudge at [{pos},{test}]");

                    let new_rows = clone_and_replace(&input.rows, test, pos);

                    if valid_mirror(input.size.1, row, &new_rows).0 {
                        score += (row + 1) * 100;
                    }
                }
            }

            // vertical mirror
            for col in 0..input.size.0 {
                let (_, smudge) = valid_mirror(input.size.0, col, &input.columns);

                if let Some((test, pos)) = smudge {
                    info!("smudge at [{test},{pos}]");

                    let new_columns = clone_and_replace(&input.columns, test, pos);

                    if valid_mirror(input.size.1, col, &new_columns).0 {
                        score += col + 1;
                    }
                }
            }

            score as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 405);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 400);
    }
}
