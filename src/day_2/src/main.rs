use std::collections::HashMap;

use lazy_static::lazy_static;

use shared::input::AocBufReader;

lazy_static! {
    static ref PART_1_MAPPING: HashMap<(char, char), usize> = {
        let mut m: HashMap<(char, char), usize> = HashMap::new();
        m.insert(('A', 'X'), 1 + 3);
        m.insert(('A', 'Y'), 2 + 6);
        m.insert(('A', 'Z'), 3 + 0);
        m.insert(('B', 'X'), 1 + 0);
        m.insert(('B', 'Y'), 2 + 3);
        m.insert(('B', 'Z'), 3 + 6);
        m.insert(('C', 'X'), 1 + 6);
        m.insert(('C', 'Y'), 2 + 0);
        m.insert(('C', 'Z'), 3 + 3);
        m
    };
    static ref PART_2_MAPPING: HashMap<(char, char), usize> = {
        let mut m: HashMap<(char, char), usize> = HashMap::new();
        m.insert(('A', 'X'), 3 + 0);
        m.insert(('A', 'Y'), 1 + 3);
        m.insert(('A', 'Z'), 2 + 6);
        m.insert(('B', 'X'), 1 + 0);
        m.insert(('B', 'Y'), 2 + 3);
        m.insert(('B', 'Z'), 3 + 6);
        m.insert(('C', 'X'), 2 + 0);
        m.insert(('C', 'Y'), 3 + 3);
        m.insert(('C', 'Z'), 1 + 6);
        m
    };
}

fn parse_to_tuples(reader: AocBufReader) -> Vec<(char, char)> {
    let mut input: Vec<(char, char)> = Vec::new();
    for line in reader {
        let mut chars = line.split(' ');
        let char_1: char = chars.next().unwrap().chars().next().unwrap();
        let char_2: char = chars.next().unwrap().chars().next().unwrap();
        input.push((char_1, char_2));
    }
    input
}

fn part_1(tuples: &Vec<(char, char)>) -> usize {
    tuples
        .iter()
        .map(|tuple| *PART_1_MAPPING.get(tuple).unwrap())
        .sum()
}

fn part_2(tuples: &Vec<(char, char)>) -> usize {
    tuples
        .iter()
        .map(|tuple| *PART_2_MAPPING.get(tuple).unwrap())
        .sum()
}

fn main() {
    let tuples = parse_to_tuples(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_1(&tuples));
    println!("{}", part_2(&tuples));
}
