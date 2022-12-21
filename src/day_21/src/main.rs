use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref VALUE_REGEX: Regex = Regex::new(r"^([^:]*): ([0-9]*)$").unwrap();
    static ref OPERATION_REGEX: Regex =
        Regex::new(r"^([^:]*): ([^ ]*) ([\+\-\*/]) ([^ ]*)$").unwrap();
}

enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

struct Operation {
    operation_type: OperationType,
    argument_1: String,
    argument_2: String,
}

enum MonkeyJob {
    DoOperation(Operation),
    Value(isize),
}

struct MonkeyJobs {
    jobs: HashMap<String, MonkeyJob>,
    _cache: HashMap<String, isize>,
}

impl MonkeyJobs {
    fn new(jobs: HashMap<String, MonkeyJob>) -> MonkeyJobs {
        MonkeyJobs {
            jobs,
            _cache: HashMap::new(),
        }
    }

    fn get_value(&mut self, monkey_id: String) -> isize {
        10
    }
}

fn part_1(mut monkey_jobs: MonkeyJobs) -> isize {
    monkey_jobs.get_value("root".to_string())
}

fn parse_input(reader: AocBufReader) -> MonkeyJobs {
    let mut jobs: HashMap<String, MonkeyJob> = HashMap::new();
    for line in reader {
        if let Some(captures) = VALUE_REGEX.captures(&line) {
            let monkey_id = captures.get(1).unwrap().as_str().to_string();
            let value = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
            jobs.insert(monkey_id, MonkeyJob::Value(value));
        } else {
            let captures = OPERATION_REGEX.captures(&line).unwrap();
            let monkey_id = captures.get(1).unwrap().as_str().to_string();
            let argument_1 = captures.get(2).unwrap().as_str().to_string();
            let argument_2 = captures.get(4).unwrap().as_str().to_string();

            let operation: Operation = match captures.get(3).unwrap().as_str() {
                "+" => Operation {
                    operation_type: OperationType::Addition,
                    argument_1,
                    argument_2,
                },
                "-" => Operation {
                    operation_type: OperationType::Subtraction,
                    argument_1,
                    argument_2,
                },
                "*" => Operation {
                    operation_type: OperationType::Multiplication,
                    argument_1,
                    argument_2,
                },
                "/" => Operation {
                    operation_type: OperationType::Division,
                    argument_1,
                    argument_2,
                },
                _ => panic!("Unknown operation type"),
            };
            jobs.insert(monkey_id, MonkeyJob::DoOperation(operation));
        }
    }

    MonkeyJobs::new(jobs)
}

fn main() {
    let monkey_jobs = parse_input(AocBufReader::from_string("inputs/example.txt"));
    println!("{}", part_1(monkey_jobs));
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
