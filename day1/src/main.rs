// Advent of Code 2024 - Day 1
// Author: Simon C.
// Date: 2024-12-24
// UTF-8

mod day1a;
mod day1b;

fn main() {
    let input_path = "src/day1.txt";

    match day1a::calculate_total_distance(input_path) {
        Ok(total_distance) => println!("Day1A - Total distance: {}", total_distance),
        Err(e) => eprintln!("Day1A - Error calculating total distance: {}", e),
    }

    match day1b::calculate_similarity_score(input_path) {
        Ok(similarity_score) => println!("Day1B - Similarity score: {}", similarity_score),
        Err(e) => eprintln!("Day1B - Error calculating similarity score: {}", e),
    }
}
