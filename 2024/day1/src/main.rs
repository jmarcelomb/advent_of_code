use itertools::izip;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_file(file_name: &str) -> io::Result<(Vec<i64>, Vec<i64>)> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut left_array = Vec::new();
    let mut right_array = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok()); // Parse numbers safely
        if let (Some(left), Some(right)) = (numbers.next(), numbers.next()) {
            left_array.push(left);
            right_array.push(right);
        } else {
            eprintln!("Warning: Skipping malformed line: {}", line);
        }
    }

    Ok((left_array, right_array))
}

fn part1(file_name: &str) -> Option<i64> {
    match parse_file(file_name) {
        Ok((mut left_array, mut right_array)) => {
            // println!("Left column: {:?}", left_array);
            // println!("Right column: {:?}", right_array);
            left_array.sort();
            right_array.sort();
            // println!("Sorted Left column: {:?}", left_array);
            // println!("Sorted Right column: {:?}", right_array);
            let mut diffencens_sum = 0;
            for (left, right) in izip!(&left_array, &right_array) {
                let difference = (left - right).abs();
                diffencens_sum += difference;
            }
            Some(diffencens_sum)
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_name, e);
            None
        }
    }
}

fn part2(file_name: &str) -> Option<usize> {
    match parse_file(file_name) {
        Ok((left_array, right_array)) => {
            let mut similarity_sum = 0;
            for left_number in left_array {
                let occurencies = right_array.iter().filter(|&n| *n == left_number).count();
                similarity_sum += left_number as usize * occurencies;
            }
            Some(similarity_sum)
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_name, e);
            None
        }
    }
}

fn main() {
    let res = part1("small_input.txt").unwrap();
    assert!(res == 11);
    let res = part1("input.txt").unwrap();
    println!("Part 1 result is {res}");

    let res = part2("small_input.txt").unwrap();
    assert!(res == 31);
    let res = part2("input.txt").unwrap();
    println!("Part 2 result is {res}");
}
