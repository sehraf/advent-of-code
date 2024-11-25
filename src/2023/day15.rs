use aoc_runner_derive::{aoc, aoc_generator};

type T = Vec<String>;

#[aoc_generator(day15)]
#[tracing::instrument(skip(input))]
pub fn input_generator(input: &str) -> T {
    input.split(',').map(|s| s.to_string()).collect()
}

#[tracing::instrument]
fn hash(data: &str) -> u32 {
    let mut state = 0;
    for c in data.chars() {
        state += c as u32;
        state *= 17;
        state %= 256;
    }
    state
}

#[aoc(day15, part1)]
#[tracing::instrument(skip(input))]
pub fn part1(input: &T) -> u32 {
    input.iter().map(|s| hash(s)).sum()
}

#[derive(Debug)]
pub struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

pub enum Op {
    Remove,
    Insert(u32),
}

#[aoc(day15, part2)]
#[tracing::instrument(skip(input))]
pub fn part2(input: &T) -> u32 {
    let mut boxes = (0..256).map(|_| Vec::<Lens>::default()).collect::<Vec<_>>();

    for i in input {
        // parse operation
        let (label, op) = {
            if i.contains('=') {
                let (label, fl) = i.split_once('=').unwrap();
                (label, Op::Insert(fl.parse().unwrap()))
            } else {
                let (label, op) = i.split_once('-').unwrap();
                assert!(op.is_empty());
                (label, Op::Remove)
            }
        };

        // get box
        let box_idx = hash(label) as usize;
        let box_ref = boxes.get_mut(box_idx).unwrap();

        // do it
        match op {
            Op::Remove => box_ref.retain(|l| l.label != label),
            Op::Insert(fl) => match box_ref.iter().position(|l| l.label == label) {
                None => box_ref.push(Lens {
                    label,
                    focal_length: fl,
                }),
                Some(pos) => box_ref.get_mut(pos).unwrap().focal_length = fl,
            },
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(pos, lens)| (box_idx + 1) * (pos + 1) * lens.focal_length as usize)
                .sum::<usize>()
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::{hash, input_generator, part1, part2};

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test_log::test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 1320);
    }

    #[test_log::test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 145);
    }

    #[test_log::test]
    fn test_hash() {
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("qp"), 1);
    }
}
