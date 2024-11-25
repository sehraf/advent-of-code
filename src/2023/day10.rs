use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use aoc_runner_derive::{aoc, aoc_generator};

type T = (HashMap<Pos, Tile>, Pos);

// (0,0) is top left
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn north(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn west(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Vertical,   // | is a vertical pipe connecting north and south.
    Horizontal, // - is a horizontal pipe connecting east and west.
    NorthEast,  // L is a 90-degree bend connecting north and east.
    NorthWest,  // J is a 90-degree bend connecting north and west.
    SouthWest,  // 7 is a 90-degree bend connecting south and west.
    SouthEast,  // F is a 90-degree bend connecting south and east.
    Ground,     // . is ground; there is no pipe in this tile.
    Start, // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            c => unreachable!("character {c} is unknown"),
        }
    }
}

impl Tile {
    pub fn get_directions(&self, pos: &Pos) -> (Pos, Pos) {
        match self {
            Tile::Vertical => (pos.north(), pos.south()),
            Tile::Horizontal => (pos.east(), pos.west()),
            Tile::NorthEast => (pos.north(), pos.east()),
            Tile::NorthWest => (pos.north(), pos.west()),
            Tile::SouthWest => (pos.south(), pos.west()),
            Tile::SouthEast => (pos.east(), pos.south()),
            Tile::Ground | Tile::Start => {
                unreachable!()
            }
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<T> {
    let mut map = HashMap::new();
    let mut start = Pos { x: 0, y: 0 };

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let tile = c.into();
            match tile {
                Tile::Start => {
                    debug_assert_eq!(start, Pos { x: 0, y: 0 });
                    start = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                _ => {}
            }
            map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                tile,
            );
        })
    });

    debug_assert_ne!(start, Pos { x: 0, y: 0 });

    let (_, tile) = find_start_candidates(&map, &start);
    map.entry(start).and_modify(|e| *e = tile);

    vec![(map, start)]
}

fn find_start_candidates(map: &HashMap<Pos, Tile>, start: &Pos) -> (VecDeque<Pos>, Tile) {
    let mut next_candidate = VecDeque::new();

    let mut north = false;
    let mut east = false;
    let mut south = false;
    let mut west = false;

    // check for possible starting points connected to S
    // north
    let next = start.north();
    if let Some(new_tile) = map.get(&next) {
        if match new_tile {
            Tile::Vertical | Tile::SouthWest | Tile::SouthEast => true,

            Tile::Horizontal | Tile::NorthEast | Tile::NorthWest | Tile::Ground => false,

            Tile::Start => {
                unreachable!()
            }
        } {
            next_candidate.push_back(next);
            north = true;
        }
    }

    // east
    let next = start.east();
    if let Some(new_tile) = map.get(&next) {
        if match new_tile {
            Tile::Horizontal | Tile::NorthWest | Tile::SouthWest => true,

            Tile::Vertical | Tile::NorthEast | Tile::SouthEast | Tile::Ground => false,

            Tile::Start => {
                unreachable!()
            }
        } {
            next_candidate.push_back(next);
            east = true;
        }
    }

    // south
    let next = start.south();
    if let Some(new_tile) = map.get(&next) {
        if match new_tile {
            Tile::Vertical | Tile::NorthEast | Tile::NorthWest => true,

            Tile::Horizontal | Tile::SouthEast | Tile::SouthWest | Tile::Ground => false,

            Tile::Start => {
                unreachable!()
            }
        } {
            next_candidate.push_back(next);
            south = true;
        }
    }

    // west
    let next = start.west();
    if let Some(new_tile) = map.get(&next) {
        if match new_tile {
            Tile::Horizontal | Tile::NorthEast | Tile::SouthEast => true,

            Tile::Vertical | Tile::NorthWest | Tile::SouthWest | Tile::Ground => false,

            Tile::Start => {
                unreachable!()
            }
        } {
            next_candidate.push_back(next);
            west = true;
        }
    }

    assert_eq!(next_candidate.len(), 2);

    let tile = {
        match (north, east, south, west) {
            (true, false, true, false) => Tile::Vertical,
            (false, true, false, true) => Tile::Horizontal,

            (true, true, false, false) => Tile::NorthEast,
            (true, false, false, true) => Tile::NorthWest,

            (false, true, true, false) => Tile::SouthEast,
            (false, false, true, true) => Tile::SouthWest,

            _ => unreachable!(),
        }
    };

    (next_candidate, tile)
}

fn traverse_map(map: &HashMap<Pos, Tile>, start: &Pos) -> HashSet<Pos> {
    // BFS or DFS? -> judging by reddit, it's probably one giant loop -> DFS
    let (mut next_candidate, _) = find_start_candidates(map, start);
    let mut last = *start;
    let mut visited: HashSet<Pos> = HashSet::new();

    while let Some(current) = next_candidate.pop_back() {
        assert_eq!(visited.insert(current), true);

        if &current == start {
            return visited;
        }

        let tile = map.get(&current).expect("must exist");
        let (a, b) = tile.get_directions(&current);

        match (a, b, last) {
            (a, b, l) if a == l => {
                next_candidate.push_back(b);
            }
            (a, b, l) if b == l => {
                next_candidate.push_back(a);
            }
            (_, _, _) => {
                unreachable!()
            }
        }

        last = current;
    }

    unreachable!()
}

#[aoc(day10, part1)]
pub fn part1(input: &[T]) -> u32 {
    let (map, start) = input.first().unwrap();

    let visited = traverse_map(map, start);
    ((visited.len() + 1) / 2) as u32
}

#[derive(Debug, Clone, Copy)]
enum State {
    Outside,
    OnLine(Tile, bool),
    Inside,
}

impl State {
    pub fn flip(&mut self) {
        *self = match self {
            State::Inside => State::Outside,
            State::Outside => State::Inside,
            State::OnLine(_, _) => unreachable!(),
        }
    }

    pub fn expect(&mut self, tile: Tile) {
        *self = match self {
            State::Inside => State::OnLine(tile, true),
            State::Outside => State::OnLine(tile, false),
            State::OnLine(_, _) => unreachable!(),
        }
    }

    pub fn test(&mut self, tile: &Tile) {
        *self = match self {
            State::Inside | State::Outside => unreachable!(),

            // flip
            State::OnLine(expected, true) if expected == tile => State::Outside,
            State::OnLine(expected, false) if expected == tile => State::Inside,

            // no flip
            State::OnLine(expected, true) if expected != tile => State::Inside,
            State::OnLine(expected, false) if expected != tile => State::Outside,

            _ => unreachable!()
        }
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &[T]) -> u32 {
    let (map, start) = input.first().unwrap();

    let visited = traverse_map(map, start);

    let mut x = 0;
    let mut y = 0;

    let mut state = State::Outside;
    let mut counter = 0;

    loop {
        let pos = Pos { x, y };

        if visited.contains(&pos) {
            let tile = map.get(&pos).unwrap();
            match (state.to_owned(), tile) {
                // easy flip
                (_, Tile::Vertical) => state.flip(),

                // start line
                (_, Tile::SouthEast) => state.expect(Tile::NorthWest),
                (_, Tile::NorthEast) => state.expect(Tile::SouthWest),

                // on line
                (State::OnLine(_, _), Tile::Horizontal) => {}

                // end line
                (State::OnLine(_, _), Tile::NorthWest) => state.test(&Tile::NorthWest),
                (State::OnLine(_, _), Tile::SouthWest) => state.test(&Tile::SouthWest),

                _ => unreachable!(),
            }
        } else {
            match state {
                State::Outside => {}
                State::Inside => counter += 1,
                State::OnLine(_, _) => unreachable!(),
            }
        }

        // "guess" next position
        let next_pos = pos.east();
        if map.contains_key(&next_pos) {
            x += 1;
        } else {
            // next row
            x = 0;
            y += 1;

            if map.contains_key(&Pos { x, y }) {
                continue;
            } else {
                break;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    const INPUT2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test11() {
        assert_eq!(part1(&input_generator(INPUT2)), 4);
    }

    #[test]
    fn test12() {
        assert_eq!(part1(&input_generator(INPUT)), 8);
    }

    const INPUT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test21() {
        assert_eq!(part2(&input_generator(INPUT3)), 4);
    }

    const INPUT4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test22() {
        assert_eq!(part2(&input_generator(INPUT4)), 8);
    }

    const INPUT5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test23() {
        assert_eq!(part2(&input_generator(INPUT5)), 10);
    }
}
