use std::cmp;
use std::collections::HashSet;

use shared::input::AocBufReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn down(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn down_left(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col - 1,
        }
    }

    fn down_right(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col + 1,
        }
    }
}

struct Cave {
    rock_coords: HashSet<Coord>,
    sand_coords: HashSet<Coord>,
    max_rock_depth: isize,
}

impl Cave {
    fn add_sand(&mut self, sand_source: &Coord) -> bool {
        let mut sand_tile_location: Coord = sand_source.clone();
        loop {
            if !self.rock_coords.contains(&sand_tile_location.down())
                && !self.sand_coords.contains(&sand_tile_location.down())
            {
                sand_tile_location = sand_tile_location.down();
            } else if !self.rock_coords.contains(&sand_tile_location.down_left())
                && !self.sand_coords.contains(&sand_tile_location.down_left())
            {
                sand_tile_location = sand_tile_location.down_left();
            } else if !self.rock_coords.contains(&sand_tile_location.down_right())
                && !self.sand_coords.contains(&sand_tile_location.down_right())
            {
                sand_tile_location = sand_tile_location.down_right();
            } else {
                // sand is at rest, cave is not yet full
                self.sand_coords.insert(sand_tile_location);
                return false;
            }
            if sand_tile_location.row >= self.max_rock_depth {
                return true;
            }
        }
    }

    fn is_floor(&self, coord: &Coord) -> bool {
        coord.row == self.max_rock_depth + 2
    }

    fn add_sand_part_2(&mut self, sand_source: &Coord) -> bool {
        let mut sand_tile_location: Coord = sand_source.clone();
        loop {
            if !self.rock_coords.contains(&sand_tile_location.down())
                && !self.sand_coords.contains(&sand_tile_location.down())
                && !self.is_floor(&sand_tile_location.down())
            {
                sand_tile_location = sand_tile_location.down();
            } else if !self.rock_coords.contains(&sand_tile_location.down_left())
                && !self.sand_coords.contains(&sand_tile_location.down_left())
                && !self.is_floor(&sand_tile_location.down_left())
            {
                sand_tile_location = sand_tile_location.down_left();
            } else if !self.rock_coords.contains(&sand_tile_location.down_right())
                && !self.sand_coords.contains(&sand_tile_location.down_right())
                && !self.is_floor(&sand_tile_location.down_right())
            {
                sand_tile_location = sand_tile_location.down_right();
            } else {
                // sand is at rest
                if sand_tile_location.row == sand_source.row {
                    return true;
                } else {
                    self.sand_coords.insert(sand_tile_location);
                    return false;
                };
            }
        }
    }
}

fn _line_to_coords(line: String) -> Vec<Coord> {
    line.split(" -> ")
        .map(|str| {
            let mut col_row = str.split(",");
            let col = col_row.next().unwrap().parse::<isize>().unwrap();
            let row = col_row.next().unwrap().parse::<isize>().unwrap();
            Coord { row, col }
        })
        .collect()
}

fn _interpolate(start: &Coord, end: &Coord) -> HashSet<Coord> {
    if start.row == end.row {
        (cmp::min(start.col, end.col)..=cmp::max(start.col, end.col))
            .map(|col| Coord {
                row: start.row,
                col,
            })
            .collect::<HashSet<Coord>>()
    } else if start.col == end.col {
        (cmp::min(start.row, end.row)..=cmp::max(start.row, end.row))
            .map(|row| Coord {
                row,
                col: start.col,
            })
            .collect::<HashSet<Coord>>()
    } else {
        panic!("WE DON'T DO DIAGONALS!");
    }
}

fn parse_line(line: String) -> HashSet<Coord> {
    let mut rock_coords: HashSet<Coord> = HashSet::new();
    let vertex_coords = _line_to_coords(line);
    for idx in 0..(vertex_coords.len() - 1) {
        rock_coords.extend(_interpolate(&vertex_coords[idx], &vertex_coords[idx + 1]));
    }

    rock_coords
}

fn parse_input(mut reader: AocBufReader) -> Cave {
    let mut rock_coords: HashSet<Coord> = HashSet::new();
    while let Some(line) = reader.next() {
        rock_coords.extend(parse_line(line));
    }
    let max_rock_depth = rock_coords.iter().map(|coord| coord.row).max().unwrap();

    Cave {
        rock_coords,
        sand_coords: HashSet::new(),
        max_rock_depth,
    }
}

fn part_1(mut cave: Cave) -> usize {
    let sand_source = Coord { row: 0, col: 500 };
    let mut n_sand_tiles: usize = 0;

    let mut cave_is_full = false;
    while !cave_is_full {
        cave_is_full = cave.add_sand(&sand_source);
        if !cave_is_full {
            n_sand_tiles += 1;
        }
    }

    n_sand_tiles
}

fn part_2(mut cave: Cave) -> usize {
    let sand_source = Coord { row: 0, col: 500 };
    let mut n_sand_tiles: usize = 0;

    let mut cave_is_full = false;
    while !cave_is_full {
        cave_is_full = cave.add_sand_part_2(&sand_source);
        n_sand_tiles += 1;
    }

    n_sand_tiles
}

fn main() {
    let cave = parse_input(AocBufReader::from_string("inputs/example.txt"));
    println!("{}", part_1(cave));

    let cave = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_2(cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        parse_line("498,4 -> 498,6 -> 496,6".to_string());
    }
}
