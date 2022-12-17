use std::collections::HashSet;

use shared::input::AocBufReader;

enum JetDirection {
    Left,
    Right,
}

struct JetIterator {
    _string: String,
    _len: usize,
    _idx: usize,
}

impl JetIterator {
    fn new(string: String) -> JetIterator {
        let _len = string.len();
        let _idx = 0;
        JetIterator {
            _string: string,
            _len,
            _idx,
        }
    }

    fn next(&mut self) -> JetDirection {
        let jet_direction: JetDirection;
        match self._string.chars().nth(self._idx).unwrap() {
            '>' => {
                jet_direction = JetDirection::Right;
            }
            '<' => {
                jet_direction = JetDirection::Left;
            }
            _ => {
                panic!("unknown character in jet string");
            }
        };
        self._idx += 1;
        if self._idx == self._len {
            self._idx = 0;
        }

        jet_direction
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RockShape {
    Horizontal,
    Plus,
    RightAngle,
    Vertical,
    Square,
}

struct RockIterator {
    _rocks: [RockShape; 5],
    _len: usize,
    _idx: usize,
}

impl RockIterator {
    fn new() -> RockIterator {
        let _rocks: [RockShape; 5] = [
            RockShape::Horizontal,
            RockShape::Plus,
            RockShape::RightAngle,
            RockShape::Vertical,
            RockShape::Square,
        ];
        let _len: usize = 5;
        let _idx: usize = 0;
        RockIterator { _rocks, _len, _idx }
    }

    fn next(&mut self) -> RockShape {
        let return_value = self._rocks[self._idx].clone();
        self._idx += 1;
        if self._idx == self._len {
            self._idx = 0;
        }
        return_value
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn shift(&self, drow: isize, dcol: isize) -> Coord {
        Coord {
            row: self.row + drow,
            col: self.col + dcol,
        }
    }
}

struct Rock {
    rock_shape: RockShape,
    left_most_bottom: Coord,
}

impl Rock {
    fn all_coords(&self) -> HashSet<Coord> {
        match self.rock_shape {
            RockShape::Horizontal => vec![
                self.left_most_bottom.clone(),
                self.left_most_bottom.shift(0, 1),
                self.left_most_bottom.shift(0, 2),
                self.left_most_bottom.shift(0, 3),
            ]
            .into_iter()
            .collect::<HashSet<Coord>>(),
            RockShape::Plus => vec![
                self.left_most_bottom.clone(),
                self.left_most_bottom.shift(1, -1),
                self.left_most_bottom.shift(1, 0),
                self.left_most_bottom.shift(1, 1),
                self.left_most_bottom.shift(2, 0),
            ]
            .into_iter()
            .collect::<HashSet<Coord>>(),
            RockShape::RightAngle => vec![
                self.left_most_bottom.clone(),
                self.left_most_bottom.shift(0, 1),
                self.left_most_bottom.shift(0, 2),
                self.left_most_bottom.shift(1, 2),
                self.left_most_bottom.shift(2, 2),
            ]
            .into_iter()
            .collect::<HashSet<Coord>>(),
            RockShape::Vertical => vec![
                self.left_most_bottom.clone(),
                self.left_most_bottom.shift(1, 0),
                self.left_most_bottom.shift(2, 0),
                self.left_most_bottom.shift(3, 0),
            ]
            .into_iter()
            .collect::<HashSet<Coord>>(),
            RockShape::Square => vec![
                self.left_most_bottom.clone(),
                self.left_most_bottom.shift(0, 1),
                self.left_most_bottom.shift(1, 0),
                self.left_most_bottom.shift(1, 1),
            ]
            .into_iter()
            .collect::<HashSet<Coord>>(),
        }
    }

    fn left_most_coord(&self) -> isize {
        self.all_coords()
            .iter()
            .map(|coord| coord.col)
            .min()
            .unwrap()
    }

    fn right_most_coord(&self) -> isize {
        self.all_coords()
            .iter()
            .map(|coord| coord.col)
            .max()
            .unwrap()
    }

    fn new_in_chamber(
        rock_shape: RockShape,
        left_edge_coord: isize,
        bottom_edge_coord: isize,
    ) -> Rock {
        let left_most_bottom: Coord = match rock_shape {
            RockShape::Horizontal => Coord {
                row: bottom_edge_coord,
                col: left_edge_coord,
            },
            RockShape::Plus => Coord {
                row: bottom_edge_coord,
                col: left_edge_coord,
            }
            .shift(0, 1),
            RockShape::RightAngle => Coord {
                row: bottom_edge_coord,
                col: left_edge_coord,
            },
            RockShape::Vertical => Coord {
                row: bottom_edge_coord,
                col: left_edge_coord,
            },
            RockShape::Square => Coord {
                row: bottom_edge_coord,
                col: left_edge_coord,
            },
        };
        Rock {
            rock_shape,
            left_most_bottom,
        }
    }

    fn attempt_to_blow(
        &mut self,
        chamber_rocks: &HashSet<Coord>,
        chamber_width: isize,
        jet_direction: JetDirection,
    ) {
        let new_rock_position: Coord;
        match jet_direction {
            JetDirection::Left => {
                new_rock_position = Coord {
                    row: self.left_most_bottom.row,
                    col: self.left_most_bottom.col - 1,
                };
            }
            JetDirection::Right => {
                new_rock_position = Coord {
                    row: self.left_most_bottom.row,
                    col: self.left_most_bottom.col + 1,
                };
            }
        }

        let new_rock = Rock {
            rock_shape: self.rock_shape,
            left_most_bottom: new_rock_position.clone(),
        };
        let rock_hits_other_rocks: bool = new_rock
            .all_coords()
            .intersection(chamber_rocks)
            .cloned()
            .collect::<HashSet<Coord>>()
            .len()
            != 0;
        let rock_hits_wall: bool =
            new_rock.left_most_coord() <= 0 || new_rock.right_most_coord() >= chamber_width + 1;
        if !rock_hits_other_rocks && !rock_hits_wall {
            self.left_most_bottom = new_rock_position;
        }
    }

    fn attempt_to_fall(&mut self, chamber_rocks: &HashSet<Coord>) -> bool {
        let new_rock_position: Coord = Coord {
            row: self.left_most_bottom.row - 1,
            col: self.left_most_bottom.col,
        };
        let new_rock = Rock {
            rock_shape: self.rock_shape,
            left_most_bottom: new_rock_position.clone(),
        };
        if new_rock_position.row == 0 {
            // we hit the bottom
            return false;
        }
        if !(new_rock
            .all_coords()
            .intersection(chamber_rocks)
            .cloned()
            .collect::<HashSet<Coord>>()
            .len()
            == 0)
        {
            // we run into other rocks
            return false;
        }

        self.left_most_bottom = new_rock_position;
        true
    }
}

struct Chamber {
    rocks: HashSet<Coord>,
    rock_iterator: RockIterator,
    jet_iterator: JetIterator,
    width: isize,
}

impl Chamber {
    fn new(width: isize, jet_iterator: JetIterator) -> Chamber {
        let rock_iterator = RockIterator::new();
        Chamber {
            rocks: HashSet::new(),
            rock_iterator,
            jet_iterator,
            width,
        }
    }

    fn add_rock(&mut self) {
        let bottom_edge_coord: isize = self.max_y_coord() + 4;
        let left_edge_coord: isize = 3;

        let rock_shape = self.rock_iterator.next();
        let mut rock = Rock::new_in_chamber(rock_shape, left_edge_coord, bottom_edge_coord);
        loop {
            rock.attempt_to_blow(&self.rocks, self.width, self.jet_iterator.next());
            let rock_moved = rock.attempt_to_fall(&self.rocks);
            if !rock_moved {
                self.rocks.extend(rock.all_coords());
                break;
            }
        }
    }

    fn max_y_coord(&self) -> isize {
        if self.rocks.len() == 0 {
            0
        } else {
            self.rocks.iter().map(|coord| coord.row).max().unwrap()
        }
    }

    fn min_y_coord(&self) -> isize {
        if self.rocks.len() == 0 {
            0
        } else {
            self.rocks.iter().map(|coord| coord.row).min().unwrap()
        }
    }

    fn tower_height(&self) -> isize {
        self.max_y_coord() - self.min_y_coord() + 1
    }
}

fn part_1(mut chamber: Chamber, n_rocks: usize) -> isize {
    for _ in 0..n_rocks {
        chamber.add_rock();
    }

    chamber.tower_height()
}

fn main() {
    let mut reader = AocBufReader::from_string("inputs/part_1.txt");
    let jet_iterator = JetIterator::new(reader.next().unwrap());
    let chamber = Chamber::new(7, jet_iterator);
    println!("{}", part_1(chamber, 2_022));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jet_iterator() {
        let mut reader = AocBufReader::from_string("inputs/example.txt");
        let mut jet_iterator = JetIterator::new(reader.next().unwrap());
        for _ in 0..1000 {
            jet_iterator.next();
        }
    }

    #[test]
    fn test_rock_iterator() {
        let mut rock_iterator = RockIterator::new();
        for _ in 0..1000 {
            rock_iterator.next();
        }
    }
}
