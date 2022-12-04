use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref INPUT_REGEX: Regex = Regex::new(r"^([0-9]*)-([0-9]*),([0-9]*)-([0-9]*)$").unwrap();
}

fn parse_single_line(string: String) -> ((usize, usize), (usize, usize)) {
    let captures = INPUT_REGEX.captures(&string).unwrap();

    (
        (
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        ),
        (
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        ),
    )
}

fn parse_to_tuples(reader: AocBufReader) -> Vec<((usize, usize), (usize, usize))> {
    reader
        .into_iter()
        .map(|line| parse_single_line(line))
        .collect::<Vec<((usize, usize), (usize, usize))>>()
}

fn one_range_contains_other(x: &(usize, usize), y: &(usize, usize)) -> bool {
    let (x1, x2) = x;
    let (y1, y2) = y;

    (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
}

fn ranges_overlap(x: &(usize, usize), y: &(usize, usize)) -> bool {
    let (x1, x2) = x;
    let (y1, y2) = y;

    !(x2 < y1 || y2 < x1)
}

fn part_1(tuple_pairs: Vec<((usize, usize), (usize, usize))>) -> usize {
    tuple_pairs
        .into_iter()
        .filter(|(r1, r2)| one_range_contains_other(r1, r2))
        .count()
}

fn part_2(tuple_pairs: Vec<((usize, usize), (usize, usize))>) -> usize {
    tuple_pairs
        .into_iter()
        .filter(|(r1, r2)| ranges_overlap(r1, r2))
        .count()
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let tuple_pairs: Vec<((usize, usize), (usize, usize))> = parse_to_tuples(reader);
    println!("{}", part_1(tuple_pairs));

    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let tuple_pairs: Vec<((usize, usize), (usize, usize))> = parse_to_tuples(reader);
    println!("{}", part_2(tuple_pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_line() {
        assert_eq!(
            parse_single_line("12-34,56-78".to_string()),
            ((12, 34), (56, 78))
        );
    }
}
