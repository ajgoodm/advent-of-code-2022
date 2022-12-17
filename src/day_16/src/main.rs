use std::cmp;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^Valve ([^ ]*) has flow rate=([0-9]*); tunnels? leads? to valves? ([A-Z, ]*)$"
    )
    .unwrap();
}

struct Valve {
    name: String,
    flow_rate: usize,
    neighbors: Vec<String>,
}

struct CaveMap {
    valves_by_name: HashMap<String, Valve>,
    non_zero_valves: HashSet<String>,
    distance_map: HashMap<(String, String), usize>,
}

impl CaveMap {
    fn all_valves(&self) -> HashSet<String> {
        self.valves_by_name
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }

    fn flow_rate_for_valve(&self, valve_name: &String) -> usize {
        self.valves_by_name.get(valve_name).unwrap().flow_rate
    }

    fn get_neighbors_for_valve(&self, valve_name: &String) -> &Vec<String> {
        &self.valves_by_name.get(valve_name).unwrap().neighbors
    }

    fn get_distance(&self, start: &String, end: &String) -> usize {
        if start == end {
            return 0;
        }
        *self
            .distance_map
            .get(&(start.clone(), end.clone()))
            .unwrap()
    }

    fn _calculate_distance(&self, start: &String, end: &String) -> usize {
        let mut unvisited_nodes: HashSet<String> = HashSet::new();
        let mut cost_to_visit_node: HashMap<String, usize> = HashMap::new();
        for node in self.all_valves() {
            cost_to_visit_node.insert(node.clone(), usize::MAX);
            unvisited_nodes.insert(node);
        }

        cost_to_visit_node.insert(start.clone(), 0);

        let mut current_node = start.clone();
        let mut current_cost = 0;
        loop {
            if &current_node == end {
                break;
            }

            unvisited_nodes.remove(&current_node);
            let unvisited_neighbors = self
                .get_neighbors_for_valve(&current_node)
                .iter()
                .filter(|node| unvisited_nodes.contains(node.clone()))
                .cloned()
                .collect::<Vec<String>>();
            for neighbor in unvisited_neighbors {
                if cost_to_visit_node.get(&neighbor).unwrap() > &(current_cost + 1) {
                    cost_to_visit_node.insert(neighbor, current_cost + 1);
                }
            }

            let mut min_cost = usize::MAX;
            for (node, cost) in cost_to_visit_node.iter() {
                if !unvisited_nodes.contains(node) {
                    continue;
                }
                if *cost < min_cost {
                    min_cost = *cost;
                    current_node = node.clone();
                    current_cost = *cost;
                }
            }
        }
        current_cost
    }

    fn _compute_distance_map(&mut self) {
        for start in self.all_valves() {
            for end in self.all_valves() {
                if start == end {
                    continue;
                }
                let distance = self._calculate_distance(&start, &end);
                self.distance_map.insert((start.clone(), end), distance);
            }
        }
    }
}

#[derive(Clone)]
enum Action {
    MoveTo(String),
    Open(String),
}

#[derive(Clone)]
struct Plan {
    actions: Vec<Action>,
    total_time: usize,
    current_valve: String,
    closed_valves: HashSet<String>,
}

impl Plan {
    fn next_plans(self, cave_map: &CaveMap) -> Vec<Plan> {
        let mut next_plans: Vec<Plan> = Vec::new();
        for closed_valve in self.closed_valves.intersection(&cave_map.non_zero_valves) {
            let distance = cave_map.get_distance(&self.current_valve, closed_valve);
            let mut next_plan_actions = self.actions.clone();
            for _ in 0..distance {
                next_plan_actions.push(Action::MoveTo(closed_valve.clone()));
            }

            next_plan_actions.push(Action::Open(closed_valve.clone()));
            let mut closed_valves = self.closed_valves.clone();
            closed_valves.remove(closed_valve);

            next_plans.push(Plan {
                actions: next_plan_actions,
                total_time: self.total_time,
                current_valve: closed_valve.clone(),
                closed_valves: closed_valves,
            })
        }

        next_plans
    }

    fn len(&self) -> usize {
        self.actions.len()
    }

    fn all_non_zero_valves_open(&self, cave_map: &CaveMap) -> bool {
        cave_map.non_zero_valves.is_disjoint(&self.closed_valves)
    }

    fn is_complete(&self, cave_map: &CaveMap) -> bool {
        self.all_non_zero_valves_open(&cave_map) || self.len() >= self.total_time
    }

    fn final_score(&self, cave_map: &CaveMap) -> usize {
        let mut total_pressure_released: usize = 0;
        for (time, action) in self.actions.iter().enumerate() {
            match action {
                Action::MoveTo(_) => (),
                Action::Open(valve_name) => {
                    let released_pressure =
                        cave_map.flow_rate_for_valve(&valve_name) * (self.total_time - (time + 1));
                    total_pressure_released += released_pressure;
                }
            }
        }
        total_pressure_released
    }

    fn upper_bound_score(&self, cave_map: &CaveMap) -> usize {
        let final_score_so_far = self.final_score(&cave_map);
        let mut upper_bound_score = final_score_so_far;

        let remaining_time = self.total_time - self.len();
        for valve in &self.closed_valves {
            let distance = cave_map.get_distance(&self.current_valve, valve);
            if distance < remaining_time {
                upper_bound_score +=
                    (remaining_time - distance - 1) * cave_map.flow_rate_for_valve(&valve);
            }
        }
        upper_bound_score
    }
}

#[derive(Clone)]
struct PlanWithElephant {
    actions_1: Vec<Action>,
    actions_2: Vec<Action>,
    total_time: usize,
    current_valve_1: String,
    current_valve_2: String,
    closed_valves: HashSet<String>,
}

impl PlanWithElephant {
    fn shortest_len(&self) -> usize {
        cmp::min(self.actions_1.len(), self.actions_2.len())
    }

    fn next_plans(self, cave_map: &CaveMap) -> Vec<PlanWithElephant> {
        let mut next_plans: Vec<PlanWithElephant> = Vec::new();
        let closed_valves: HashSet<String> = self
            .closed_valves
            .intersection(&cave_map.non_zero_valves)
            .cloned()
            .collect();
        if self.actions_1.len() < self.actions_2.len() {
            for next_valve in closed_valves {
                let distance = cave_map.get_distance(&self.current_valve_1, &next_valve);
                let mut next_plan_actions = self.actions_1.clone();
                for _ in 0..distance {
                    next_plan_actions.push(Action::MoveTo(next_valve.clone()));
                }

                next_plan_actions.push(Action::Open(next_valve.clone()));
                let mut next_closed_valves = self.closed_valves.clone();
                next_closed_valves.remove(&next_valve);

                next_plans.push(PlanWithElephant {
                    actions_1: next_plan_actions,
                    actions_2: self.actions_2.clone(),
                    total_time: self.total_time,
                    current_valve_1: next_valve,
                    current_valve_2: self.current_valve_2.clone(),
                    closed_valves: next_closed_valves,
                })
            }
        } else {
            for next_valve in closed_valves {
                let distance = cave_map.get_distance(&self.current_valve_2, &next_valve);
                let mut next_plan_actions = self.actions_2.clone();
                for _ in 0..distance {
                    next_plan_actions.push(Action::MoveTo(next_valve.clone()));
                }

                next_plan_actions.push(Action::Open(next_valve.clone()));
                let mut next_closed_valves = self.closed_valves.clone();
                next_closed_valves.remove(&next_valve);

                next_plans.push(PlanWithElephant {
                    actions_1: self.actions_1.clone(),
                    actions_2: next_plan_actions,
                    total_time: self.total_time,
                    current_valve_1: self.current_valve_1.clone(),
                    current_valve_2: next_valve,
                    closed_valves: next_closed_valves,
                })
            }
        }

        next_plans
    }

    fn all_non_zero_valves_open(&self, cave_map: &CaveMap) -> bool {
        cave_map.non_zero_valves.is_disjoint(&self.closed_valves)
    }

    fn is_complete(&self, cave_map: &CaveMap) -> bool {
        self.all_non_zero_valves_open(&cave_map) || self.shortest_len() >= self.total_time
    }

    fn final_score(&self, cave_map: &CaveMap) -> usize {
        let mut total_pressure_released: usize = 0;
        for time in 0..self.total_time {
            if time < self.actions_1.len() {
                match &self.actions_1[time] {
                    Action::MoveTo(_) => (),
                    Action::Open(valve_name) => {
                        let released_pressure = cave_map.flow_rate_for_valve(&valve_name)
                            * (self.total_time - (time + 1));
                        total_pressure_released += released_pressure;
                    }
                }
            }
            if time < self.actions_2.len() {
                match &self.actions_2[time] {
                    Action::MoveTo(_) => (),
                    Action::Open(valve_name) => {
                        let released_pressure = cave_map.flow_rate_for_valve(&valve_name)
                            * (self.total_time - (time + 1));
                        total_pressure_released += released_pressure;
                    }
                }
            }
        }
        total_pressure_released
    }

    fn upper_bound_score(&self, cave_map: &CaveMap) -> usize {
        let final_score_so_far = self.final_score(&cave_map);
        let mut upper_bound_score = final_score_so_far;

        let remaining_time = self.total_time - self.shortest_len();
        for valve in &self.closed_valves {
            let distance_1 = cave_map.get_distance(&self.current_valve_1, valve);
            let distance_2 = cave_map.get_distance(&self.current_valve_2, valve);
            let distance = cmp::min(distance_1, distance_2);
            if distance < remaining_time {
                upper_bound_score +=
                    (remaining_time - distance - 1) * cave_map.flow_rate_for_valve(&valve);
            }
        }
        upper_bound_score
    }
}

fn parse_line(line: String) -> Valve {
    let captures = INPUT_RE.captures(&line).unwrap();
    let valve_name: String = captures.get(1).unwrap().as_str().to_string();
    let flow_rate: usize = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let destination_valves: Vec<String> = captures
        .get(3)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    Valve {
        name: valve_name.clone(),
        flow_rate,
        neighbors: destination_valves,
    }
}

fn parse_input(reader: AocBufReader) -> CaveMap {
    let valves_by_name: HashMap<String, Valve> = reader
        .map(|line| {
            let valve = parse_line(line);
            (valve.name.clone(), valve)
        })
        .collect();
    let non_zero_valves: HashSet<String> = valves_by_name
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(valve_name, _)| valve_name.clone())
        .collect();

    let mut cave_map = CaveMap {
        valves_by_name,
        non_zero_valves,
        distance_map: HashMap::new(),
    };
    cave_map._compute_distance_map();
    cave_map
}

fn part_1(cave_map: &CaveMap, time_available: usize, starting_valve_name: String) -> usize {
    let mut best_plan_score: usize = 0;
    let mut candidate_plans: Vec<Plan> = vec![Plan {
        actions: vec![],
        total_time: time_available,
        current_valve: starting_valve_name,
        closed_valves: cave_map.all_valves(),
    }];

    while candidate_plans.len() > 0 {
        let depth_first_candidate = candidate_plans.pop().unwrap();
        let next_plans = depth_first_candidate.next_plans(&cave_map);
        for next_plan in next_plans {
            if next_plan.is_complete(&cave_map) {
                if next_plan.final_score(&cave_map) > best_plan_score {
                    best_plan_score = next_plan.final_score(&cave_map);
                }
            } else {
                let upper_bound_score = next_plan.upper_bound_score(&cave_map);
                if upper_bound_score > best_plan_score {
                    candidate_plans.push(next_plan);
                }
            }
        }
    }
    best_plan_score
}

fn part_2(cave_map: &CaveMap, time_available: usize, starting_valve_name: String) -> usize {
    let mut best_plan_score: usize = 0;
    let mut candidate_plans: Vec<PlanWithElephant> = vec![PlanWithElephant {
        actions_1: vec![],
        actions_2: vec![],
        total_time: time_available,
        current_valve_1: starting_valve_name.clone(),
        current_valve_2: starting_valve_name,
        closed_valves: cave_map.all_valves(),
    }];

    while candidate_plans.len() > 0 {
        let depth_first_candidate = candidate_plans.pop().unwrap();
        let next_plans = depth_first_candidate.next_plans(&cave_map);
        for next_plan in next_plans {
            if next_plan.is_complete(&cave_map) {
                if next_plan.final_score(&cave_map) > best_plan_score {
                    best_plan_score = next_plan.final_score(&cave_map);
                }
            } else {
                let upper_bound_score = next_plan.upper_bound_score(&cave_map);
                if upper_bound_score > best_plan_score {
                    candidate_plans.push(next_plan);
                }
            }
        }
    }
    best_plan_score
}

fn main() {
    let cave_map = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_1(&cave_map, 30, "AA".to_string()));
    println!("{}", part_2(&cave_map, 26, "AA".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        parse_line("Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string());
        parse_input(AocBufReader::from_string("inputs/example.txt"));
        parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    }

    #[test]
    fn test_scoring() {
        let cave_map = parse_input(AocBufReader::from_string("inputs/example.txt"));

        let plan = Plan {
            actions: vec![
                Action::MoveTo("DD".to_string()),
                Action::Open("DD".to_string()),
                Action::MoveTo("CC".to_string()),
                Action::MoveTo("BB".to_string()),
                Action::Open("BB".to_string()),
                Action::MoveTo("AA".to_string()),
                Action::MoveTo("II".to_string()),
                Action::MoveTo("JJ".to_string()),
                Action::Open("JJ".to_string()),
                Action::MoveTo("II".to_string()),
                Action::MoveTo("AA".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("EE".to_string()),
                Action::MoveTo("FF".to_string()),
                Action::MoveTo("GG".to_string()),
                Action::MoveTo("HH".to_string()),
                Action::Open("HH".to_string()),
                Action::MoveTo("GG".to_string()),
                Action::MoveTo("FF".to_string()),
                Action::MoveTo("EE".to_string()),
                Action::Open("EE".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("CC".to_string()),
                Action::Open("CC".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("DD".to_string()),
                Action::MoveTo("DD".to_string()),
            ],
            total_time: 30,
            current_valve: "AA".to_string(),
            closed_valves: HashSet::new(),
        };
        assert_eq!(plan.final_score(&cave_map), 1651);
    }
}
