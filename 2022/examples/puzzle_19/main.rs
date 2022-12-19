use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

enum Robot {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

#[derive(Clone, Copy, Debug, Display, FromStr)]
#[display(r"Blueprint {index}: Each ore robot costs {ore_robot_ore_cost} ore. Each clay robot costs {clay_robot_ore_cost} ore. Each obsidian robot costs {obsidian_robot_ore_cost} ore and {obsidian_robot_clay_cost} clay. Each geode robot costs {geode_robot_ore_cost} ore and {geode_robot_obsidian_cost} obsidian.")]
struct Blueprint {
    index: u8,
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MiningState {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    num_ore_robots: u8,
    num_clay_robots: u8,
    num_obsidian_robots: u8,
    num_geode_robots: u8,
}

impl Default for MiningState {
    fn default() -> Self {
        MiningState {
            ore: 2,
            clay: 0,
            obsidian: 0,
            geode: 0,
            num_ore_robots: 1,
            num_clay_robots: 0,
            num_obsidian_robots: 0,
            num_geode_robots: 0,
        }
    }
}

impl MiningState {
    fn produce(&mut self, robot: Option<Robot>) {
        self.ore += self.num_ore_robots;
        self.clay += self.num_clay_robots;
        self.obsidian += self.num_obsidian_robots;
        self.geode += self.num_geode_robots;

        // correct for just made robot
        match robot {
            Some(Robot::ORE) => self.ore -= 1,
            Some(Robot::CLAY) => self.clay -= 1,
            Some(Robot::OBSIDIAN) => self.obsidian -= 1,
            Some(Robot::GEODE) => self.geode -= 1,
            _ => (),
        }
    }

    /// Weighted score for mining state using blueprint costs
    fn heuristic_score(&self, blueprint: &Blueprint) -> i64 {
        // never want to build more robots of type x than the most expensive cost of type x
        let ore_costs = vec![
            blueprint.clay_robot_ore_cost,
            blueprint.obsidian_robot_ore_cost,
            blueprint.geode_robot_ore_cost,
        ];
        let max_ore_cost = ore_costs.iter().max().unwrap();
        if self.num_ore_robots > *max_ore_cost {
            return i64::MIN;
        }
        if self.num_obsidian_robots > blueprint.geode_robot_obsidian_cost {
            return i64::MIN;
        }
        if self.num_clay_robots > blueprint.obsidian_robot_clay_cost {
            return i64::MIN;
        }

        let mut score: i64 = 0;
        score += 5 * self.geode as i64;
        score += 5 * self.num_geode_robots as i64;
        score += self.num_obsidian_robots as i64;
        score += self.num_clay_robots as i64;
        score += self.num_clay_robots as i64;
        score -= (self.ore + self.clay + self.obsidian) as i64;
        score
    }
}

fn update_geode_per_state_cache(
    cache: &mut HashMap<MiningState, u8>,
    state: &MiningState,
    next_states: &mut HashSet<MiningState>,
) {
    cache
        .entry(*state)
        .and_modify(|x| {
            if *x <= state.geode {
                next_states.insert(*state);
                *x = state.geode
            }
        })
        .or_insert_with(|| {
            next_states.insert(*state);
            state.geode
        });
}

fn simulate(blueprint: Blueprint, total_minutes: u8) -> (MiningState, u8) {
    // can skip two minutes since we need 2 ore to do anything else
    let mut current_minutes_elapsed = 2;
    let start_state = MiningState::default();
    let mut states: HashSet<MiningState> = HashSet::from([(start_state)]);
    let mut most_geodes_per_state: HashMap<MiningState, u8> = HashMap::new();
    most_geodes_per_state.insert(start_state, 0);

    while current_minutes_elapsed < total_minutes {
        let mut next_states: HashSet<MiningState> = HashSet::new();

        for state in states {
            if state.ore >= blueprint.ore_robot_ore_cost {
                let mut new_state = state.clone();
                new_state.num_ore_robots += 1;
                new_state.ore -= blueprint.ore_robot_ore_cost;
                new_state.produce(Some(Robot::ORE));
                update_geode_per_state_cache(
                    &mut most_geodes_per_state,
                    &new_state,
                    &mut next_states,
                );
            }
            if state.ore >= blueprint.clay_robot_ore_cost {
                let mut new_state = state.clone();
                new_state.num_clay_robots += 1;
                new_state.ore -= blueprint.clay_robot_ore_cost;
                new_state.produce(Some(Robot::CLAY));
                update_geode_per_state_cache(
                    &mut most_geodes_per_state,
                    &new_state,
                    &mut next_states,
                );
            }
            if state.ore >= blueprint.obsidian_robot_ore_cost
                && state.clay >= blueprint.obsidian_robot_clay_cost
            {
                let mut new_state = state.clone();
                new_state.num_obsidian_robots += 1;
                new_state.ore -= blueprint.obsidian_robot_ore_cost;
                new_state.clay -= blueprint.obsidian_robot_clay_cost;
                new_state.produce(Some(Robot::OBSIDIAN));
                update_geode_per_state_cache(
                    &mut most_geodes_per_state,
                    &new_state,
                    &mut next_states,
                );
            }
            if state.ore >= blueprint.geode_robot_ore_cost
                && state.obsidian >= blueprint.geode_robot_obsidian_cost
            {
                let mut new_state = state.clone();
                new_state.num_geode_robots += 1;
                new_state.ore -= blueprint.geode_robot_ore_cost;
                new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
                new_state.produce(Some(Robot::GEODE));
                update_geode_per_state_cache(
                    &mut most_geodes_per_state,
                    &new_state,
                    &mut next_states,
                );
            }
            // make nothing case
            let mut new_state = state.clone();
            new_state.produce(None);
            update_geode_per_state_cache(&mut most_geodes_per_state, &new_state, &mut next_states);
        }

        // use a heuristic to restrict the states we'll continue searching with
        states = HashSet::new();
        let sample_size = 200_000;

        if next_states.len() <= sample_size {
            states = next_states;
        } else {
            let mut scored_states = next_states
                .into_iter()
                .map(|s| (s.heuristic_score(&blueprint), s))
                .collect::<Vec<(i64, MiningState)>>();
            scored_states.sort_by(|a, b| b.0.cmp(&a.0));
            for (_, state) in scored_states.into_iter().take(sample_size) {
                states.insert(state);
            }
        }
        current_minutes_elapsed += 1;
    }

    most_geodes_per_state
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

fn part_1(contents: &str) -> u64 {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in contents.lines() {
        let blueprint: Blueprint = line.parse().unwrap();
        blueprints.push(blueprint);
    }

    let mut answer = 0;
    for blueprint in blueprints {
        let (state, geodes) = simulate(blueprint, 24);
        let quality_level = (geodes as u64) * (blueprint.index as u64);
        println!(
            "Blueprint {} has quality level {} for state {:?}",
            blueprint.index, quality_level, state
        );
        println!("");
        answer += quality_level;
    }
    answer
}

fn part_2(contents: &str) -> u64 {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in contents.lines().take(3) {
        let blueprint: Blueprint = line.parse().unwrap();
        blueprints.push(blueprint);
    }

    let mut answer: u64 = 1;
    for blueprint in blueprints {
        let (state, geodes) = simulate(blueprint, 32);
        println!(
            "Blueprint {} has best output state {:?}",
            blueprint.index, state
        );
        println!("");
        answer *= geodes as u64;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 33);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
