use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

type T = Input;

#[derive(Debug)]
pub struct Mapping {
    dst_range_start: u32,
    src_range_start: u32,
    range_len: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    s: u32,
    e: u32, // inclusive end
}

#[derive(Debug)]
pub struct Input {
    seeds_p1: Vec<u32>,
    seeds_p2: Vec<Range>,

    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> T {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split_at(7)
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap());
    let seeds_p1 = seeds.clone().collect();
    let seeds_p2 = seeds
        .tuples()
        .map(|(s, l)| Range { s, e: s + l - 1 })
        .collect();

    let read_mappings = |lines: &mut dyn Iterator<Item = &str>| -> Vec<Mapping> {
        let mut mappings = vec![];

        while let Some(next) = lines.next() {
            if next.is_empty() {
                break;
            }

            let mut it = next.split_ascii_whitespace();
            let dst_range_start: u32 = it.next().unwrap().parse().unwrap();
            let src_range_start: u32 = it.next().unwrap().parse().unwrap();
            let range_len: u32 = it.next().unwrap().parse().unwrap();
            debug_assert!(it.next().is_none());

            mappings.push(Mapping {
                dst_range_start,
                src_range_start,
                range_len,
            })
        }

        mappings
    };

    assert!(lines.next() == Some(""));

    assert!(lines.next() == Some("seed-to-soil map:"));
    let seed_to_soil = read_mappings(&mut lines);
    assert!(lines.next() == Some("soil-to-fertilizer map:"));
    let soil_to_fertilizer = read_mappings(&mut lines);
    assert!(lines.next() == Some("fertilizer-to-water map:"));
    let fertilizer_to_water = read_mappings(&mut lines);
    assert!(lines.next() == Some("water-to-light map:"));
    let water_to_light = read_mappings(&mut lines);
    assert!(lines.next() == Some("light-to-temperature map:"));
    let light_to_temperature = read_mappings(&mut lines);
    assert!(lines.next() == Some("temperature-to-humidity map:"));
    let temperature_to_humidity = read_mappings(&mut lines);
    assert!(lines.next() == Some("humidity-to-location map:"));
    let humidity_to_location = read_mappings(&mut lines);

    debug_assert!(lines.next().is_none());

    Input {
        seeds_p1,
        seeds_p2,

        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &T) -> u32 {
    let state = &input.seeds_p1;

    let apply_mapping = |old_state: &Vec<u32>, mapping: &Vec<Mapping>| -> Vec<u32> {
        let mut result = vec![];
        'outer: for prev in old_state {
            for m in mapping {
                let p = prev;
                let s = &m.src_range_start;
                let l = &m.range_len;
                let d = &m.dst_range_start;

                if p >= s && *p < s + l {
                    result.push((p - s) + d);

                    // don't break current loop, continue outer!
                    continue 'outer;
                }
            }

            // no mapping found -> identity
            result.push(*prev);
        }
        result
    };

    let state = apply_mapping(&state, &input.seed_to_soil);
    let state = apply_mapping(&state, &input.soil_to_fertilizer);
    let state = apply_mapping(&state, &input.fertilizer_to_water);
    let state = apply_mapping(&state, &input.water_to_light);
    let state = apply_mapping(&state, &input.light_to_temperature);
    let state = apply_mapping(&state, &input.temperature_to_humidity);
    let state = apply_mapping(&state, &input.humidity_to_location);

    state.iter().min().unwrap().to_owned()
}

#[aoc(day5, part2)]
pub fn part2(input: &T) -> u32 {
    let state = &input.seeds_p2;

    let apply_mapping = |old_state: &Vec<Range>, mapping: &Vec<Mapping>| -> Vec<Range> {
        let move_range = |r: &Range, length: i32| -> Range {
            Range {
                s: (r.s as i32 + length) as u32,
                e: (r.e as i32 + length) as u32,
            }
        };

        let mut result = vec![];
        for prev in old_state {
            let mut to_process = vec![*prev];

            'outer: while let Some(prev) = to_process.pop() {
                for m in mapping {
                    const OLD_CODE: bool = false;
                    if OLD_CODE {
                        // old code

                        let p = prev;
                        let s = &m.src_range_start;
                        let l = &m.range_len;
                        let d = &m.dst_range_start;

                        let mv = *d as i32 - *s as i32;

                        match (p.s, p.e, *s, s + l - 1) {
                            // prev range is below mapping range
                            (_, pe, ms, _) if pe < ms => {}
                            // prev range is beyond mapping range
                            (ps, _, _, me) if ps > me => {}

                            // prev range is subset of mapping range
                            (ps, pe, ms, me) if ps >= ms && pe <= me => {
                                result.push(move_range(&prev, mv));
                                continue 'outer;
                            }
                            // prev range is below mapping range but overlaps partially
                            (ps, pe, ms, me) if ps < ms && pe >= ms && pe <= me => {
                                // split range
                                let range_a = Range { s: ps, e: ms - 1 };
                                let range_b = Range { s: ms, e: pe };

                                to_process.push(range_a);
                                result.push(move_range(&range_b, mv));
                                continue 'outer;
                            }
                            // prev range is above mapping range but overlaps partially
                            (ps, pe, ms, me) if ps >= ms && ps <= me && pe > me => {
                                // split range
                                let range_a = Range { s: ps, e: me };
                                let range_b = Range { s: me + 1, e: pe };

                                result.push(move_range(&range_a, mv));
                                to_process.push(range_b);
                                continue 'outer;
                            }
                            // mapping range is subset of prev range
                            (ps, pe, ms, me) if ps < ms && pe > me => {
                                // split range
                                let range_a = Range { s: ps, e: ms - 1 };
                                let range_b = Range { s: ms, e: me };
                                let range_c = Range { s: me + 1, e: pe };

                                to_process.push(range_a);
                                result.push(move_range(&range_b, mv));
                                to_process.push(range_c);
                                continue 'outer;
                            }
                            (_, _, _, _) => unreachable!(),
                        }
                    } else {
                        // new code

                        // prev
                        let ps = &prev.s;
                        let pe = &prev.e;
                        // mapping
                        let ms = &m.src_range_start;
                        let me = &(m.src_range_start + m.range_len - 1);

                        let mv = m.dst_range_start as i32 - *ms as i32;

                        // intersection
                        let is = ps.max(ms);
                        let ie = pe.min(me);

                        if is <= ie {
                            result.push(move_range(&Range { s: *is, e: *ie }, mv));
                            if ps < is {
                                to_process.push(Range { s: *ps, e: is - 1 });
                            }
                            if ie < pe {
                                to_process.push(Range { s: ie + 1, e: *pe });
                            }
                            continue 'outer;
                        }
                    }
                }

                // no mapping found -> identity
                result.push(prev);
            }
        }
        result
    };

    let state = apply_mapping(&state, &input.seed_to_soil);
    let state = apply_mapping(&state, &input.soil_to_fertilizer);
    let state = apply_mapping(&state, &input.fertilizer_to_water);
    let state = apply_mapping(&state, &input.water_to_light);
    let state = apply_mapping(&state, &input.light_to_temperature);
    let state = apply_mapping(&state, &input.temperature_to_humidity);
    let state = apply_mapping(&state, &input.humidity_to_location);

    state.iter().map(|r| r.s).min().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 35);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 46);
    }
}
