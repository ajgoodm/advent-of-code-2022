use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref BLUEPRINT_RE: Regex = Regex::new(
        r"^Blueprint ([0-9]*): Each ore robot costs ([^\.]*). Each clay robot costs ([^\.]*). Each obsidian robot costs ([^\.]*). Each geode robot costs ([^\.]*).$"
    ).unwrap();

    static ref COST_RE: Regex = Regex::new(
        r"^([0-9]*) ([^ ]*)$"
    ).unwrap();
}

struct ResourceCost {
    n_ore: usize,
    n_clay: usize,
    n_obsidian: usize,
}

struct Blueprint {
    id: usize,
    ore_collector_cost: ResourceCost,
    clay_collector_cost: ResourceCost,
    obsidian_collector_cost: ResourceCost,
    geode_cracker_cost: ResourceCost,
    maximum_ore_gatherers_needed: usize,
    maximum_clay_gatherers_needed: usize,
    maximum_obsidian_collectors_needed: usize,
}

impl Blueprint {
    fn new(
        id: usize,
        ore_collector_cost: ResourceCost,
        clay_collector_cost: ResourceCost,
        obsidian_collector_cost: ResourceCost,
        geode_cracker_cost: ResourceCost,
    ) -> Blueprint {
        let maximum_ore_gatherers_needed: usize = *vec![
            &ore_collector_cost.n_ore,
            &clay_collector_cost.n_ore,
            &obsidian_collector_cost.n_ore,
            &geode_cracker_cost.n_ore,
        ]
        .into_iter()
        .max()
        .unwrap();
        let maximum_clay_gatherers_needed: usize = *vec![
            &ore_collector_cost.n_clay,
            &clay_collector_cost.n_clay,
            &obsidian_collector_cost.n_clay,
            &geode_cracker_cost.n_clay,
        ]
        .into_iter()
        .max()
        .unwrap();
        let maximum_obsidian_collectors_needed: usize = *vec![
            &ore_collector_cost.n_obsidian,
            &clay_collector_cost.n_obsidian,
            &obsidian_collector_cost.n_obsidian,
            &geode_cracker_cost.n_obsidian,
        ]
        .into_iter()
        .max()
        .unwrap();

        Blueprint {
            id,
            ore_collector_cost,
            clay_collector_cost,
            obsidian_collector_cost,
            geode_cracker_cost,
            maximum_ore_gatherers_needed,
            maximum_clay_gatherers_needed,
            maximum_obsidian_collectors_needed,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SimulationState {
    t_minutes: usize,
    n_ore_possessed: usize,
    n_clay_possessed: usize,
    n_obsidian_possessed: usize,
    n_geodes_cracked: usize,
    n_ore_collectors_built: usize,
    n_obsidian_collectors_built: usize,
    n_clay_collectors_built: usize,
    n_geode_crackers_built: usize,
}

impl SimulationState {
    fn new_simulation() -> SimulationState {
        SimulationState {
            t_minutes: 0,
            n_ore_possessed: 0,
            n_clay_possessed: 0,
            n_obsidian_possessed: 0,
            n_geodes_cracked: 0,
            n_ore_collectors_built: 1,
            n_obsidian_collectors_built: 0,
            n_clay_collectors_built: 0,
            n_geode_crackers_built: 0,
        }
    }

    fn can_afford_robot(&self, resource_cost: &ResourceCost) -> bool {
        self.n_ore_possessed >= resource_cost.n_ore
            && self.n_clay_possessed >= resource_cost.n_clay
            && self.n_obsidian_possessed >= resource_cost.n_obsidian
    }

    fn buy_robot(&mut self, resource_cost: &ResourceCost) {
        self.n_ore_possessed -= resource_cost.n_ore;
        self.n_clay_possessed -= resource_cost.n_clay;
        self.n_obsidian_possessed -= resource_cost.n_obsidian;
    }

    fn buy_ore_collector(&self, blueprint: &Blueprint) -> SimulationState {
        let mut new_simulation = self.clone();
        new_simulation.buy_robot(&blueprint.ore_collector_cost);
        new_simulation.n_ore_collectors_built += 1;
        new_simulation
    }

    fn buy_clay_collector(&self, blueprint: &Blueprint) -> SimulationState {
        let mut new_simulation = self.clone();
        new_simulation.buy_robot(&blueprint.clay_collector_cost);
        new_simulation.n_clay_collectors_built += 1;
        new_simulation
    }

    fn buy_obsidian_collector(&self, blueprint: &Blueprint) -> SimulationState {
        let mut new_simulation = self.clone();
        new_simulation.buy_robot(&blueprint.obsidian_collector_cost);
        new_simulation.n_obsidian_collectors_built += 1;
        new_simulation
    }

    fn buy_geode_cracker(&self, blueprint: &Blueprint) -> SimulationState {
        let mut new_simulation = self.clone();
        new_simulation.buy_robot(&blueprint.geode_cracker_cost);
        new_simulation.n_geode_crackers_built += 1;
        new_simulation
    }

    /// Increment time; given the robots we have, update our resources
    fn time_passes(&mut self) {
        self.t_minutes += 1;
        self.n_ore_possessed += self.n_ore_collectors_built;
        self.n_clay_possessed += self.n_clay_collectors_built;
        self.n_obsidian_possessed += self.n_obsidian_collectors_built;
        self.n_geodes_cracked += self.n_geode_crackers_built;
    }

    /// Given the resources available, what could we do next?
    fn next_possible_states(&self, blueprint: &Blueprint) -> Vec<SimulationState> {
        let mut next_state_noop = self.clone();
        next_state_noop.time_passes();

        // If we have enough supporting robots to supply our factory
        // to make a geode cracker, every turn, we no longer need to make
        // any choices, the simulation is optimal from here forward.
        if self.n_ore_collectors_built >= blueprint.geode_cracker_cost.n_ore
            && self.n_clay_collectors_built >= blueprint.geode_cracker_cost.n_clay
            && self.n_obsidian_collectors_built >= blueprint.geode_cracker_cost.n_obsidian
        {
            return vec![next_state_noop];
        }

        let mut next_states: Vec<SimulationState> = Vec::new();
        if self.n_ore_collectors_built < blueprint.maximum_ore_gatherers_needed
            && self.can_afford_robot(&blueprint.ore_collector_cost)
        {
            next_states.push(next_state_noop.buy_ore_collector(&blueprint));
        }
        if self.n_clay_collectors_built < blueprint.maximum_clay_gatherers_needed
            && self.can_afford_robot(&blueprint.clay_collector_cost)
        {
            next_states.push(next_state_noop.buy_clay_collector(&blueprint));
        }
        if self.n_obsidian_collectors_built < blueprint.maximum_obsidian_collectors_needed
            && self.can_afford_robot(&blueprint.obsidian_collector_cost)
        {
            next_states.push(next_state_noop.buy_obsidian_collector(&blueprint));
        }
        if self.can_afford_robot(&blueprint.geode_cracker_cost) {
            next_states.push(next_state_noop.buy_geode_cracker(&blueprint));
        }
        next_states.push(next_state_noop);

        next_states
    }
}

fn _parse_cost(cost_str: &str) -> ResourceCost {
    let mut resource_cost: ResourceCost = ResourceCost {
        n_ore: 0,
        n_clay: 0,
        n_obsidian: 0,
    };
    let costs = cost_str.split(" and ");
    for cost in costs {
        let captures = COST_RE.captures(cost).unwrap();
        let cost_in_resource = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let resource = captures.get(2).unwrap().as_str();
        match resource {
            "ore" => {
                resource_cost.n_ore = cost_in_resource;
            }
            "clay" => {
                resource_cost.n_clay = cost_in_resource;
            }
            "obsidian" => {
                resource_cost.n_obsidian = cost_in_resource;
            }
            _ => {
                panic!("unexpected resource type {}", resource);
            }
        }
    }

    resource_cost
}

fn parse_line(line: String) -> Blueprint {
    let captures = BLUEPRINT_RE.captures(&line).unwrap();
    Blueprint::new(
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        _parse_cost(captures.get(2).unwrap().as_str()), // ore
        _parse_cost(captures.get(3).unwrap().as_str()), // clay
        _parse_cost(captures.get(4).unwrap().as_str()), // obsidian
        _parse_cost(captures.get(5).unwrap().as_str()), //
    )
}

fn parse_input(reader: AocBufReader) -> Vec<Blueprint> {
    reader.map(|line| parse_line(line)).collect()
}

fn maximum_geodes_cracked(blueprint: &Blueprint, n_steps: usize) -> usize {
    let mut univisited_states: HashSet<SimulationState> = vec![SimulationState::new_simulation()]
        .into_iter()
        .collect();
    let mut visited_states: HashSet<SimulationState> = HashSet::new();

    let mut max_geodes_cracked: usize = 0;
    while univisited_states.len() > 0 {
        let visited_state = univisited_states.iter().next().unwrap().clone();
        univisited_states.remove(&visited_state);
        visited_states.insert(visited_state.clone());

        let next_states = visited_state.next_possible_states(&blueprint);
        for candidate_state in next_states {
            if candidate_state.t_minutes >= n_steps {
                let geodes_cracked: usize = candidate_state.n_geodes_cracked;
                if geodes_cracked > max_geodes_cracked {
                    max_geodes_cracked = geodes_cracked;
                }
            } else if !visited_states.contains(&candidate_state) {
                let geodes_cracked: usize = candidate_state.n_geodes_cracked;
                let n_geode_crackers: usize = candidate_state.n_geode_crackers_built;
                let remaining_time: usize = n_steps - candidate_state.t_minutes;

                let max_score_possible: usize = geodes_cracked
                    + (n_geode_crackers..(n_geode_crackers + remaining_time)).sum::<usize>();
                if max_score_possible >= max_geodes_cracked {
                    univisited_states.insert(candidate_state);
                }
            }
        }
    }

    println!(
        "blueprint {} max cracked: {}",
        &blueprint.id, max_geodes_cracked
    );
    max_geodes_cracked
}

fn part_1(blue_prints: &Vec<Blueprint>, n_steps: usize) -> usize {
    blue_prints
        .iter()
        .map(|blueprint| maximum_geodes_cracked(blueprint, n_steps) * blueprint.id)
        .sum()
}

fn part_2(blue_prints: &[Blueprint], n_steps: usize) -> usize {
    blue_prints
        .iter()
        .map(|blueprint| maximum_geodes_cracked(blueprint, n_steps))
        .product()
}

fn main() {
    let blue_prints = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {}", part_1(&blue_prints, 24));
    println!("part 2: {}", part_2(&blue_prints[..3], 32));
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
