use std::collections::{HashMap, HashSet};

use shared::input::AocBufReader;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn north(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn east(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn south(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn west(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }
}

struct BoundingBox {
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
    all_spaces: HashSet<Coord>,
}

impl BoundingBox {
    fn new(min_row: usize, max_row: usize, min_col: usize, max_col: usize) -> BoundingBox {
        let mut all_spaces: HashSet<Coord> = HashSet::new();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                all_spaces.insert(Coord { row, col });
            }
        }

        BoundingBox {
            min_row,
            max_row,
            min_col,
            max_col,
            all_spaces,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    position: Coord,
    t: usize,
}

struct BlizzardMap {
    start: Coord,
    end: Coord,
    bounding_box: BoundingBox,
    blizzards_north: HashSet<Coord>,
    blizzards_east: HashSet<Coord>,
    blizzards_south: HashSet<Coord>,
    blizzards_west: HashSet<Coord>,
    _t: usize,
    _open_spaces_by_t: HashMap<usize, HashSet<Coord>>,
}

impl BlizzardMap {
    fn new(
        start: Coord,
        end: Coord,
        bounding_box: BoundingBox,
        blizzards_north: HashSet<Coord>,
        blizzards_east: HashSet<Coord>,
        blizzards_south: HashSet<Coord>,
        blizzards_west: HashSet<Coord>,
    ) -> BlizzardMap {
        let mut blizzard_map = BlizzardMap {
            start,
            end,
            bounding_box,
            blizzards_north,
            blizzards_east,
            blizzards_south,
            blizzards_west,
            _t: 0,
            _open_spaces_by_t: HashMap::new(),
        };

        let all_spaces = blizzard_map
            .bounding_box
            .all_spaces
            .iter()
            .cloned()
            .collect::<HashSet<Coord>>();
        let occupied_spaces = blizzard_map.blizzard_spaces();
        blizzard_map._open_spaces_by_t.insert(
            0,
            all_spaces
                .difference(&occupied_spaces)
                .cloned()
                .collect::<HashSet<Coord>>(),
        );

        blizzard_map
    }

    fn blizzard_spaces(&self) -> HashSet<Coord> {
        self.blizzards_north
            .union(&self.blizzards_east)
            .cloned()
            .collect::<HashSet<Coord>>()
            .union(&self.blizzards_south)
            .cloned()
            .collect::<HashSet<Coord>>()
            .union(&self.blizzards_west)
            .cloned()
            .collect::<HashSet<Coord>>()
    }

    fn calculate_next(&mut self) {
        self._t += 1;

        let blizzards_north: HashSet<Coord> = self
            .blizzards_north
            .iter()
            .map(|coord| {
                let row: usize;
                let col: usize = coord.col;
                if coord.row == self.bounding_box.min_row {
                    row = self.bounding_box.max_row;
                } else {
                    row = coord.row - 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_north = blizzards_north;

        let blizzards_east: HashSet<Coord> = self
            .blizzards_east
            .iter()
            .map(|coord| {
                let row: usize = coord.row;
                let col: usize;
                if coord.col == self.bounding_box.max_col {
                    col = self.bounding_box.min_col;
                } else {
                    col = coord.col + 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_east = blizzards_east;

        let blizzards_south: HashSet<Coord> = self
            .blizzards_south
            .iter()
            .map(|coord| {
                let row: usize;
                let col: usize = coord.col;
                if coord.row == self.bounding_box.max_row {
                    row = self.bounding_box.min_row;
                } else {
                    row = coord.row + 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_south = blizzards_south;

        let blizzards_west: HashSet<Coord> = self
            .blizzards_west
            .iter()
            .map(|coord| {
                let row: usize = coord.row;
                let col: usize;
                if coord.col == self.bounding_box.min_col {
                    col = self.bounding_box.max_col;
                } else {
                    col = coord.col - 1;
                }
                Coord { row, col }
            })
            .collect();
        self.blizzards_west = blizzards_west;

        let all_spaces = self
            .bounding_box
            .all_spaces
            .iter()
            .cloned()
            .collect::<HashSet<Coord>>();
        let occupied_spaces = self.blizzard_spaces();
        let mut unoccupied_spaces = all_spaces
            .difference(&occupied_spaces)
            .cloned()
            .collect::<HashSet<Coord>>();
        unoccupied_spaces.insert(self.end.clone());

        self._open_spaces_by_t.insert(self._t, unoccupied_spaces);
    }

    fn get_open_spaces_at_time_t(&mut self, t: usize) -> &HashSet<Coord> {
        while self._t < t {
            self.calculate_next()
        }

        self._open_spaces_by_t.get(&t).unwrap()
    }

    fn get_neighbor_nodes(&mut self, node: Node) -> Vec<Node> {
        let next_t = node.t + 1;
        let mut neighbor_nodes: Vec<Node> = Vec::new();

        let all_open_spaces = self.get_open_spaces_at_time_t(next_t);
        if node.position.row != 0 {
            let north = node.position.north();
            if all_open_spaces.contains(&north) {
                neighbor_nodes.push(Node {
                    position: north,
                    t: next_t,
                });
            }
        }

        if all_open_spaces.contains(&node.position) {
            // we can just wait
            neighbor_nodes.push(Node {
                position: node.position.clone(),
                t: next_t,
            });
        }

        let east = node.position.east();
        if all_open_spaces.contains(&east) {
            neighbor_nodes.push(Node {
                position: east,
                t: next_t,
            });
        }
        let south = node.position.south();
        if all_open_spaces.contains(&south) {
            neighbor_nodes.push(Node {
                position: south,
                t: next_t,
            });
        }
        let west = node.position.west();
        if all_open_spaces.contains(&west) {
            neighbor_nodes.push(Node {
                position: west,
                t: next_t,
            });
        }

        neighbor_nodes
    }
}

fn parse_input(reader: AocBufReader) -> BlizzardMap {
    let mut blizzards_north: HashSet<Coord> = HashSet::new();
    let mut blizzards_east: HashSet<Coord> = HashSet::new();
    let mut blizzards_south: HashSet<Coord> = HashSet::new();
    let mut blizzards_west: HashSet<Coord> = HashSet::new();

    let mut max_row: usize = 0;
    let mut max_col: usize = 0;
    let mut start: Coord = Coord { row: 0, col: 0 };
    let mut end: Coord = Coord { row: 0, col: 0 };
    for (row, line) in reader.enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => (),
                '.' => {
                    if row == 0 {
                        start = Coord { row, col };
                    } else {
                        end = Coord { row, col };
                    }
                }
                '^' => {
                    blizzards_north.insert(Coord { row, col });
                }
                '>' => {
                    blizzards_east.insert(Coord { row, col });
                }
                'v' => {
                    blizzards_south.insert(Coord { row, col });
                }
                '<' => {
                    blizzards_west.insert(Coord { row, col });
                }
                _ => {
                    panic!("unexepcted char {}", c);
                }
            }
            max_col = col as usize;
        }
        max_row = row as usize;
    }

    let bounding_box = BoundingBox::new(1, max_row - 1, 1, max_col - 1);

    BlizzardMap::new(
        start,
        end,
        bounding_box,
        blizzards_north,
        blizzards_east,
        blizzards_south,
        blizzards_west,
    )
}

fn part_1(reader: AocBufReader) -> usize {
    let mut blizzard_map = parse_input(reader);
    let mut visited_nodes: HashSet<Node> = HashSet::new();
    let mut unvisited_nodes: HashSet<Node> = HashSet::new();

    unvisited_nodes.insert(Node {
        position: blizzard_map.start.clone(),
        t: 0,
    });

    let mut minimum_time: usize = usize::MAX;
    while unvisited_nodes.len() > 0 {
        let node = unvisited_nodes.iter().next().unwrap().clone();
        unvisited_nodes.remove(&node);
        visited_nodes.insert(node.clone());

        let next_nodes = blizzard_map.get_neighbor_nodes(node);
        for next_node in next_nodes {
            if next_node.position == blizzard_map.end {
                if next_node.t < minimum_time {
                    minimum_time = next_node.t;
                }
            } else if !visited_nodes.contains(&next_node) {
                if next_node.t < (minimum_time - 1) {
                    unvisited_nodes.insert(next_node);
                }
            }
        }
    }

    minimum_time
}

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("inputs/part_1.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_input() {
        parse_input(AocBufReader::from_string("inputs/example.txt"));
        parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    }
}
