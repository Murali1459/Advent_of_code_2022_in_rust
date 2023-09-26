pub mod day_three;

use std::fs;

const INPUT_FILE: &'static str = "inputs/input.txt";

fn main() {
    let file: String = fs::read_to_string(INPUT_FILE).expect("Error Reading file");
    let contents: Vec<&str> = file.split("\n").collect();
    // println!("{:?}", contents);
    let solution = day_three::solution::part2(contents);
    println!("{:?}", solution);
}
