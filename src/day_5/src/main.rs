use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"^move ([0-9]*) from ([0-9]*) to ([0-9]*)$").unwrap();
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    source_stack_idx: usize,
    destination_stack_idx: usize,
    n_boxes: usize,
}

fn parse_stack_line(line: String) -> Vec<Option<char>> {
    let mut boxes: Vec<Option<char>> = Vec::new();
    let mut line_chars = line.chars();

    let mut central_char;
    loop {
        line_chars.next().unwrap();
        central_char = line_chars.next().unwrap();
        line_chars.next().unwrap();

        if central_char == ' ' {
            boxes.push(None);
        } else {
            boxes.push(Some(central_char));
        }

        if line_chars.next().is_none() {
            break;
        }
    }
    boxes
}

fn parse_instruction(line: String) -> Instruction {
    let captures = INSTRUCTION_REGEX.captures(&line).unwrap();
    Instruction {
        source_stack_idx: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
        destination_stack_idx: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        n_boxes: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
    }
}

fn parse_input(mut reader: AocBufReader) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let empty_string = "".to_string();
    let mut line: String;

    let mut parsed_boxes: Vec<Vec<Option<char>>> = Vec::new();
    loop {
        line = reader.next().unwrap();
        if line == empty_string {
            break;
        }
        parsed_boxes.push(parse_stack_line(line));
    }

    let n_stacks: usize = parsed_boxes[0].len();
    let mut stacks: Vec<Vec<char>> = (0..n_stacks).map(|_| Vec::new()).collect();

    let highest_stack_height: usize = parsed_boxes.len() - 1;
    for row in parsed_boxes[..highest_stack_height].iter().rev() {
        for (stack_idx, box_) in row.iter().enumerate() {
            if let Some(char_) = box_ {
                stacks[stack_idx].push(*char_);
            }
        }
    }

    let mut instructions: Vec<Instruction> = vec![];
    while let Some(line) = reader.next() {
        instructions.push(parse_instruction(line));
    }

    (stacks, instructions)
}

fn _part_1_execute_instruction(stacks: &mut Vec<Vec<char>>, instruction: Instruction) {
    for iter in 0..instruction.n_boxes {
        if let Some(char_) = stacks[instruction.source_stack_idx].pop() {
            stacks[instruction.destination_stack_idx].push(char_);
        }
    }
}

fn _part_2_execute_instruction(stacks: &mut Vec<Vec<char>>, instruction: Instruction) {
    let source_stack_height = stacks[instruction.source_stack_idx].len();
    let bottom_box_idx = source_stack_height - instruction.n_boxes;

    let mut boxes_to_move: Vec<char> =
        stacks[instruction.source_stack_idx].split_off(bottom_box_idx);
    stacks[instruction.destination_stack_idx].append(&mut boxes_to_move);
}

fn part_1(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions {
        _part_1_execute_instruction(&mut stacks, instruction);
    }

    stacks
        .iter()
        .map(|stack| {
            let height = stack.len();
            stack[height - 1]
        })
        .collect::<String>()
}

fn part_2(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions {
        _part_2_execute_instruction(&mut stacks, instruction);
    }

    stacks
        .iter()
        .map(|stack| {
            let height = stack.len();
            stack[height - 1]
        })
        .collect::<String>()
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let (stacks, instructions): (Vec<Vec<char>>, Vec<Instruction>) = parse_input(reader);
    println!("{}", part_1(stacks, instructions));

    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let (stacks, instructions): (Vec<Vec<char>>, Vec<Instruction>) = parse_input(reader);
    println!("{}", part_2(stacks, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stack_line() {
        assert_eq!(
            parse_stack_line("[C]     [P]".to_string()),
            vec![Some('C'), None, Some('P')]
        );
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("move 22 from 1 to 8".to_string()),
            Instruction {
                source_stack_idx: 1,
                destination_stack_idx: 8,
                n_boxes: 22,
            }
        )
    }
}
