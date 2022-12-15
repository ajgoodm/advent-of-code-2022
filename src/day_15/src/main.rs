use std::cmp;
use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^Sensor at x=([0-9\-]*), y=([0-9\-]*): closest beacon is at x=([0-9\-]*), y=([0-9\-]*)$"
    )
    .unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn manhattan_distance(start: &Coord, end: &Coord) -> usize {
        (end.row - start.row).abs() as usize + (end.col - start.col).abs() as usize
    }

    fn interval_within_x_at_row(
        &self,
        manhattan_distance: usize,
        row: isize,
    ) -> Option<IntervalInclusive> {
        let distance_to_row: usize = (row - self.row).abs() as usize;
        if distance_to_row > manhattan_distance {
            None
        } else {
            let horizontal_remainder: isize = (manhattan_distance - distance_to_row) as isize;
            Some(IntervalInclusive {
                min: self.col - horizontal_remainder,
                max: self.col + horizontal_remainder,
            })
        }
    }
}

#[derive(Clone)]
struct IntervalInclusive {
    min: isize,
    max: isize,
}

impl IntervalInclusive {
    fn overlaps(&self, other: &IntervalInclusive) -> bool {
        !(self.max < other.min || self.min > other.max)
    }

    fn contains(&self, other: &IntervalInclusive) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn merge(&mut self, other: IntervalInclusive) {
        self.min = cmp::min(self.min, other.min);
        self.max = cmp::max(self.max, other.max);
    }
}

struct IntervalCollection {
    intervals: Vec<IntervalInclusive>,
}

impl IntervalCollection {
    fn new() -> IntervalCollection {
        IntervalCollection { intervals: vec![] }
    }

    fn add_point(&mut self, point: isize) {
        self.add(IntervalInclusive {
            min: point,
            max: point,
        })
    }

    fn add(&mut self, mut new: IntervalInclusive) {
        let mut merged_intervals: Vec<IntervalInclusive> = Vec::new();
        for interval in &self.intervals {
            if interval.overlaps(&new) {
                new.merge(interval.clone());
            } else {
                merged_intervals.push(interval.clone());
            }
        }
        merged_intervals.push(new);
        self.intervals = merged_intervals;
    }

    fn difference(&mut self, other: &IntervalInclusive) {
        let mut differenced_intervals: Vec<IntervalInclusive> = Vec::new();
        for interval in &self.intervals {
            if interval.contains(other) {
                if other.min > interval.min {
                    differenced_intervals.push(IntervalInclusive {
                        min: interval.min,
                        max: other.min - 1,
                    })
                }
                if other.max < interval.max {
                    differenced_intervals.push(IntervalInclusive {
                        min: other.max + 1,
                        max: interval.max,
                    })
                }
            } else if other.contains(interval) {
                continue;
            } else if interval.overlaps(other) {
                if other.min <= interval.min && other.max < interval.max {
                    differenced_intervals.push(IntervalInclusive {
                        min: other.max + 1,
                        max: interval.max,
                    })
                } else if interval.min < other.min {
                    differenced_intervals.push(IntervalInclusive {
                        min: interval.min,
                        max: other.min - 1,
                    })
                }
            } else {
                differenced_intervals.push(interval.clone());
            }
        }
        self.intervals = differenced_intervals;
    }

    fn total_length(&self) -> usize {
        self.intervals
            .iter()
            .map(|interval| (interval.max - interval.min + 1) as usize)
            .sum()
    }
}

struct Sensor {
    location: Coord,
    closest_beacon: Coord,
    distance_to_beacon: usize,
}

fn parse_input(reader: AocBufReader) -> Vec<Sensor> {
    reader
        .map(|line| {
            let captures = INPUT_RE.captures(&line).unwrap();
            let location = Coord {
                row: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                col: captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            };
            let closest_beacon = Coord {
                row: captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                col: captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            };
            let distance_to_beacon = Coord::manhattan_distance(&location, &closest_beacon);
            Sensor {
                location,
                closest_beacon,
                distance_to_beacon,
            }
        })
        .collect()
}

fn part_1(sensors: Vec<Sensor>, row: isize) -> usize {
    let mut known_beacon_locations = IntervalCollection::new();
    let mut coords_in_row_covered_by_sensor = IntervalCollection::new();

    for sensor in sensors {
        if sensor.closest_beacon.row == row {
            known_beacon_locations.add_point(sensor.closest_beacon.col);
        }
        if let Some(interval) = sensor
            .location
            .interval_within_x_at_row(sensor.distance_to_beacon, row)
        {
            coords_in_row_covered_by_sensor.add(interval);
        }
    }

    for beacon in known_beacon_locations.intervals {
        coords_in_row_covered_by_sensor.difference(&beacon);
    }

    coords_in_row_covered_by_sensor.total_length()
}

fn part_2(sensors: Vec<Sensor>, max_coord: isize) -> isize {
    let mut beacon_row: isize = 0;
    let mut beacon_col: isize = 0;

    for row in 0isize..max_coord {
        let mut coords_in_row_covered_by_sensor = IntervalCollection::new();
        for sensor in &sensors {
            if let Some(interval) = sensor
                .location
                .interval_within_x_at_row(sensor.distance_to_beacon, row)
            {
                coords_in_row_covered_by_sensor.add(interval);
            }
        }
        let mut possible_locations = IntervalCollection {
            intervals: vec![IntervalInclusive {
                min: 0,
                max: max_coord,
            }],
        };
        for interval in coords_in_row_covered_by_sensor.intervals {
            possible_locations.difference(&interval);
        }
        if possible_locations.total_length() > 0 {
            beacon_row = row;
            beacon_col = possible_locations.intervals[0].min;
            break;
        }
    }
    beacon_col * 4_000_000 + beacon_row
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let sensors = parse_input(reader);
    println!("{}", part_1(sensors, 2000000));

    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let sensors = parse_input(reader);
    println!("{}", part_2(sensors, 4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let reader = AocBufReader::from_string("inputs/example.txt");
        let sensors = parse_input(reader);
        assert_eq!(part_1(sensors, 10), 26);
    }

    #[test]
    fn test_example_part_2() {
        let reader = AocBufReader::from_string("inputs/example.txt");
        let sensors = parse_input(reader);
        assert_eq!(part_2(sensors, 20), 56000011);
    }
}
