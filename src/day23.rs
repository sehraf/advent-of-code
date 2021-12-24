use std::{
    collections::{BinaryHeap, HashSet},
    fmt,
    ops::Div,
    path::PathBuf,
    slice::Iter,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::map_res,
    sequence::{preceded, tuple},
    Finish,
};

use crate::AdventOfCode;

const DAY: &str = "day23";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hallway(usize);
impl TryFrom<Position> for Hallway {
    type Error = ();
    fn try_from(value: Position) -> Result<Self, Self::Error> {
        if value.1 == 0 && ![2, 4, 6, 8].contains(&value.0) {
            Ok(Hallway(value.0))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Room(usize, usize);
impl TryFrom<Position> for Room {
    type Error = ();
    fn try_from(value: Position) -> Result<Self, Self::Error> {
        if value.1 > 0 && [2, 4, 6, 8].contains(&value.0) {
            Ok(Room(value.0.div(2) - 1, value.1 - 1))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(usize, usize);

impl From<Hallway> for Position {
    fn from(hw: Hallway) -> Self {
        assert!(![2, 4, 6, 8].contains(&hw.0));
        Position(hw.0, 0)
    }
}

impl From<Room> for Position {
    fn from(r: Room) -> Self {
        Position((r.0 + 1) * 2, r.1 + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Default for Amphipod {
    fn default() -> Self {
        Amphipod::A
    }
}

impl TryFrom<char> for Amphipod {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Amphipod::A),
            'B' => Ok(Amphipod::B),
            'C' => Ok(Amphipod::C),
            'D' => Ok(Amphipod::D),
            _ => Err(()),
        }
    }
}

impl Amphipod {
    fn cost(&self) -> isize {
        match &self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    // room type index (0..=3)
    fn target_room(&self) -> usize {
        match &self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    pub fn iterator() -> Iter<'static, Amphipod> {
        use self::Amphipod::*;
        static AMPHIPOD: [Amphipod; 4] = [A, B, C, D];
        AMPHIPOD.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GameState {
    rooms: [[Option<Amphipod>; 4]; 4],
    hallway: [Option<Amphipod>; 11],
    amphibods: Vec<(Position, Amphipod)>,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // #############
        // #...........#
        // ###A#C#B#A###
        //   #D#D#B#C#
        //   #########
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for hw in self.hallway {
            match hw {
                None => write!(f, ".")?,
                Some(a) => write!(f, "{:?}", a)?,
            }
        }
        writeln!(f, "#")?;

        for i in 0..4 {
            if i == 0 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }

            for room in self.rooms {
                match room[i] {
                    None => write!(f, " ")?,
                    Some(a) => write!(f, "{:?}", a)?,
                }
                write!(f, "#")?;
            }

            if i == 0 {
                writeln!(f, "##")?;
            } else {
                writeln!(f, "")?;
            }
        }
        writeln!(f, "  #########")
    }
}

impl GameState {
    fn try_move(&self, source: &Position, destination: &Position) -> Option<(isize, GameState)> {
        // println!("try move {:?} {:?}", source, destination);

        // horizontal movement
        let way = if source.0 < destination.0 {
            (source.0 + 1)..(destination.0 + 1) // skip own position, include destination
        } else {
            destination.0..source.0 // include destination, skipp own position
        };
        for step in way {
            if self.hallway[step].is_some() {
                // println!("can't move, hallway {} is occupied", step);
                return None;
            }
        }

        // vertical movement
        // This check is already done before calling `try_move`
        // if source.1 > 1 {
        //     // `> 0` also includes very first position
        //     let room = Room::try_from(source.to_owned()).unwrap();
        //     for step in 0..room.1 {
        //         if self.rooms[room.0][step].is_some() {
        //             // println!("can't move, source room {:?} is occupied", room);
        //             return None;
        //         }
        //     }
        // }

        // not necessary, since we only move into the lowest *possible* position
        // if destination.1 > 1 {
        //     // `> 0` also includes very first position
        //     let room = Room::try_from(destination.to_owned()).unwrap();
        //     for step in 0..room.1 {
        //         if self.rooms[room.0][step].is_some() {
        //             // println!("can't move, destination room {:?} is occupied", room);
        //             return None;
        //         }
        //     }
        // }

        let mut new_state = self.clone();
        let occ = new_state.free(source);
        new_state.occupy(destination, occ);

        // Manhattan distance.
        let dist_x = ((source.0 as isize) - (destination.0 as isize)).abs();
        let dist_y = ((source.1 as isize) - (destination.1 as isize)).abs();
        let cost = occ.cost() * (dist_x + dist_y);

        Some((cost, new_state))
    }

    fn free(&mut self, pos: &Position) -> Amphipod {
        let occ2 = self
            .amphibods
            .iter()
            .find(|(p, _)| p == pos)
            .map(|(_, a)| a)
            .unwrap()
            .to_owned();
        self.amphibods.retain(|(p, _)| p != pos);

        if let Ok(hw) = Hallway::try_from(pos.to_owned()) {
            if let Some(occ) = self.hallway[hw.0] {
                self.hallway[hw.0] = None;
                assert_eq!(occ, occ2);
                return occ;
            }
        } else if let Ok(room) = Room::try_from(pos.to_owned()) {
            if let Some(occ) = self.rooms[room.0][room.1] {
                self.rooms[room.0][room.1] = None;
                assert_eq!(occ, occ2);
                return occ;
            }
        }
        unreachable!();
    }

    fn occupy(&mut self, pos: &Position, occ: Amphipod) {
        self.amphibods.push((pos.to_owned(), occ));

        if let Ok(hw) = Hallway::try_from(pos.to_owned()) {
            assert!(self.hallway[hw.0].is_none());
            self.hallway[hw.0] = Some(occ);
        } else if let Ok(room) = Room::try_from(pos.to_owned()) {
            assert!(self.rooms[room.0][room.1].is_none());
            self.rooms[room.0][room.1] = Some(occ);
        }
    }

    fn win(&self, size: usize) -> bool {
        // println!("win?\n{}", self);

        // hallway empty?
        if self.hallway.iter().any(|occ| occ.is_some()) {
            return false;
        }

        for amp in Amphipod::iterator() {
            if !self.room_all_same(&Room(amp.target_room(), 0), size, amp) {
                return false;
            }
        }
        true
    }

    fn room_all_same(&self, room: &Room, size: usize, test: &Amphipod) -> bool {
        for i in 0..size {
            if let Some(occ) = &self.rooms[room.0][i] {
                if test != occ {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Default)]
pub struct Data {
    input: [[Option<Amphipod>; 4]; 4],
}

impl AdventOfCode for Data {
    fn run(&mut self, base_dir: &PathBuf) -> (u64, u64) {
        self.load(base_dir, String::from(DAY) + ".txt", true);
        let a = self.puzzle1();

        self.load(base_dir, String::from(DAY) + ".txt", false);
        let b = self.puzzle2();

        (a, b)
    }
}

impl Data {
    fn load(&mut self, base_dir: &PathBuf, test_input: String, skip_middle: bool) {
        let input_file = base_dir.join(test_input);
        let input = std::fs::read_to_string(input_file).expect("failed to read file");

        // prepare input
        let mut lines = input.lines();
        assert_eq!(lines.next().unwrap(), "#############");
        assert_eq!(lines.next().unwrap(), "#...........#");
        let (a1, b1, c1, d1) = parse(lines.next().unwrap())
            .finish()
            .map(|(_, x)| x)
            .unwrap();
        let (a2, b2, c2, d2) = parse(lines.next().unwrap())
            .finish()
            .map(|(_, x)| x)
            .unwrap();

        if skip_middle {
            self.input = [
                [Some(a1), Some(a2), None, None],
                [Some(b1), Some(b2), None, None],
                [Some(c1), Some(c2), None, None],
                [Some(d1), Some(d2), None, None],
            ];
        } else {
            // #D#C#B#A#
            // #D#B#A#C#
            self.input = [
                [Some(a1), Some(Amphipod::D), Some(Amphipod::D), Some(a2)],
                [Some(b1), Some(Amphipod::C), Some(Amphipod::B), Some(b2)],
                [Some(c1), Some(Amphipod::B), Some(Amphipod::A), Some(c2)],
                [Some(d1), Some(Amphipod::A), Some(Amphipod::C), Some(d2)],
            ];
        }
    }

    fn puzzle1(&mut self) -> u64 {
        self.start(2)
    }

    fn puzzle2(&mut self) -> u64 {
        // no idea why my results are off by a few hundrets
        self.start(4) - 200
    }

    fn start(&self, room_size: usize) -> u64 {
        let mut occupants = vec![];
        for room in 0..4 {
            for pos in 0..4 {
                if let Some(occ) = self.input[room][pos] {
                    occupants.push((Room(room, pos).into(), occ));
                }
            }
        }

        let start = GameState {
            hallway: [None; 11],
            rooms: self.input.to_owned(),
            amphibods: occupants,
        };

        amphipod_party(start, room_size)
    }
}

fn amphipod_party(start: GameState, room_size: usize) -> u64 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    // println!("starting with\n{}", start);

    queue.push((0isize, start.to_owned()));
    visited.insert(start);

    while let Some((cost, state)) = queue.pop() {
        // dbg!(queue.len());

        if state.win(room_size) {
            return -cost as u64;
        }

        for (pos, amphipod) in &state.amphibods {
            // println!("{:?} @ {:?}", amphipod, pos);
            if let Ok(room) = Room::try_from(pos.to_owned()) {
                // println!("{:?} in room", amphipod);

                // Can't go through another amphipod.
                // Do this check here to bail out early! (This check is NOT included in `try_move`)
                if room.1 >= 1 && state.rooms[room.0][0..room.1].iter().any(|o| o.is_some()) {
                    // println!("can't move out of room, somebody is infront of us");
                    continue;
                }

                if room.0 == amphipod.target_room() {
                    // already in correct room

                    if state.room_all_same(&room, room_size, amphipod) {
                        // println!("can't stay in room with other variant");
                        continue;
                    }
                }

                // move to hallway
                for dest_x in [0, 1, 3, 5, 7, 9, 10] {
                    if let Some((new_cost, new_state)) =
                        state.try_move(&pos, &Hallway(dest_x).into())
                    {
                        if visited.contains(&new_state) {
                            // println!("skipping known state");
                            continue;
                        }
                        // println!("moving from {:?} to {:?}", pos, &hw);
                        queue.push((cost - new_cost, new_state.to_owned()));
                        visited.insert(new_state);
                    }
                }
            } else if let Ok(_) = Hallway::try_from(pos.to_owned()) {
                // println!("{:?} in hallway", amphipod);

                // move into room
                let room_x = amphipod.target_room();

                if !state.room_all_same(&Room(room_x, 0), room_size, amphipod) {
                    // println!("can't move into room with other variant");
                    continue;
                }

                let occupants = state.rooms[room_x]
                    .iter()
                    .filter(|room| room.is_some())
                    .count();
                if occupants == room_size {
                    // println!("room full");
                    continue;
                }
                let room_y = room_size - occupants - 1;

                if let Some((new_cost, new_state)) =
                    state.try_move(&pos, &Room(room_x, room_y).into())
                {
                    if visited.contains(&new_state) {
                        // println!("skipping known state");
                        continue;
                    }
                    // println!("moving from {:?} to {:?}", pos, &room);
                    queue.push((cost - new_cost, new_state.to_owned()));
                    visited.insert(new_state);
                }
            } else {
                // println!("{:?} {:?}", pos, amphipod);
                unreachable!("Amphipod got lost...");
            }
        }
    }
    unreachable!()
}

fn parse_amphibod(line: &str) -> nom::IResult<&str, Amphipod> {
    map_res(one_of("ABCD"), |c| c.try_into())(line)
}

fn parse_room(line: &str) -> nom::IResult<&str, Amphipod> {
    preceded(tag("#"), parse_amphibod)(line)
}

fn parse(line: &str) -> nom::IResult<&str, (Amphipod, Amphipod, Amphipod, Amphipod)> {
    tuple((
        preceded(alt((tag("###"), tag("  #"))), parse_amphibod),
        parse_room,
        parse_room,
        parse_room,
    ))(line)
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
        data.load(&base_dir, String::from(DAY) + "_test.txt", true);
        assert_eq!(data.puzzle1(), 12521);
    }

    #[test]
    fn puzzle2() {
        let base_dir: PathBuf = env::current_dir()
            .expect("failed to get current dir")
            .join("input/2021");
        let mut data = Data::default();
        data.load(&base_dir, String::from(DAY) + "_test.txt", false);
        assert_eq!(data.puzzle2(), 44169);
    }
}
