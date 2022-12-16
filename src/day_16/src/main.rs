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
    open_valves: HashSet<String>,
}

impl Plan {
    fn next_plans(self, valves_by_name: &HashMap<String, Valve>) -> Vec<Plan> {
        let mut next_plans: Vec<Plan> = Vec::new();
        if !self.open_valves.contains(&self.current_valve)
            && valves_by_name.get(&self.current_valve).unwrap().flow_rate > 0
        {
            let mut next_plan_actions = self.actions.clone();
            next_plan_actions.push(Action::Open(self.current_valve.clone()));

            let mut open_valves = self.open_valves.clone();
            open_valves.insert(self.current_valve.clone());

            next_plans.push(Plan {
                actions: next_plan_actions,
                total_time: self.total_time,
                current_valve: self.current_valve.clone(),
                open_valves: open_valves,
            });
        }

        for next_valve in &valves_by_name.get(&self.current_valve).unwrap().neighbors {
            let mut next_plan_actions = self.actions.clone();
            next_plan_actions.push(Action::MoveTo(next_valve.clone()));
            next_plans.push(Plan {
                actions: next_plan_actions,
                total_time: self.total_time,
                current_valve: next_valve.clone(),
                open_valves: self.open_valves.clone(),
            });
        }

        next_plans
    }

    fn len(&self) -> usize {
        self.actions.len()
    }

    fn all_non_zero_valves_open(&self, non_zero_valves: &HashSet<String>) -> bool {
        non_zero_valves.is_subset(&self.open_valves)
    }

    fn is_complete(&self, non_zero_valves: &HashSet<String>) -> bool {
        self.all_non_zero_valves_open(&non_zero_valves) || self.len() >= self.total_time
    }

    fn final_score(&self, valves_by_name: &HashMap<String, Valve>) -> usize {
        let mut total_pressure_released: usize = 0;
        for (time, action) in self.actions.iter().enumerate() {
            match action {
                Action::MoveTo(_) => (),
                Action::Open(valve_name) => {
                    let released_pressure = valves_by_name.get(valve_name).unwrap().flow_rate
                        * (self.total_time - (time + 1));
                    total_pressure_released += released_pressure;
                }
            }
        }
        total_pressure_released
    }

    fn upper_bound_score(
        &self,
        valves_by_name: &HashMap<String, Valve>,
        non_zero_valves: &HashSet<String>,
    ) -> usize {
        let mut upper_bound_score = 0;
        for valve in &self.open_valves {
            upper_bound_score += self.total_time * valves_by_name.get(valve).unwrap().flow_rate;
        }

        let remaining_time = self.total_time - self.actions.len();
        for valve in non_zero_valves.difference(&self.open_valves) {
            upper_bound_score += remaining_time * valves_by_name.get(valve).unwrap().flow_rate;
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

fn parse_input(reader: AocBufReader) -> HashMap<String, Valve> {
    reader
        .map(|line| {
            let valve = parse_line(line);
            (valve.name.clone(), valve)
        })
        .collect()
}

fn part_1(
    valves_by_name: HashMap<String, Valve>,
    time_available: usize,
    starting_valve_name: String,
) -> usize {
    let mut best_plan_score: usize = 0;
    let mut candidate_plans: Vec<Plan> = vec![Plan {
        actions: vec![],
        total_time: time_available,
        current_valve: starting_valve_name,
        open_valves: HashSet::new(),
    }];
    let non_zero_valves: HashSet<String> = valves_by_name
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(valve_name, _)| valve_name.clone())
        .collect();

    while candidate_plans.len() > 0 {
        let depth_first_candidate = candidate_plans.pop().unwrap();
        let next_plans = depth_first_candidate.next_plans(&valves_by_name);
        for next_plan in next_plans {
            if next_plan.is_complete(&non_zero_valves) {
                if next_plan.final_score(&valves_by_name) > best_plan_score {
                    best_plan_score = next_plan.final_score(&valves_by_name);
                    println!("{}", best_plan_score);
                }
            } else if next_plan.upper_bound_score(&valves_by_name, &non_zero_valves)
                > best_plan_score
            {
                candidate_plans.push(next_plan);
            }
        }
    }
    best_plan_score
}

fn main() {
    let valves_by_name = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_1(valves_by_name, 30, "AA".to_string()));
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
        let valves_by_name = parse_input(AocBufReader::from_string("inputs/example.txt"));

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
            open_valves: HashSet::new(),
        };
        assert_eq!(plan.final_score(&valves_by_name), 1651);
    }
}
