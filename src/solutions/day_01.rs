use crate::solution;

pub struct Solution;

impl solution::Solution<1, Vec<String>, i128> for Solution {
    fn process_input(&self, input: &str) -> Vec<String> {
        input.split('\n').map(|s| s.to_owned()).collect()
    }

    fn part_1(&self, input: &Vec<String>) -> i128 {
        0
    }

    fn part_2(&self, input: &Vec<String>) -> i128 {
        0
    }
}
