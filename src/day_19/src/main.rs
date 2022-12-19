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
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct SimulationState {
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

    /// Given the robots we have, update our resources
    fn collect_resources(&mut self) {
        self.n_ore_possessed += self.n_ore_collectors_built;
        self.n_clay_possessed += self.n_clay_collectors_built;
        self.n_obsidian_possessed += self.n_obsidian_collectors_built;
        self.n_geodes_cracked += self.n_geode_crackers_built;
    }

    /// Given the resources available, what could we do next?
    fn next_possible_states(&self, blueprint: &Blueprint) -> Vec<SimulationState> {
        let mut next_state_noop = self.clone();
        next_state_noop.collect_resources();

        let mut next_states: Vec<SimulationState> = Vec::new();
        if self.can_afford_robot(&blueprint.ore_collector_cost) {
            next_states.push(next_state_noop.buy_ore_collector(&blueprint));
        }
        if self.can_afford_robot(&blueprint.clay_collector_cost) {
            next_states.push(next_state_noop.buy_clay_collector(&blueprint));
        }
        if self.can_afford_robot(&blueprint.obsidian_collector_cost) {
            next_states.push(next_state_noop.buy_obsidian_collector(&blueprint));
        }
        if self.can_afford_robot(&blueprint.geode_cracker_cost) {
            next_states.push(next_state_noop.buy_geode_cracker(&blueprint));
        }
        // we don't have to buy anything... sometimes we can't!
        let mut next_states = vec![next_state_noop];

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
    Blueprint {
        id: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        ore_collector_cost: _parse_cost(captures.get(2).unwrap().as_str()),
        clay_collector_cost: _parse_cost(captures.get(2).unwrap().as_str()),
        obsidian_collector_cost: _parse_cost(captures.get(2).unwrap().as_str()),
        geode_cracker_cost: _parse_cost(captures.get(2).unwrap().as_str()),
    }
}

fn parse_input(reader: AocBufReader) -> Vec<Blueprint> {
    reader.map(|line| parse_line(line)).collect()
}

fn main() {
    println!("Hello, world!");
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
