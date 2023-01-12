use std::collections::{HashMap, HashSet};

use shared::input::AocBufReader;

#[derive(Clone)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn north(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn north_east(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col + 1,
        }
    }

    fn east(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn south_east(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col + 1,
        }
    }

    fn south(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn south_west(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col - 1,
        }
    }

    fn west(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn north_west(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col - 1,
        }
    }

    fn neighbors_north(&self) -> HashSet<Coord> {
        vec![self.north_west(), self.north(), self.north_east()]
            .into_iter()
            .collect()
    }

    fn neighbors_east(&self) -> HashSet<Coord> {
        vec![self.north_east(), self.east(), self.south_east()]
            .into_iter()
            .collect()
    }

    fn neighbors_south(&self) -> HashSet<Coord> {
        vec![self.south_east(), self.south(), self.south_west()]
            .into_iter()
            .collect()
    }

    fn neighbors_west(&self) -> HashSet<Coord> {
        vec![self.south_west(), self.west(), self.north_west()]
            .into_iter()
            .collect()
    }
}

struct Elf {
    position: Coord,
    proposed_move: Coord,
}

impl Elf {
    fn new(row: isize, col: isize) -> Elf {
        Elf {
            position: Coord { row, col },
            proposed_move: Coord { row, col },
        }
    }
}

struct Elves {
    elves: Vec<Elf>,
    _direction: CardinalDirection,
}

impl Elves {
    fn new(elves: Vec<Elf>) -> Elves {
        Elves {
            elves,
            _direction: CardinalDirection::North,
        }
    }

    fn len(&self) -> usize {
        self.elves.len()
    }

    fn get_candidate_directions(&self) -> Vec<CardinalDirection> {
        match self._direction {
            CardinalDirection::North => {
                vec![
                    CardinalDirection::North,
                    CardinalDirection::South,
                    CardinalDirection::West,
                    CardinalDirection::East,
                ]
            }
            CardinalDirection::South => {
                vec![
                    CardinalDirection::South,
                    CardinalDirection::West,
                    CardinalDirection::East,
                    CardinalDirection::North,
                ]
            }
            CardinalDirection::West => {
                vec![
                    CardinalDirection::West,
                    CardinalDirection::East,
                    CardinalDirection::North,
                    CardinalDirection::South,
                ]
            }
            CardinalDirection::East => {
                vec![
                    CardinalDirection::East,
                    CardinalDirection::North,
                    CardinalDirection::South,
                    CardinalDirection::West,
                ]
            }
        }
    }

    fn cycle_direction(&mut self) {
        match self._direction {
            CardinalDirection::North => self._direction = CardinalDirection::South,
            CardinalDirection::South => self._direction = CardinalDirection::West,
            CardinalDirection::West => self._direction = CardinalDirection::East,
            CardinalDirection::East => self._direction = CardinalDirection::North,
        }
    }

    fn elf_positions(&self) -> HashSet<Coord> {
        self.elves.iter().map(|elf| elf.position.clone()).collect()
    }

    fn has_neighbors_north(position: &Coord, elf_positions: &HashSet<Coord>) -> bool {
        !position.neighbors_north().is_disjoint(elf_positions)
    }

    fn has_neighbors_east(position: &Coord, elf_positions: &HashSet<Coord>) -> bool {
        !position.neighbors_east().is_disjoint(elf_positions)
    }

    fn has_neighbors_south(position: &Coord, elf_positions: &HashSet<Coord>) -> bool {
        !position.neighbors_south().is_disjoint(elf_positions)
    }

    fn has_neighbors_west(position: &Coord, elf_positions: &HashSet<Coord>) -> bool {
        !position.neighbors_west().is_disjoint(elf_positions)
    }

    fn plan_moves(&mut self) {
        let candidate_directions = self.get_candidate_directions();
        self.cycle_direction();

        let elf_positions = self.elf_positions();
        for elf_idx in 0..self.len() {
            let elf_has_neighbors_north =
                Elves::has_neighbors_north(&self.elves[elf_idx].position, &elf_positions);
            let elf_has_neighbors_east =
                Elves::has_neighbors_east(&self.elves[elf_idx].position, &elf_positions);
            let elf_has_neighbors_south =
                Elves::has_neighbors_south(&self.elves[elf_idx].position, &elf_positions);
            let elf_has_neighbors_west =
                Elves::has_neighbors_west(&self.elves[elf_idx].position, &elf_positions);
            let elf_has_neighbors = elf_has_neighbors_north
                || elf_has_neighbors_east
                || elf_has_neighbors_south
                || elf_has_neighbors_west;

            let mut elf = &mut self.elves[elf_idx];
            if !elf_has_neighbors {
                elf.proposed_move = elf.position.clone();
            } else {
                for direction in candidate_directions.iter() {
                    match direction {
                        CardinalDirection::North => {
                            if !elf_has_neighbors_north {
                                elf.proposed_move = elf.position.north();
                                break;
                            }
                        }
                        CardinalDirection::East => {
                            if !elf_has_neighbors_east {
                                elf.proposed_move = elf.position.east();
                                break;
                            }
                        }
                        CardinalDirection::South => {
                            if !elf_has_neighbors_south {
                                elf.proposed_move = elf.position.south();
                                break;
                            }
                        }
                        CardinalDirection::West => {
                            if !elf_has_neighbors_west {
                                elf.proposed_move = elf.position.west();
                                break;
                            }
                        }
                    }
                    // If there are neighbors in every direction (each conditional evaluates to false),
                    // the elf doesn't move.
                    elf.proposed_move = elf.position.clone();
                }
            }
        }
    }

    fn execute_moves(&mut self) -> bool {
        let mut n_elves_planning_to_move_to_coord: HashMap<Coord, usize> = HashMap::new();
        for elf in self.elves.iter() {
            n_elves_planning_to_move_to_coord
                .entry(elf.proposed_move.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut elves_moved = false;
        for elf in self.elves.iter_mut() {
            if *n_elves_planning_to_move_to_coord
                .get(&elf.proposed_move)
                .unwrap()
                == 1
            {
                if elf.position != elf.proposed_move {
                    elves_moved = true;
                }
                elf.position = elf.proposed_move.clone();
            }
        }

        elves_moved
    }

    fn part_1_count(&self) -> isize {
        let elf_positions = self.elf_positions();

        let min_row = elf_positions.iter().map(|coord| coord.row).min().unwrap();
        let max_row = elf_positions.iter().map(|coord| coord.row).max().unwrap();
        let min_col = elf_positions.iter().map(|coord| coord.col).min().unwrap();
        let max_col = elf_positions.iter().map(|coord| coord.col).max().unwrap();

        (max_row - min_row + 1) * (max_col - min_col + 1) - elf_positions.len() as isize
    }
}

fn parse_input(reader: AocBufReader) -> Elves {
    let mut elves: Vec<Elf> = Vec::new();
    for (row_idx, row) in reader.enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            match c {
                '#' => elves.push(Elf::new(row_idx as isize, col_idx as isize)),
                '.' => (),
                _ => {
                    panic!("unexpected character")
                }
            }
        }
    }

    Elves::new(elves)
}

fn part_1(mut elves: Elves) -> isize {
    for _ in 0..10 {
        elves.plan_moves();
        elves.execute_moves();
    }

    elves.part_1_count()
}

fn part_2(mut elves: Elves) -> usize {
    let mut n_rounds: usize = 0;
    loop {
        n_rounds += 1;
        elves.plan_moves();
        let elves_moved = elves.execute_moves();
        if !elves_moved {
            break;
        }
    }

    n_rounds
}

fn main() {
    println!(
        "{}",
        part_1(parse_input(AocBufReader::from_string("inputs/part_1.txt")))
    );
    println!(
        "{}",
        part_2(parse_input(AocBufReader::from_string("inputs/part_1.txt")))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        parse_input(AocBufReader::from_string("inputs/example.txt"));
        parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    }
}
