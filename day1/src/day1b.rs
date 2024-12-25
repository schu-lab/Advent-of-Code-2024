// Advent of Code 2024 - Day 1
// Author: Simon C.
// Date: 2024-12-24
// UTF-8

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn calculate_similarity_score(input_path: &str) -> io::Result<i32> {
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

    let mut right_count: HashMap<i32, i32> = HashMap::new();
    for &num in &right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * right_count.get(&num).unwrap_or(&0))
        .sum();

    Ok(similarity_score)
}
