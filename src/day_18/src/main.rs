use std::collections::HashSet;

use shared::input::AocBufReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn neighbors(&self) -> HashSet<Coord> {
        vec![
            Coord {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Coord {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Coord {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
        .into_iter()
        .collect()
    }
}

struct BoundingBox {
    minimum_corner: Coord,
    maximum_corner: Coord,
}

impl BoundingBox {
    fn contains_coord(&self, coord: &Coord) -> bool {
        self.minimum_corner.x <= coord.x
            && coord.x <= self.maximum_corner.x
            && self.minimum_corner.y <= coord.y
            && coord.y <= self.maximum_corner.y
            && self.minimum_corner.z <= coord.z
            && coord.z <= self.maximum_corner.z
    }
}

struct RockWithHoles {
    rock_coords: HashSet<Coord>,
    bounding_box: BoundingBox,
}

impl RockWithHoles {
    fn new(reader: AocBufReader) -> RockWithHoles {
        let rock_coords = parse_input(reader);

        let min_x: isize = rock_coords.iter().map(|coord| coord.x).min().unwrap();
        let max_x: isize = rock_coords.iter().map(|coord| coord.x).max().unwrap();
        let min_y: isize = rock_coords.iter().map(|coord| coord.y).min().unwrap();
        let max_y: isize = rock_coords.iter().map(|coord| coord.y).max().unwrap();
        let min_z: isize = rock_coords.iter().map(|coord| coord.z).min().unwrap();
        let max_z: isize = rock_coords.iter().map(|coord| coord.z).max().unwrap();
        let bounding_box = BoundingBox {
            minimum_corner: Coord {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            maximum_corner: Coord {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        };

        RockWithHoles {
            rock_coords,
            bounding_box,
        }
    }

    fn all_boundary_neighbors(&self) -> HashSet<Coord> {
        let mut all_boundary_neighbors: HashSet<Coord> = HashSet::new();
        for coord in &self.rock_coords {
            all_boundary_neighbors.extend(
                coord
                    .neighbors()
                    .difference(&self.rock_coords)
                    .cloned()
                    .collect::<HashSet<Coord>>(),
            );
        }
        all_boundary_neighbors
    }

    fn total_surface_area(&self) -> usize {
        self.rock_coords
            .iter()
            .map(|coord| {
                coord
                    .neighbors()
                    .difference(&self.rock_coords)
                    .cloned()
                    .collect::<HashSet<Coord>>()
                    .len()
            })
            .sum()
    }

    fn bubble_surface_area(&self) -> usize {
        let mut unclassified_neighbors = self.all_boundary_neighbors();
        let mut bubble_coords: HashSet<Coord> = HashSet::new();

        // iterate through unclassified neighbors and classify as external or internal
        while unclassified_neighbors.len() > 1 {
            let unclassified_coord = unclassified_neighbors.iter().next().unwrap();
            let (is_bubble, coords) = self.classify_coord(unclassified_coord.clone());

            unclassified_neighbors = unclassified_neighbors
                .difference(&coords)
                .cloned()
                .collect::<HashSet<Coord>>();
            if is_bubble {
                bubble_coords.extend(coords);
            }
        }

        bubble_coords
            .iter()
            .map(|coord| {
                coord
                    .neighbors()
                    .difference(&bubble_coords)
                    .cloned()
                    .collect::<HashSet<Coord>>()
                    .len()
            })
            .sum()
    }

    /// Returns a boolean indicating whether this coord belongs to a bubble (true)
    /// and a set of coordinates that were explored while classifying this coordinate.
    /// All returned coordinates are `connected` to coord.
    fn classify_coord(&self, coord: Coord) -> (bool, HashSet<Coord>) {
        let mut explored_coords: HashSet<Coord> = vec![].into_iter().collect();
        let mut unexplored_coords: HashSet<Coord> = vec![coord].into_iter().collect();
        loop {
            if unexplored_coords.len() == 0 {
                return (true, explored_coords);
            }

            let coord_to_explore = unexplored_coords.iter().next().unwrap().clone();
            unexplored_coords.remove(&coord_to_explore);
            explored_coords.insert(coord_to_explore.clone());
            if !self.bounding_box.contains_coord(&coord_to_explore) {
                // this exterior region breaks out of the bounds of the rock.
                // we are not in a bubble!
                break;
            }

            let new_neighbors_to_explore = coord_to_explore
                .neighbors()
                .difference(&self.rock_coords)
                .cloned()
                .collect::<HashSet<Coord>>()
                .difference(&explored_coords)
                .cloned()
                .collect::<HashSet<Coord>>();
            unexplored_coords.extend(new_neighbors_to_explore);
        }

        (false, explored_coords)
    }
}

fn parse_input(reader: AocBufReader) -> HashSet<Coord> {
    reader
        .map(|line| {
            let mut split = line.split(",");
            Coord {
                x: split.next().unwrap().parse::<isize>().unwrap(),
                y: split.next().unwrap().parse::<isize>().unwrap(),
                z: split.next().unwrap().parse::<isize>().unwrap(),
            }
        })
        .collect()
}

fn part_1(reader: AocBufReader) -> usize {
    let rock_with_holes = RockWithHoles::new(reader);
    rock_with_holes.total_surface_area()
}

fn part_2(reader: AocBufReader) -> usize {
    let rock_with_holes = RockWithHoles::new(reader);
    rock_with_holes.total_surface_area() - rock_with_holes.bubble_surface_area()
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("{}", part_1(reader));

    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("{}", part_2(reader));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let reader = AocBufReader::from_string("inputs/example.txt");
        let coords = parse_input(reader);
    }

    #[test]
    fn test_example_part_2() {
        let reader = AocBufReader::from_string("inputs/example.txt");
        assert_eq!(part_2(reader), 58);
    }
}
