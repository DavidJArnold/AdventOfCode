use memoize::memoize;

const REAL_FILENAME: &str = "19.real.txt";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blueprint {
    ore_ore: u16,
    ore_cla: u16,
    ore_obs: u16,
    cla_obs: u16,
    ore_geo: u16,
    obs_geo: u16,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    ore: u16,
    cla: u16,
    obs: u16,
    geo: u16,
    ore_bots: u16,
    cla_bots: u16,
    obs_bots: u16,
    geo_bots: u16,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut output: Vec<Blueprint> = vec![];
    for line in input.split('\n') {
        let nums: Vec<u16> = line
            .split(' ')
            .filter_map(|x| x.parse::<u16>().ok())
            .collect();
        output.push(Blueprint {
            ore_ore: nums[0],
            ore_cla: nums[1],
            ore_obs: nums[2],
            cla_obs: nums[3],
            ore_geo: nums[4],
            obs_geo: nums[5],
        });
    }
    output
}

#[memoize]
fn score_blueprint(
    blueprint: Blueprint,
    max_time: u16,
    current_time: u16,
    state: State,
    best_score: u16,
) -> u16 {
    let max_ore = blueprint
        .ore_geo
        .max(blueprint.ore_obs)
        .max(blueprint.ore_cla)
        .max(blueprint.ore_ore);

    if current_time + 1 == max_time {
        return state.geo + state.geo_bots;
    }

    let ub = state.geo + state.geo_bots * (max_time - current_time);

    let mut score: u16;
    let mut new_best_score: u16 = best_score;
    let mut build_anything = false;

    if state.ore_bots < max_ore {
        // build ore bots if creating less per turn than max ore cost of a bot
        let time_until_build = 0.max(
            ((blueprint.ore_ore as f32 - state.ore as f32) / state.ore_bots as f32).ceil() as u16,
        ) + 1;
        let time_left = max_time as i16 - current_time as i16 - time_until_build as i16;
        if time_left > 0 && ub + ((time_left * (time_left + 1) / 2) as u16) > best_score {
            let temp = State {
                ore: state.ore + time_until_build * state.ore_bots - blueprint.ore_ore,
                cla: state.cla + time_until_build * state.cla_bots,
                obs: state.obs + time_until_build * state.obs_bots,
                geo: state.geo + time_until_build * state.geo_bots,
                ore_bots: state.ore_bots + 1,
                cla_bots: state.cla_bots,
                obs_bots: state.obs_bots,
                geo_bots: state.geo_bots,
            };
            score = score_blueprint(
                blueprint,
                max_time,
                current_time + time_until_build,
                temp,
                best_score,
            );

            new_best_score = new_best_score.max(score);
            build_anything = true;
        }
    }

    if state.cla_bots < blueprint.cla_obs {
        // build clay bot if required
        let time_until_build = 0.max(
            ((blueprint.ore_cla as f32 - state.ore as f32) / state.ore_bots as f32).ceil() as u16,
        ) + 1;
        let time_left = max_time as i16 - current_time as i16 - time_until_build as i16;
        if time_left > 0 && ub + ((time_left * (time_left + 1) / 2) as u16) > best_score {
            let temp = State {
                ore: state.ore + time_until_build * state.ore_bots - blueprint.ore_cla,
                cla: state.cla + time_until_build * state.cla_bots,
                obs: state.obs + time_until_build * state.obs_bots,
                geo: state.geo + time_until_build * state.geo_bots,
                ore_bots: state.ore_bots,
                cla_bots: state.cla_bots + 1,
                obs_bots: state.obs_bots,
                geo_bots: state.geo_bots,
            };
            score = score_blueprint(
                blueprint,
                max_time,
                current_time + time_until_build,
                temp,
                best_score,
            );

            new_best_score = new_best_score.max(score);
            build_anything = true;
        }
    }

    if state.cla_bots > 0 && state.obs < blueprint.obs_geo {
        let time_until_build =
            0.max(
                ((blueprint.ore_obs as f32 - state.ore as f32) / state.ore_bots as f32).ceil()
                    as u16,
            )
            .max(
                ((blueprint.cla_obs as f32 - state.cla as f32) / state.cla_bots as f32).ceil()
                    as u16,
            ) + 1;
        let time_left = max_time as i16 - current_time as i16 - time_until_build as i16;
        if time_left > 0 && ub + ((time_left * (time_left + 1) / 2) as u16) > best_score {
            let temp = State {
                ore: state.ore + time_until_build * state.ore_bots - blueprint.ore_obs,
                cla: state.cla + time_until_build * state.cla_bots - blueprint.cla_obs,
                obs: state.obs + time_until_build * state.obs_bots,
                geo: state.geo + time_until_build * state.geo_bots,
                ore_bots: state.ore_bots,
                cla_bots: state.cla_bots,
                obs_bots: state.obs_bots + 1,
                geo_bots: state.geo_bots,
            };
            score = score_blueprint(
                blueprint,
                max_time,
                current_time + time_until_build,
                temp,
                best_score,
            );

            new_best_score = new_best_score.max(score);
            build_anything = true;
        }
    }

    if state.obs_bots > 0 {
        //build geode
        let time_until_build =
            0.max(
                ((blueprint.obs_geo as f32 - state.obs as f32) / state.obs_bots as f32).ceil()
                    as u16,
            )
            .max(
                ((blueprint.ore_geo as f32 - state.ore as f32) / state.ore_bots as f32).ceil()
                    as u16,
            ) + 1;
        let time_left = max_time as i16 - current_time as i16 - time_until_build as i16;
        if time_left > 0 && ub + ((time_left * (time_left + 1) / 2) as u16) > best_score {
            let temp = State {
                ore: state.ore + time_until_build * state.ore_bots - blueprint.ore_geo,
                cla: state.cla + time_until_build * state.cla_bots,
                obs: state.obs + time_until_build * state.obs_bots - blueprint.obs_geo,
                geo: state.geo + time_until_build * state.geo_bots,
                ore_bots: state.ore_bots,
                cla_bots: state.cla_bots,
                obs_bots: state.obs_bots,
                geo_bots: state.geo_bots + 1,
            };
            score = score_blueprint(
                blueprint,
                max_time,
                current_time + time_until_build,
                temp,
                best_score,
            );

            new_best_score = new_best_score.max(score);
            build_anything = true;
        }
    }

    if !build_anything {
        score = state.geo + (max_time - current_time) * state.geo_bots;
        new_best_score = best_score.max(score);
    }

    new_best_score
}

fn part1(filename: &str) -> u16 {
    let mut input = std::fs::read_to_string(filename).unwrap();
    input = input.strip_suffix('\n').unwrap().to_string();
    let blueprints = parse_input(&input);

    let mut quality = 0;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        let state = State {
            ore: 0,
            cla: 0,
            obs: 0,
            geo: 0,
            ore_bots: 1,
            cla_bots: 0,
            obs_bots: 0,
            geo_bots: 0,
        };
        let score = score_blueprint(*blueprint, 24, 0, state, 0);
        quality += (idx as u16 + 1) * score;
    }
    quality
}

fn part2(filename: &str) -> u16 {
    let mut input = std::fs::read_to_string(filename).unwrap();
    input = input.strip_suffix('\n').unwrap().to_string();
    let blueprints = parse_input(&input);

    let mut scores = 1;
    for blueprint in blueprints.iter().take(3.min(blueprints.len())) {
        let state = State {
            ore: 0,
            cla: 0,
            obs: 0,
            geo: 0,
            ore_bots: 1,
            cla_bots: 0,
            obs_bots: 0,
            geo_bots: 0,
        };
        let score = score_blueprint(*blueprint, 32, 0, state, 0);
        scores *= score;
    }
    scores
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "19.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 33)
    }

    #[test]
    fn test_parsing() {
        let mut input = std::fs::read_to_string(TEST_FILENAME).unwrap();
        input = input.strip_suffix('\n').unwrap().to_string();
        let bps = parse_input(&input);
        let bp0 = Blueprint {
            ore_ore: 4,
            ore_cla: 2,
            ore_obs: 3,
            cla_obs: 14,
            ore_geo: 2,
            obs_geo: 7,
        };
        assert_eq!(bps[0], bp0)
    }

    #[test]
    fn test_calc() {
        let mut input = std::fs::read_to_string(TEST_FILENAME).unwrap();
        input = input.strip_suffix('\n').unwrap().to_string();
        let blueprints = parse_input(&input);

        let state = State {
            ore: 0,
            cla: 0,
            obs: 0,
            geo: 0,
            ore_bots: 1,
            cla_bots: 0,
            obs_bots: 0,
            geo_bots: 0,
        };
        let score = score_blueprint(blueprints[0], 24, 0, state, 0);
        assert_eq!(score, 9);
    }
}
