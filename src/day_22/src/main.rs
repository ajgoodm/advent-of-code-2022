use std::collections::HashSet;

use shared::input::AocBufReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

struct Board {
    open_tiles: HashSet<Coord>,
    solid_walls: HashSet<Coord>,
}

enum RightLeft {
    Right,
    Left,
}

enum Instruction {
    Rotate(RightLeft),
    Move(usize),
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

fn main() {
    println!("Hello, world!");
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
