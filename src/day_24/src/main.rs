use std::collections::{HashMap, HashSet};

use shared::input::AocBufReader;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

struct BoundingBox {
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
    all_spaces: HashSet<Coord>,
}

impl BoundingBox {
    fn new(min_row: usize, max_row: usize, min_col: usize, max_col: usize) -> BoundingBox {
        let mut all_spaces: HashSet<Coord> = HashSet::new();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                all_spaces.insert(Coord { row, col });
            }
        }

        BoundingBox {
            min_row,
            max_row,
            min_col,
            max_col,
            all_spaces,
        }
    }
}

struct BlizzardMap {
    start: Coord,
    end: Coord,
    bounding_box: BoundingBox,
    blizzards_north: HashSet<Coord>,
    blizzards_east: HashSet<Coord>,
    blizzards_south: HashSet<Coord>,
    blizzards_west: HashSet<Coord>,
    _t: usize,
    _open_spaces_by_t: HashMap<usize, HashSet<Coord>>,
}

impl BlizzardMap {
    fn new(
        start: Coord,
        end: Coord,
        bounding_box: BoundingBox,
        blizzards_north: HashSet<Coord>,
        blizzards_east: HashSet<Coord>,
        blizzards_south: HashSet<Coord>,
        blizzards_west: HashSet<Coord>,
    ) -> BlizzardMap {
        let mut blizzard_map = BlizzardMap {
            start,
            end,
            bounding_box,
            blizzards_north,
            blizzards_east,
            blizzards_south,
            blizzards_west,
            _t: 0,
            _open_spaces_by_t: HashMap::new(),
        };

        let all_spaces = blizzard_map
            .bounding_box
            .all_spaces
            .iter()
            .cloned()
            .collect::<HashSet<Coord>>();
        let occupied_spaces = blizzard_map.blizzard_spaces();
        blizzard_map._open_spaces_by_t.insert(
            0,
            all_spaces
                .difference(&occupied_spaces)
                .cloned()
                .collect::<HashSet<Coord>>(),
        );

        blizzard_map
    }

    fn blizzard_spaces(&self) -> HashSet<Coord> {
        self.blizzards_north
            .union(&self.blizzards_east)
            .cloned()
            .collect::<HashSet<Coord>>()
            .union(&self.blizzards_south)
            .cloned()
            .collect::<HashSet<Coord>>()
            .union(&self.blizzards_west)
            .cloned()
            .collect::<HashSet<Coord>>()
    }

    fn calculate_next(&mut self) {
        self._t += 1;

        let blizzards_north: HashSet<Coord> = self
            .blizzards_north
            .iter()
            .map(|coord| {
                let row: usize;
                let col: usize = coord.col;
                if coord.row == self.bounding_box.min_row {
                    row = self.bounding_box.max_row;
                } else {
                    row = coord.row - 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_north = blizzards_north;

        let blizzards_east: HashSet<Coord> = self
            .blizzards_east
            .iter()
            .map(|coord| {
                let row: usize = coord.row;
                let col: usize;
                if coord.col == self.bounding_box.max_col {
                    col = self.bounding_box.min_col;
                } else {
                    col = coord.col + 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_east = blizzards_east;

        let blizzards_south: HashSet<Coord> = self
            .blizzards_south
            .iter()
            .map(|coord| {
                let row: usize;
                let col: usize = coord.col;
                if coord.row == self.bounding_box.max_row {
                    row = self.bounding_box.min_row;
                } else {
                    row = coord.row + 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_south = blizzards_south;

        let blizzards_west: HashSet<Coord> = self
            .blizzards_west
            .iter()
            .map(|coord| {
                let row: usize = coord.row;
                let col: usize;
                if coord.col == self.bounding_box.min_col {
                    col = self.bounding_box.max_col;
                } else {
                    col = coord.col - 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_west = blizzards_west;

        let all_spaces = self
            .bounding_box
            .all_spaces
            .iter()
            .cloned()
            .collect::<HashSet<Coord>>();
        let occupied_spaces = self.blizzard_spaces();
        self._open_spaces_by_t.insert(
            self._t,
            all_spaces
                .difference(&occupied_spaces)
                .cloned()
                .collect::<HashSet<Coord>>(),
        );
    }

    fn get_open_spaces_at_time_t(&mut self, t: usize) -> &HashSet<Coord> {
        while self._t < t {
            self.calculate_next()
        }

        self._open_spaces_by_t.get(&t).unwrap()
    }
}

fn parse_input(reader: AocBufReader) -> BlizzardMap {
    let mut blizzards_north: HashSet<Coord> = HashSet::new();
    let mut blizzards_east: HashSet<Coord> = HashSet::new();
    let mut blizzards_south: HashSet<Coord> = HashSet::new();
    let mut blizzards_west: HashSet<Coord> = HashSet::new();

    let mut max_row: usize = 0;
    let mut max_col: usize = 0;
    let mut start: Coord = Coord { row: 0, col: 0 };
    let mut end: Coord = Coord { row: 0, col: 0 };
    for (row, line) in reader.enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => (),
                '.' => {
                    if row == 1 {
                        start = Coord { row, col };
                    } else {
                        end = Coord { row, col };
                    }
                }
                '^' => {
                    blizzards_north.insert(Coord { row, col });
                }
                '>' => {
                    blizzards_east.insert(Coord { row, col });
                }
                'v' => {
                    blizzards_south.insert(Coord { row, col });
                }
                '<' => {
                    blizzards_west.insert(Coord { row, col });
                }
                _ => {
                    panic!("unexepcted char {}", c);
                }
            }
            max_col = col as usize;
        }
        max_row = row as usize;
    }

    let bounding_box = BoundingBox::new(1, max_row - 1, 1, max_col - 1);

    BlizzardMap::new(
        start,
        end,
        bounding_box,
        blizzards_north,
        blizzards_east,
        blizzards_south,
        blizzards_west,
    )
}

fn part_1(reader: AocBufReader) {
    let mut blizzard_map = parse_input(reader);
}

fn main() {
    part_1(AocBufReader::from_string("inputs/example.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_input() {
        parse_input(AocBufReader::from_string("inputs/example.txt"));
        parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    }
}
