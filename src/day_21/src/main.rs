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
}

struct LinearExpression {
    constant: isize,
    human_coefficient: isize,
}

impl MonkeyJobs {
    fn new(jobs: HashMap<String, MonkeyJob>) -> MonkeyJobs {
        MonkeyJobs { jobs }
    }

    fn get_value(&self, monkey_id: String) -> isize {
        let monkey_job = self.jobs.get(&monkey_id).unwrap();
        match monkey_job {
            MonkeyJob::DoOperation(operation) => {
                let argument_1 = self.get_value(operation.argument_1.clone());
                let argument_2 = self.get_value(operation.argument_2.clone());
                match operation.operation_type {
                    OperationType::Addition => argument_1 + argument_2,
                    OperationType::Subtraction => argument_1 - argument_2,
                    OperationType::Multiplication => argument_1 * argument_2,
                    OperationType::Division => argument_1 / argument_2,
                }
            }
            MonkeyJob::Value(return_value) => *return_value,
        }
    }

    fn test_humn(&self, monkey_id: String, test_value: isize) -> isize {
        if monkey_id == "humn".to_string() {
            return test_value;
        }

        let monkey_job = self.jobs.get(&monkey_id).unwrap();
        match monkey_job {
            MonkeyJob::DoOperation(operation) => {
                let argument_1 = self.test_humn(operation.argument_1.clone(), test_value);
                let argument_2 = self.test_humn(operation.argument_2.clone(), test_value);
                match operation.operation_type {
                    OperationType::Addition => argument_1 + argument_2,
                    OperationType::Subtraction => argument_1 - argument_2,
                    OperationType::Multiplication => argument_1 * argument_2,
                    OperationType::Division => argument_1 / argument_2,
                }
            }
            MonkeyJob::Value(return_value) => *return_value,
        }
    }
}

fn part_1(monkey_jobs: MonkeyJobs) -> isize {
    monkey_jobs.get_value("root".to_string())
}

fn part_2(monkey_jobs: MonkeyJobs) -> isize {
    let comparison_argument_1: String;
    let comparison_argument_2: String;
    match monkey_jobs.jobs.get("root").unwrap() {
        MonkeyJob::DoOperation(operation) => {
            comparison_argument_1 = operation.argument_1.clone();
            comparison_argument_2 = operation.argument_2.clone();
        }
        MonkeyJob::Value(_) => {
            panic!("expected root to be a binary operation");
        }
    }

    let mut humn: isize = match monkey_jobs.jobs.get("humn").unwrap() {
        MonkeyJob::DoOperation(_) => {
            panic!("expected humn to be a value")
        }
        MonkeyJob::Value(value) => *value,
    };

    let mut current_abs_error: usize = 0;
    let mut jitter: isize = 10_000_000_000;
    loop {
        let abs_error = (monkey_jobs.test_humn(comparison_argument_1.clone(), humn)
            - monkey_jobs.test_humn(comparison_argument_2.clone(), humn))
        .abs() as usize;

        if abs_error == 0 {
            break; // we found it!
        }

        if abs_error > current_abs_error {
            // we've gone too far! come back in in smaller steps
            jitter *= -1;
            jitter /= 10;
        }

        current_abs_error = abs_error;
        humn += jitter;
    }

    humn
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
    let monkey_jobs = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_1(monkey_jobs));

    let monkey_jobs = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_2(monkey_jobs));
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
