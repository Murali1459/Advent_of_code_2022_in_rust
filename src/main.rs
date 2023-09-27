pub mod day_four;

use std::fs;

const INPUT_FILE: &'static str = "inputs/input.txt";

fn main() {
    let file: String = fs::read_to_string(INPUT_FILE).expect("Error Reading file");
    let mut contents: Vec<&str> = file.split("\n").collect();
    contents.pop();
    // println!("{:?}", contents);
    let solution = day_four::solution::part2(contents);
    println!("{:?}", solution);
}
