use std::collections::{HashMap, HashSet};

use shared::conversion::char_to_usize;
use shared::input::AocBufReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn up(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }
}

struct Map {
    start: Coord,
    end: Coord,
    heights: Vec<Vec<usize>>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn get_val(&self, coord: &Coord) -> usize {
        self.heights[coord.row][coord.col]
    }

    fn next_coords_allowed(&self, coord: &Coord) -> Vec<Coord> {
        let mut next_coords_allowed: Vec<Coord> = Vec::new();

        let current_val = self.get_val(coord);
        if coord.row > 0 && self.get_val(&coord.up()) <= current_val + 1 {
            next_coords_allowed.push(coord.up());
        }
        if coord.row < self.n_rows - 1 && self.get_val(&coord.down()) <= current_val + 1 {
            next_coords_allowed.push(coord.down());
        }
        if coord.col > 0 && self.get_val(&coord.left()) <= current_val + 1 {
            next_coords_allowed.push(coord.left());
        }
        if coord.col < self.n_cols - 1 && self.get_val(&coord.right()) <= current_val + 1 {
            next_coords_allowed.push(coord.right());
        }
        next_coords_allowed
    }

    fn find_route(&self) -> usize {
        let mut unvisited_nodes: HashSet<Coord> = HashSet::new();
        let mut cost_to_visit_node: HashMap<Coord, usize> = HashMap::new();
        for (row_idx, row) in self.heights.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {
                let coord = Coord {
                    row: row_idx,
                    col: col_idx,
                };
                cost_to_visit_node.insert(coord.clone(), usize::MAX);
                unvisited_nodes.insert(coord);
            }
        }
        cost_to_visit_node.insert(self.start.clone(), 0);

        let mut current_node = self.start.clone();
        let mut current_cost = 0;
        loop {
            if current_node == self.end {
                break;
            }

            unvisited_nodes.remove(&current_node);
            let unvisited_neighbors = self
                .next_coords_allowed(&current_node)
                .iter()
                .filter(|coord| unvisited_nodes.contains(coord))
                .cloned()
                .collect::<Vec<Coord>>();
            for neighbor in unvisited_neighbors {
                if cost_to_visit_node.get(&neighbor).unwrap() > &(current_cost + 1) {
                    cost_to_visit_node.insert(neighbor, current_cost + 1);
                }
            }
            current_node = *cost_to_visit_node
                .iter()
                .filter(|(coord, _)| unvisited_nodes.contains(coord))
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k)
                .unwrap();
            current_cost = *cost_to_visit_node.get(&current_node).unwrap();
        }

        current_cost
    }

    fn find_route_part_2(&self) -> usize {
        let mut unvisited_nodes: HashSet<Coord> = HashSet::new();
        let mut cost_to_visit_node: HashMap<Coord, usize> = HashMap::new();
        for (row_idx, row) in self.heights.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                let coord = Coord {
                    row: row_idx,
                    col: col_idx,
                };
                if val == &1usize {
                    cost_to_visit_node.insert(coord.clone(), 0);
                } else {
                    cost_to_visit_node.insert(coord.clone(), usize::MAX);
                }
                unvisited_nodes.insert(coord);
            }
        }
        cost_to_visit_node.insert(self.start.clone(), 0);

        let mut current_node = self.start.clone();
        let mut current_cost = 0;
        loop {
            if current_node == self.end {
                break;
            }

            unvisited_nodes.remove(&current_node);
            let unvisited_neighbors = self
                .next_coords_allowed(&current_node)
                .iter()
                .filter(|coord| unvisited_nodes.contains(coord))
                .cloned()
                .collect::<Vec<Coord>>();
            for neighbor in unvisited_neighbors {
                if cost_to_visit_node.get(&neighbor).unwrap() > &(current_cost + 1) {
                    cost_to_visit_node.insert(neighbor, current_cost + 1);
                }
            }
            current_node = *cost_to_visit_node
                .iter()
                .filter(|(coord, _)| unvisited_nodes.contains(coord))
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k)
                .unwrap();
            current_cost = *cost_to_visit_node.get(&current_node).unwrap();
        }

        current_cost
    }
}

fn parse_input(reader: AocBufReader) -> Map {
    let mut heights: Vec<Vec<usize>> = Vec::new();
    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;

    for (row_idx, line) in reader.enumerate() {
        let mut row: Vec<usize> = Vec::new();
        for (col_idx, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some(Coord {
                    row: row_idx,
                    col: col_idx,
                });
                row.push(char_to_usize('a'));
            } else if c == 'E' {
                end = Some(Coord {
                    row: row_idx,
                    col: col_idx,
                });
                row.push(char_to_usize('z'));
            } else {
                row.push(char_to_usize(c));
            }
        }
        heights.push(row);
    }
    let n_rows: usize = heights.len();
    let n_cols: usize = heights[0].len();

    Map {
        start: start.unwrap(),
        end: end.unwrap(),
        heights,
        n_rows,
        n_cols,
    }
}

fn main() {
    let part_1_map = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_1_map.find_route());
    println!("{}", part_1_map.find_route_part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let example_map = parse_input(AocBufReader::from_string("inputs/example.txt"));
        println!("{}", example_map.find_route());
    }
}
