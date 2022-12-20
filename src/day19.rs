use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};

type OreAmount = u16;
type Recipe = [OreAmount; 4];

#[derive(Debug)]
pub struct Blueprint {
    recipes: [Recipe; 4],
}

#[derive(Debug)]
pub struct State {
    ores: [OreAmount; 4],
    robots: [u16; 4],
    time: u16,
}

fn recurse_simulation(
    blueprint: &Blueprint,
    state: State,
    max_time: u16,
    max_robots: &[u16; 4],
    max_geodes: &mut OreAmount,
) {
    let mut has_recursed = false;
    for i in 0..4 {
        if state.robots[i] == max_robots[i] {
            continue;
        }
        let recipe = &blueprint.recipes[i];
        // Find the limiting ore for the recipe.
        let wait_time = (0..3)
            .filter_map(|ore_type| {
                if recipe[ore_type] == 0 {
                    None
                } else if recipe[ore_type] <= state.ores[ore_type] {
                    Some(0)
                } else if state.robots[ore_type] == 0 {
                    // No robot yet, we can't build it (it takes more than max_time to build it).
                    Some(max_time as u16 + 1)
                } else {
                    Some(
                        (recipe[ore_type] - state.ores[ore_type] + state.robots[ore_type] - 1)
                            / state.robots[ore_type],
                    )
                }
            })
            .max()
            .unwrap();
        let time_finished = state.time + wait_time + 1;
        if time_finished >= max_time {
            continue;
        }
        let mut new_ores = [0; 4];
        let mut new_robots = [0; 4];
        for o in 0..4 {
            new_ores[o] = state.ores[o] + state.robots[o] * (wait_time + 1) - recipe[o];
            new_robots[o] = state.robots[o] + u16::from(o == i);
        }
        let remaining_time = max_time - time_finished;
        // If we were to build only geode robots every turn, could we beat the current max?
        if ((remaining_time - 1) * remaining_time) / 2
            + new_ores[3]
            + remaining_time * new_robots[3]
            < *max_geodes
        {
            continue;
        }
        has_recursed = true;
        recurse_simulation(
            blueprint,
            State {
                ores: new_ores,
                robots: new_robots,
                time: time_finished,
            },
            max_time,
            max_robots,
            max_geodes,
        );
    }
    if !has_recursed {
        // We couldn't make new robots, so this is the best this branch can do.
        *max_geodes = std::cmp::max(
            *max_geodes,
            state.ores[3] + state.robots[3] * (max_time - state.time) as u16,
        );
    }
}

fn simulate_blueprint(blueprint: &Blueprint, max_time: u16) -> OreAmount {
    let mut max_robots = [u16::max_value(); 4];
    for i in 0..3 {
        max_robots[i] = blueprint.recipes.iter().map(|r| r[i]).max().unwrap();
    }
    let mut max_geodes = 0;
    recurse_simulation(
        blueprint,
        State {
            ores: [0; 4],
            robots: [1, 0, 0, 0],
            time: 0,
        },
        max_time,
        &max_robots,
        &mut max_geodes,
    );
    max_geodes
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    let (rem, ret) = parse(input).expect("failed to parse input");
    assert!(rem.is_empty(), "remaining {rem}");
    ret
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, b)| simulate_blueprint(b, 24) as usize * (i + 1))
        .sum::<usize>()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Blueprint]) -> usize {
    input
        .iter()
        .take(3)
        .map(|b| simulate_blueprint(b, 32) as usize)
        .product::<usize>()
}

fn parse_blueprint(input: &str) -> nom::IResult<&str, Blueprint> {
    let (rem, _id) = delimited(tag("Blueprint "), complete::u64, tag(":"))(input)?;
    let (rem, ore) = delimited(tag(" Each ore robot costs "), complete::u16, tag(" ore."))(rem)?;
    let (rem, clay) = delimited(tag(" Each clay robot costs "), complete::u16, tag(" ore."))(rem)?;
    let (input, obsidian) = delimited(
        tag(" Each obsidian robot costs "),
        separated_pair(complete::u16, tag(" ore and "), complete::u16),
        tag(" clay."),
    )(rem)?;
    let (rem, geode) = delimited(
        tag(" Each geode robot costs "),
        separated_pair(complete::u16, tag(" ore and "), complete::u16),
        tag(" obsidian."),
    )(input)?;

    let ore_recipe = [ore, 0, 0, 0];
    let clay_recipe = [clay, 0, 0, 0];
    let obsidian_recipe = [obsidian.0, obsidian.1, 0, 0];
    let geode_recipe = [geode.0, 0, geode.1, 0];

    Ok((
        rem,
        Blueprint {
            recipes: [ore_recipe, clay_recipe, obsidian_recipe, geode_recipe],
        },
    ))
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Blueprint>> {
    separated_list1(line_ending, parse_blueprint)(input)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(INPUT)), 33);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(INPUT)), 56 * 62);
    }
}
