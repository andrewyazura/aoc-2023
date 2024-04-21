use regex::Regex;
use std::collections::HashMap;

use crate::solution;

pub struct EngineSchematic {
    numbers: Vec<(u32, usize, usize, usize)>,
    symbols: Vec<(usize, usize)>,
    gears: Vec<(usize, usize)>,
}

pub struct Solution;

impl solution::Solution<3, EngineSchematic, u32> for Solution {
    fn process_input(&self, input: String) -> EngineSchematic {
        let clean_input: Vec<&str> = input
            .lines()
            .filter_map(|line| match line.is_empty() {
                true => None,
                false => Some(line),
            })
            .collect();

        let integer_regex = Regex::new(r"\d+").unwrap();

        let numbers = clean_input
            .iter()
            .enumerate()
            .filter_map(|(index, line)| {
                let line_numbers: Vec<(u32, usize, usize, usize)> = integer_regex
                    .find_iter(line)
                    .map(|m| {
                        let (start, end) = (m.start(), m.end());
                        let number = m.as_str().parse().unwrap();
                        (number, index, start, end)
                    })
                    .collect();

                match line_numbers.is_empty() {
                    true => None,
                    false => Some(line_numbers),
                }
            })
            .flatten()
            .collect();

        let symbols = clean_input
            .iter()
            .enumerate()
            .flat_map(|(index, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(
                        |(char_index, char)| match !char.is_digit(10) && char != '.' {
                            true => Some((index, char_index)),
                            false => None,
                        },
                    )
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        let gears = clean_input
            .iter()
            .enumerate()
            .flat_map(|(index, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(char_index, char)| match char {
                        '*' => Some((index, char_index)),
                        _ => None,
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        EngineSchematic {
            numbers,
            symbols,
            gears,
        }
    }

    fn part_1(&self, input: &EngineSchematic) -> u32 {
        input
            .numbers
            .iter()
            .flat_map(|(number, line, start, end)| {
                (line.saturating_sub(1)..=line.saturating_add(1)).flat_map(move |line_index| {
                    (start.saturating_sub(1)..end.saturating_add(1)).map(move |char_index| {
                        match input.symbols.contains(&(line_index, char_index)) {
                            true => *number,
                            false => 0,
                        }
                    })
                })
            })
            .sum()
    }

    fn part_2(&self, input: &EngineSchematic) -> u32 {
        let mut gear_collisions: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        for (number, line, start, end) in &input.numbers {
            for line_index in line.saturating_sub(1)..=line.saturating_add(1) {
                for char_index in start.saturating_sub(1)..end.saturating_add(1) {
                    if input.gears.contains(&(line_index, char_index)) {
                        gear_collisions
                            .entry((line_index, char_index))
                            .or_insert_with(Vec::new)
                            .push(*number);
                    }
                }
            }
        }

        gear_collisions
            .iter()
            .filter_map(|(_, numbers)| match numbers.len() {
                2 => Some(numbers[0] * numbers[1]),
                _ => None,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution as SolutionTrait;
    use crate::solutions::day_03::Solution;

    const SOLUTION: Solution = Solution;
    const SAMPLE_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_1() {
        let input = SOLUTION.process_input(SAMPLE_INPUT.to_string());
        assert_eq!(SOLUTION.part_1(&input), 4361);
    }

    #[test]
    fn test_part_2() {
        let input = SOLUTION.process_input(SAMPLE_INPUT.to_string());
        assert_eq!(SOLUTION.part_2(&input), 467835);
    }
}
