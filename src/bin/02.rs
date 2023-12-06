#[macro_use]
extern crate lazy_static;

use regex::Regex;

advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

lazy_static! {
    static ref GAME_REGEX: Regex =
        Regex::new("Game ([0-9]+): (.+)").expect("Could not compile Regex");
    static ref RED_REGEX: Regex = Regex::new("([0-9]+) red").expect("Could not compile Regex");
    static ref GREEN_REGEX: Regex = Regex::new("([0-9]+) green").expect("Could not compile Regex");
    static ref BLUE_REGEX: Regex = Regex::new("([0-9]+) blue").expect("Could not compile Regex");
}

struct Round {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

struct Game {
    index: u32,
    rounds: Vec<Round>,
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let game_captures = GAME_REGEX.captures(line).unwrap();

            let game_index = game_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let game_line = GAME_REGEX.captures(line).unwrap().get(2).unwrap();

            let game_rounds = game_line
                .as_str()
                .split(';')
                .map(|game_round| {
                    let red = RED_REGEX.captures(game_round).and_then(|capture| {
                        Some(capture.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    });

                    let green = GREEN_REGEX.captures(game_round).and_then(|capture| {
                        Some(capture.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    });

                    let blue = BLUE_REGEX.captures(game_round).and_then(|capture| {
                        Some(capture.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    });

                    Round { red, green, blue }
                })
                .collect();

            Game {
                index: game_index,
                rounds: game_rounds,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = parse_games(input)
        .iter()
        .map(|game| {
            for round in &game.rounds {
                if MAX_RED < round.red.unwrap_or(0) {
                    return 0;
                }

                if MAX_BLUE < round.blue.unwrap_or(0) {
                    return 0;
                }

                if MAX_GREEN < round.green.unwrap_or(0) {
                    return 0;
                }
            }

            game.index
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_games(input)
            .iter()
            .map(|game| {
                let mut max_red = 0u32;
                let mut max_green = 0u32;
                let mut max_blue = 0u32;

                for round in &game.rounds {
                    max_red = std::cmp::max(max_red, round.red.unwrap_or(0));
                    max_green = std::cmp::max(max_green, round.green.unwrap_or(0));
                    max_blue = std::cmp::max(max_blue, round.blue.unwrap_or(0));
                }

                max_red * max_green * max_blue
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
