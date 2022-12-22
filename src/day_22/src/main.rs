use std::collections::HashSet;

use shared::input::AocBufReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn up(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }
}

struct Board {
    open_tiles: HashSet<Coord>,
    solid_walls: HashSet<Coord>,
}

impl Board {
    fn wrap(&self, position: &Coord, orientation: &Orientation) -> Coord {
        match orientation {
            Orientation::Up => {
                let col = position.col;
                // wrap to the bottom (largest number)
                let row = self
                    .open_tiles
                    .union(&self.solid_walls)
                    .cloned()
                    .collect::<HashSet<Coord>>()
                    .iter()
                    .filter(|coord| coord.col == col)
                    .map(|coord| coord.row)
                    .max()
                    .unwrap();
                Coord { row, col }
            }
            Orientation::Down => {
                let col = position.col;
                // wrap to the top (smallest number)
                let row = self
                    .open_tiles
                    .union(&self.solid_walls)
                    .cloned()
                    .collect::<HashSet<Coord>>()
                    .iter()
                    .filter(|coord| coord.col == col)
                    .map(|coord| coord.row)
                    .min()
                    .unwrap();
                Coord { row, col }
            }
            Orientation::Left => {
                let row = position.row;
                // wrap to the right (largest number)
                let col = self
                    .open_tiles
                    .union(&self.solid_walls)
                    .cloned()
                    .collect::<HashSet<Coord>>()
                    .iter()
                    .filter(|coord| coord.row == row)
                    .map(|coord| coord.col)
                    .max()
                    .unwrap();
                Coord { row, col }
            }
            Orientation::Right => {
                let row = position.row;
                // wrap to the left (smallest number)
                let col = self
                    .open_tiles
                    .union(&self.solid_walls)
                    .cloned()
                    .collect::<HashSet<Coord>>()
                    .iter()
                    .filter(|coord| coord.row == row)
                    .map(|coord| coord.col)
                    .min()
                    .unwrap();
                Coord { row, col }
            }
        }
    }
}

enum RightLeft {
    Right,
    Left,
}

enum Instruction {
    Rotate(RightLeft),
    Move(usize),
}

enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn right(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn left(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
            Orientation::Right => Orientation::Up,
        }
    }

    fn facing(&self) -> isize {
        match self {
            Orientation::Up => 3,
            Orientation::Left => 2,
            Orientation::Down => 1,
            Orientation::Right => 0,
        }
    }
}

struct Mover {
    position: Coord,
    orientation: Orientation,
}

impl Mover {
    fn new(board: &Board) -> Mover {
        let top_row = 1;
        let left_most_column = board
            .open_tiles
            .iter()
            .filter(|coord| coord.row == top_row)
            .map(|coord| coord.col)
            .min()
            .unwrap();
        let position = Coord {
            row: top_row,
            col: left_most_column,
        };
        Mover {
            position,
            orientation: Orientation::Right,
        }
    }

    fn _rotate(&mut self, right_left: RightLeft) {
        match right_left {
            RightLeft::Right => self.orientation = self.orientation.right(),
            RightLeft::Left => self.orientation = self.orientation.left(),
        }
    }

    fn _move(&mut self, n_moves: usize, board: &Board) {
        for _ in 0..n_moves {
            let mut next = match self.orientation {
                Orientation::Up => self.position.up(),
                Orientation::Right => self.position.right(),
                Orientation::Down => self.position.down(),
                Orientation::Left => self.position.left(),
            };

            if !board.open_tiles.contains(&next) && !board.solid_walls.contains(&next) {
                // this space isn't on the map!
                next = board.wrap(&self.position, &self.orientation);
            }

            if board.solid_walls.contains(&next) {
                // we ran into a wall!
                break;
            }

            self.position = next;
        }
    }

    fn execute_instruction(&mut self, board: &Board, instruction: Instruction) {
        match instruction {
            Instruction::Rotate(right_left) => self._rotate(right_left),
            Instruction::Move(n_moves) => self._move(n_moves, &board),
        }
    }
}

fn parse_instructions(line: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut chars = line.chars();
    let mut number_string: String = String::new();
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            number_string.push(c);
        } else {
            if number_string.len() > 0 {
                instructions.push(Instruction::Move(number_string.parse::<usize>().unwrap()));
                number_string = String::new();
            }
            match c {
                'R' => {
                    instructions.push(Instruction::Rotate(RightLeft::Right));
                }
                'L' => {
                    instructions.push(Instruction::Rotate(RightLeft::Left));
                }
                _ => {
                    panic!("unknown instruction character");
                }
            }
        }
    }
    if number_string.len() > 0 {
        instructions.push(Instruction::Move(number_string.parse::<usize>().unwrap()));
    }

    instructions
}

fn parse_input(mut reader: AocBufReader) -> (Board, Vec<Instruction>) {
    let mut open_tiles: HashSet<Coord> = HashSet::new();
    let mut solid_walls: HashSet<Coord> = HashSet::new();
    let mut row_idx: usize = 0;
    loop {
        let line = reader.next().unwrap();
        row_idx += 1; // rows and columns are 1-indexed

        if line == "".to_string() {
            break;
        }

        for (col_minus_1, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    open_tiles.insert(Coord {
                        row: row_idx as isize,
                        col: (col_minus_1 + 1) as isize,
                    });
                }
                '#' => {
                    solid_walls.insert(Coord {
                        row: row_idx as isize,
                        col: (col_minus_1 + 1) as isize,
                    });
                }
                ' ' => (),
                _ => {
                    panic!("unknown character building map {}", c);
                }
            }
        }
    }
    let board = Board {
        open_tiles,
        solid_walls,
    };
    let instructions = parse_instructions(reader.next().unwrap());

    (board, instructions)
}

fn part_1(reader: AocBufReader) -> isize {
    let (board, instructions) = parse_input(reader);
    let mut mover = Mover::new(&board);
    for instruction in instructions {
        mover.execute_instruction(&board, instruction);
    }

    let final_row = mover.position.row;
    let final_col = mover.position.col;
    let facing = mover.orientation.facing();

    println!("row: {}, col: {}, facing: {}", final_row, final_col, facing);

    1_000 * final_row + 4 * final_col + facing
}

fn main() {
    println!("{}", part_1(AocBufReader::from_string("inputs/part_1.txt")));
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
