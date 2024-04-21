use std::fmt::Display;
use std::fs;

pub trait Solution<const DAY: u8, T, R: Display> {
    fn part_1(&self, input: &T) -> R;
    fn part_2(&self, input: &T) -> R;
    fn process_input(&self, input: String) -> T;

    fn load_input(&self) -> String {
        let path = format!("src/inputs/day_{:02}.txt", DAY);
        fs::read_to_string(&path).unwrap_or_else(|_| panic!("File must be readable: {}", &path))
    }

    fn run(&self) {
        let processed_input = self.process_input(self.load_input());

        let result_1 = self.part_1(&processed_input);
        println!("Result from day {} part 1: {}", DAY, result_1);

        let result_2 = self.part_2(&processed_input);
        println!("Result from day {} part 2: {}", DAY, result_2);
    }
}
