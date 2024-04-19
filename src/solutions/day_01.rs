use crate::solution;

pub struct Solution;

impl solution::Solution<1, Vec<String>, u32> for Solution {
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

    fn part_2(&self, input: &Vec<String>) -> u32 {
        const NUMBERS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        input
            .iter()
            .map(|calibration_value| {
                let mut numbers =
                    calibration_value
                        .chars()
                        .enumerate()
                        .filter_map(|(i, c)| match c {
                            c if c.is_numeric() => c.to_digit(10),
                            _ => {
                                for n in NUMBERS {
                                    if calibration_value[i..].starts_with(n) {
                                        return text_to_int(&calibration_value[i..i + n.len()]);
                                    }
                                }

                                return None;
                            }
                        });

                let first = numbers.next().unwrap_or_else(|| {
                    panic!("No numbers in calibration code {}", &calibration_value)
                });

                let last = numbers.last().unwrap_or(first);

                format!("{first}{last}").parse::<u32>().unwrap()
            })
            .sum()
    }
}

fn text_to_int(text: &str) -> Option<u32> {
    match text {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution as SolutionTrait;
    use crate::solutions::day_01::Solution;

    const SOLUTION: Solution = Solution;

    #[test]
    fn test_part_1() {
        const SAMPLE_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        let input = SOLUTION.process_input(SAMPLE_INPUT.to_owned());
        assert_eq!(SOLUTION.part_1(&input), 142);
    }

    #[test]
    fn test_part_2() {
        const SAMPLE_INPUT: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        let input = SOLUTION.process_input(SAMPLE_INPUT.to_owned());
        assert_eq!(SOLUTION.part_2(&input), 281);
    }
}
