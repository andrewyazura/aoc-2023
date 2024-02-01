use crate::solution;

pub struct Solution;

impl solution::Solution<1, Vec<String>, u32> for Solution {
    fn process_input(&self, input: String) -> Vec<String> {
        input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect()
    }

    fn part_1(&self, input: &Vec<String>) -> u32 {
        input
            .iter()
            .map(|calibration_value| {
                let mut numbers = calibration_value
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10).unwrap_or_else(|| panic!("Unexpected error")));

                let first = numbers.next().unwrap_or_else(|| {
                    panic!("No numbers in calibration code {}", &calibration_value)
                });

                let last = numbers.last().unwrap_or(first);

                format!("{first}{last}").parse::<u32>().unwrap()
            })
            .sum()
    }

    fn part_2(&self, _input: &Vec<String>) -> u32 {
        0
    }
}
