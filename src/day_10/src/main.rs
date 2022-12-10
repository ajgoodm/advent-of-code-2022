use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref ADDX_RE: Regex = Regex::new(r"^addx ([\-0-9]*)$").unwrap();
    static ref NOOP_RE: Regex = Regex::new(r"^noop$").unwrap();
    static ref PART_1_MODULUS: usize = 40;
}

enum InstructionType {
    ADDX,
    NOOP,
}

struct Instruction {
    instruction_type: InstructionType,
    val: isize,
    progress_counter: usize,
}

impl Instruction {
    fn new(instruction_type: InstructionType, val: isize) -> Instruction {
        Instruction {
            instruction_type,
            val,
            progress_counter: 0,
        }
    }

    fn execute(&mut self) -> Option<isize> {
        match self.instruction_type {
            InstructionType::NOOP => Some(0),
            InstructionType::ADDX => {
                if self.progress_counter == 1 {
                    Some(self.val)
                } else {
                    self.progress_counter += 1;
                    None
                }
            }
        }
    }
}

struct Crt {
    pixels: Vec<Vec<bool>>,
    nrows: usize,
    ncols: usize,
}

impl Crt {
    fn new(nrows: usize, ncols: usize) -> Crt {
        let pixels = (0..nrows)
            .into_iter()
            .map(|_| (0..ncols).into_iter().map(|_| false).collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();
        Crt {
            pixels,
            nrows,
            ncols,
        }
    }

    fn light_pixel(&mut self, row_idx: usize, col_idx: usize) {
        self.pixels[row_idx][col_idx] = true;
    }

    /// Draw the screen to stdout
    fn draw(&self) {
        for row in self.pixels.iter() {
            println!(
                "{}",
                row.iter()
                    .map(|x| {
                        match x {
                            true => '#',
                            false => '.',
                        }
                    })
                    .collect::<String>()
            );
        }
    }
}

fn parse_input(reader: AocBufReader) -> Vec<Instruction> {
    reader
        .map(|line| {
            if let Some(capture) = ADDX_RE.captures(&line) {
                Instruction::new(
                    InstructionType::ADDX,
                    capture.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                )
            } else if let Some(_) = NOOP_RE.captures(&line) {
                Instruction::new(InstructionType::NOOP, 0)
            } else {
                panic!("Could not parse instruction: {}", &line);
            }
        })
        .collect()
}

fn part_1(instructions: Vec<Instruction>) -> isize {
    let mut instructions_iter = instructions.into_iter();
    // the first instruction
    let mut instruction = instructions_iter.next().unwrap();
    let mut next_instruction: Option<Instruction>;

    let mut x_register_val: isize = 1;
    let mut cycle_number: usize = 1;
    let mut signal_strength_sum: isize = 0;
    loop {
        if cycle_number % *PART_1_MODULUS == 20 {
            signal_strength_sum += cycle_number as isize * x_register_val
        }

        if let Some(addend) = instruction.execute() {
            x_register_val += addend;
            next_instruction = instructions_iter.next();
            match next_instruction {
                None => {
                    break;
                }
                Some(x) => {
                    instruction = x;
                }
            }
        }
        cycle_number += 1;
    }
    signal_strength_sum
}

fn part_2(instructions: Vec<Instruction>, mut crt: Crt) {
    let mut instructions_iter = instructions.into_iter();
    // the first instruction
    let mut instruction = instructions_iter.next().unwrap();
    let mut next_instruction: Option<Instruction>;

    let mut x_register_val: isize = 1;
    let mut cycle_number: usize = 1;
    loop {
        let row_idx: usize = cycle_number / crt.ncols;
        let col_idx: usize = (cycle_number - 1) % crt.ncols;

        let signed_col_idx: isize = col_idx.try_into().unwrap();
        if signed_col_idx >= x_register_val - 1 && signed_col_idx <= x_register_val + 1 {
            crt.light_pixel(row_idx, col_idx);
        }

        if let Some(addend) = instruction.execute() {
            x_register_val += addend;
            next_instruction = instructions_iter.next();
            match next_instruction {
                None => {
                    break;
                }
                Some(x) => {
                    instruction = x;
                }
            }
        }
        cycle_number += 1;
    }
    crt.draw();
}

fn main() {
    println!(
        "{}",
        part_1(parse_input(AocBufReader::from_string("inputs/part_1.txt")))
    );
    let crt = Crt::new(6, 40);
    part_2(
        parse_input(AocBufReader::from_string("inputs/part_1.txt")),
        crt,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let reader = AocBufReader::from_string("inputs/example.txt");
        let instructions = parse_input(reader);
        assert_eq!(part_1(instructions), 13140);
    }

    #[test]
    fn test_crt() {
        let crt = Crt::new(6, 40);
        crt.draw();
    }

    #[test]
    fn test_example_part_2() {
        let reader = AocBufReader::from_string("inputs/example.txt");
        let instructions = parse_input(reader);
        let crt = Crt::new(6, 40);
        part_2(instructions, crt);
    }
}
