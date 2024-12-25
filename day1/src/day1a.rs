// Advent of Code 2024 - Day 1
// Author: Simon C.
// Date: 2024-12-24
// UTF-8

use std::fs::File;
use std::io::{self, BufRead};

pub fn calculate_total_distance(input_path: &str) -> io::Result<i32> {
    let file = File::open(&input_path)?;
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            left_list.push(left.parse::<i32>().expect("Invalid number in left column"));
            right_list.push(right.parse::<i32>().expect("Invalid number in right column"));
        }
    }

    left_list.sort();
    right_list.sort();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(total_distance)
}
