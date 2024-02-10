use crate::solution;

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    static ref COLOR_REGEX: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_captures = GAME_REGEX
            .captures(&s)
            .expect("the string must be in format Game <game_id>: <colors>");

        let id: u32 = game_captures[1].parse().unwrap();
        let sets_str = &game_captures[2];

        let sets = sets_str
            .split(";")
            .map(|set_str| {
                let (mut red, mut green, mut blue) = (0, 0, 0);

                for color_capture in COLOR_REGEX.captures_iter(set_str) {
                    let quantity: u32 = color_capture[1].parse().unwrap_or(0);

                    match &color_capture[2] {
                        "red" => red = quantity,
                        "green" => green = quantity,
                        "blue" => blue = quantity,
                        _ => (),
                    }
                }

                Set { red, green, blue }
            })
            .collect();

        Ok(Game { id, sets })
    }
}

pub struct Solution;

impl solution::Solution<2, Vec<String>, u32> for Solution {
    fn process_input(&self, input: String) -> Vec<String> {
        input
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(line.to_owned())
                }
            })
            .collect()
    }

    fn part_1(&self, input: &Vec<String>) -> u32 {
        let games: Vec<Game> = input.iter().map(|s| Game::from_str(s).unwrap()).collect();

        let (max_red, max_green, max_blue) = (12, 13, 14);

        games
            .iter()
            .filter_map(|game| {
                if game.sets.iter().all(|set| {
                    if set.red > max_red {
                        return false;
                    } else if set.green > max_green {
                        return false;
                    } else if set.blue > max_blue {
                        return false;
                    }

                    return true;
                }) {
                    return Some(game.id);
                }

                return None;
            })
            .sum()
    }

    fn part_2(&self, input: &Vec<String>) -> u32 {
        let games: Vec<Game> = input.iter().map(|s| Game::from_str(s).unwrap()).collect();

        1
    }
}
