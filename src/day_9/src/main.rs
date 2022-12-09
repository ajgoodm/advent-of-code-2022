use std::cmp;
use std::collections::HashSet;

use shared::input::AocBufReader;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn to_vector(&self) -> Vector {
        match self {
            Direction::UP => Vector { drow: 1, dcol: 0 },
            Direction::RIGHT => Vector { drow: 0, dcol: 1 },
            Direction::DOWN => Vector { drow: -1, dcol: 0 },
            Direction::LEFT => Vector { drow: 0, dcol: -1 },
        }
    }
}

struct Instruction {
    direction: Direction,
    n_moves: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn add_vector(&self, vector: &Vector) -> Coord {
        Coord {
            row: self.row + vector.drow,
            col: self.col + vector.dcol,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Vector {
    drow: isize,
    dcol: isize,
}

impl Vector {
    fn from_coords(original_location: Coord, new_location: Coord) -> Vector {
        Vector {
            drow: new_location.row - original_location.row,
            dcol: new_location.col - original_location.col,
        }
    }
}

struct Rope {
    knots: Vec<Coord>,
}

impl Rope {
    fn new(len: usize) -> Rope {
        let mut knots: Vec<Coord> = Vec::new();
        for _ in 0..len {
            knots.push(Coord { row: 0, col: 0 });
        }
        Rope { knots }
    }

    fn _move(&mut self, direction: Direction) {
        let knot_move: Vector = direction.to_vector();
        self.knots[0] = self.knots[0].add_vector(&knot_move);

        for knot_idx in 1..self.length() {
            let prev_knot_displacement =
                Vector::from_coords(self.knots[knot_idx - 1], self.knots[knot_idx]);
            if cmp::max(
                prev_knot_displacement.drow.abs(),
                prev_knot_displacement.dcol.abs(),
            ) > 1
            {
                if prev_knot_displacement.drow > 0 {
                    self.knots[knot_idx].row -= 1;
                } else if prev_knot_displacement.drow < 0 {
                    self.knots[knot_idx].row += 1;
                }

                if prev_knot_displacement.dcol > 0 {
                    self.knots[knot_idx].col -= 1;
                } else if prev_knot_displacement.dcol < 0 {
                    self.knots[knot_idx].col += 1;
                }
            }
        }
    }

    fn length(&self) -> usize {
        self.knots.len()
    }

    fn tail_position(&self) -> Coord {
        self.knots[self.length() - 1]
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> HashSet<Coord> {
        let mut visited_spaces: HashSet<Coord> = HashSet::new();
        for _ in 0..instruction.n_moves {
            self._move(instruction.direction);
            visited_spaces.insert(self.tail_position());
        }
        visited_spaces
    }
}

fn parse_input(reader: AocBufReader) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for row in reader {
        let mut direction_n_moves = row.split(' ');
        let direction = match direction_n_moves.next().unwrap() {
            "U" => Direction::UP,
            "R" => Direction::RIGHT,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            _ => panic!("Unexpected direction!"),
        };

        let n_moves: usize = direction_n_moves.next().unwrap().parse::<usize>().unwrap();
        instructions.push(Instruction { direction, n_moves });
    }

    instructions
}

fn simulate_rope(reader: AocBufReader, rope_length: usize) -> usize {
    let instructions = parse_input(reader);

    let mut rope = Rope::new(rope_length);
    let mut visited_coords: HashSet<Coord> = HashSet::new();
    for instruction in instructions {
        let newly_visited_coords = rope.execute_instruction(instruction);
        visited_coords.extend(newly_visited_coords);
    }
    visited_coords.len()
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("{}", simulate_rope(reader, 2));
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("{}", simulate_rope(reader, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let reader = AocBufReader::from_string("inputs/example_1.txt");
        let instructions = parse_input(reader);

        let mut rope = Rope::new(2);
        let mut visited_coords: HashSet<Coord> = HashSet::new();
        for instruction in instructions {
            let newly_visited_coords = rope.execute_instruction(instruction);
            visited_coords.extend(newly_visited_coords);
        }
        println!("{}", visited_coords.len());
    }
}
