use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref STARTING_ITEMS_RE: Regex = Regex::new(r"Starting items: ([0-9, ]*)$").unwrap();
    static ref OPERATION_RE: Regex =
        Regex::new(r"Operation: new = old ([\*\+]) ([0-9]*)$").unwrap();
    static ref TEST_RE: Regex = Regex::new(r"Test: divisible by ([0-9]*)$").unwrap();
    static ref TRUE_CASE_RE: Regex = Regex::new(r"If true: throw to monkey ([0-9]*)$").unwrap();
    static ref FALSE_CASE_RE: Regex = Regex::new(r"If false: throw to monkey ([0-9]*)$").unwrap();
}

enum OperationType {
    ADDITION,
    MULTIPLICATION,
    SQUARING,
}

struct Operation {
    operation_type: OperationType,
    operand: usize,
}

struct Test {
    divisor_to_check: usize,
    true_case_destination: usize,
    false_case_destination: usize,
}

struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    items_inspected_counter: usize,
}

fn parse_input(mut reader: AocBufReader) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    loop {
        // "Monkey i:"
        reader.next().unwrap();
        let items: VecDeque<usize> = {
            STARTING_ITEMS_RE
                .captures(&reader.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|xx| xx.parse::<usize>().unwrap())
                .collect()
        };
        let operation: Operation = {
            let line = reader.next().unwrap();
            if let Some(capture) = OPERATION_RE.captures(&line) {
                let operation_type = match capture.get(1).unwrap().as_str() {
                    "*" => OperationType::MULTIPLICATION,
                    "+" => OperationType::ADDITION,
                    _ => panic!("Unknown operation type"),
                };
                let operand = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                Operation {
                    operation_type,
                    operand,
                }
            } else {
                assert_eq!(&line, "  Operation: new = old * old");
                Operation {
                    operation_type: OperationType::SQUARING,
                    operand: 0,
                }
            }
        };
        let test: Test = {
            let divisor_to_check: usize = TEST_RE
                .captures(&reader.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let true_case_destination: usize = TRUE_CASE_RE
                .captures(&reader.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let false_case_destination: usize = FALSE_CASE_RE
                .captures(&reader.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            Test {
                divisor_to_check,
                true_case_destination,
                false_case_destination,
            }
        };
        monkeys.push(Monkey {
            items,
            operation,
            test,
            items_inspected_counter: 0,
        });

        if reader.next() == None {
            break;
        }
    }

    monkeys
}

fn _monkey_around_part_1(monkeys: &mut Vec<Monkey>) {
    for src_idx in 0..monkeys.len() {
        let mut dest_indx: usize;
        while let Some(item) = monkeys[src_idx].items.pop_front() {
            monkeys[src_idx].items_inspected_counter += 1;
            let item_to_toss: usize = match monkeys[src_idx].operation.operation_type {
                OperationType::ADDITION => (item + monkeys[src_idx].operation.operand) / 3,
                OperationType::MULTIPLICATION => item * monkeys[src_idx].operation.operand / 3,
                OperationType::SQUARING => item * item / 3,
            };
            if item_to_toss % monkeys[src_idx].test.divisor_to_check == 0 {
                dest_indx = monkeys[src_idx].test.true_case_destination;
            } else {
                dest_indx = monkeys[src_idx].test.false_case_destination;
            }
            monkeys[dest_indx].items.push_back(item_to_toss);
        }
    }
}

/// By capping things at _a_ common multiple of all of the monkeys test
/// divisors, we can preserve the test behavior.
fn _monkey_around_part_2(monkeys: &mut Vec<Monkey>, common_multiple: usize) {
    for src_idx in 0..monkeys.len() {
        let mut dest_indx: usize;
        while let Some(item) = monkeys[src_idx].items.pop_front() {
            monkeys[src_idx].items_inspected_counter += 1;
            let item_to_toss: usize = match monkeys[src_idx].operation.operation_type {
                OperationType::ADDITION => item + monkeys[src_idx].operation.operand,
                OperationType::MULTIPLICATION => item * monkeys[src_idx].operation.operand,
                OperationType::SQUARING => item * item,
            } % common_multiple;
            if item_to_toss % monkeys[src_idx].test.divisor_to_check == 0 {
                dest_indx = monkeys[src_idx].test.true_case_destination;
            } else {
                dest_indx = monkeys[src_idx].test.false_case_destination;
            }
            monkeys[dest_indx].items.push_back(item_to_toss);
        }
    }
}

fn part_1(mut monkeys: Vec<Monkey>, n_rounds: usize) -> usize {
    for _ in 0..n_rounds {
        _monkey_around_part_1(&mut monkeys);
    }
    monkeys.sort_by(|a, b| b.items_inspected_counter.cmp(&a.items_inspected_counter));
    monkeys[0].items_inspected_counter * monkeys[1].items_inspected_counter
}

fn part_2(mut monkeys: Vec<Monkey>, n_rounds: usize) -> usize {
    let common_multiple = monkeys.iter().map(|m| m.test.divisor_to_check).product();
    for _ in 0..n_rounds {
        _monkey_around_part_2(&mut monkeys, common_multiple);
    }
    monkeys.sort_by(|a, b| b.items_inspected_counter.cmp(&a.items_inspected_counter));
    monkeys[0].items_inspected_counter * monkeys[1].items_inspected_counter
}

fn main() {
    println!(
        "{}",
        part_1(
            parse_input(AocBufReader::from_string("inputs/part_1.txt")),
            20
        )
    );
    println!(
        "{}",
        part_2(
            parse_input(AocBufReader::from_string("inputs/part_1.txt")),
            10000
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let monkeys = parse_input(AocBufReader::from_string("inputs/example.txt"));
        let monkeys = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    }

    #[test]
    fn test_part_1_example() {
        let mut monkeys = parse_input(AocBufReader::from_string("inputs/example.txt"));
        assert_eq!(part_1(monkeys, 20), 10605);
    }
}
