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

enum RobotType {
    OreCollecting,
    ObsidianCollecting,
    ClayCollecting,
    GeodeCracking,
}

enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    CrackedGeode,
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
