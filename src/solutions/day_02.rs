use crate::solution;

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

struct Game {
    id: u32,
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>,
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

        let (mut reds, mut blues, mut greens) = (vec![], vec![], vec![]);

        for set in sets_str.split(";") {
            let (mut red, mut green, mut blue) = (0, 0, 0);

            for color_capture in COLOR_REGEX.captures_iter(set) {
                let quantity: u32 = color_capture[1].parse().unwrap_or(0);

                match &color_capture[2] {
                    "red" => red = quantity,
                    "green" => green = quantity,
                    "blue" => blue = quantity,
                    _ => (),
                }
            }

            reds.push(red);
            greens.push(green);
            blues.push(blue);
        }

        Ok(Game {
            id,
            reds,
            greens,
            blues,
        })
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
                if game
                    .reds
                    .iter()
                    .zip(game.greens.iter())
                    .zip(game.blues.iter())
                    .all(|((&r, &g), &b)| {
                        if r > max_red {
                            return false;
                        } else if g > max_green {
                            return false;
                        } else if b > max_blue {
                            return false;
                        }

                        return true;
                    })
                {
                    return Some(game.id);
                }

                return None;
            })
            .sum()
    }

    fn part_2(&self, input: &Vec<String>) -> u32 {
        let games: Vec<Game> = input.iter().map(|s| Game::from_str(s).unwrap()).collect();

        games
            .iter()
            .map(|game| {
                game.reds.iter().max().unwrap()
                    * game.greens.iter().max().unwrap()
                    * game.blues.iter().max().unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution as SolutionTrait;
    use crate::solutions::day_02::Solution;

    const SOLUTION: Solution = Solution;
    const SAMPLE_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part_1() {
        let input = SOLUTION.process_input(SAMPLE_INPUT.to_owned());
        assert_eq!(SOLUTION.part_1(&input), 8);
    }

    #[test]
    fn test_part_2() {
        let input = SOLUTION.process_input(SAMPLE_INPUT.to_owned());
        assert_eq!(SOLUTION.part_2(&input), 2286);
    }
}
