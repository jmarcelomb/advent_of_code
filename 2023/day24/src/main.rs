use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug)]
struct SnowFlake {
    i: usize,

    px: f64,
    py: f64,
    pz: f64,

    vx: f64,
    vy: f64,
    vz: f64,

    m: f64,
    b: f64,
}

impl SnowFlake {
    fn new(snowflake_str: &str, index: usize) -> Self {
        // Example snow flake string: 19, 13, 30 @ -2,  1, -2
        let (positions, speeds) = snowflake_str.split_once("@").unwrap();

        let positions: Vec<f64> = positions
            .split(",")
            .map(|string| string.trim().parse::<f64>().unwrap())
            .collect();

        let speeds: Vec<f64> = speeds
            .split(",")
            .map(|string| string.trim().parse::<f64>().unwrap())
            .collect();

        let mut inst = SnowFlake {
            i: index,
            px: positions[0],
            py: positions[1],
            pz: positions[2],
            vx: speeds[0],
            vy: speeds[1],
            vz: speeds[2],
            m: 0.0,
            b: 0.0,
        };
        inst.m = inst.vy / inst.vx;
        inst.b = inst.py - (inst.m * inst.px);
        inst
    }
}

fn snowflakes_intersect(
    snowflake1: &SnowFlake,
    snowflake2: &SnowFlake,
    x_begin: f64,
    x_end: f64,
    y_begin: f64,
    y_end: f64,
) -> bool {
    let x_intersect = (snowflake2.b - snowflake1.b) / (snowflake1.m - snowflake2.m);
    let y_intersect = snowflake1.m * x_intersect + snowflake1.b;

    // println!(
    //     "snowflake 1: {:?}\nsnowflake 2: {:?}",
    //     snowflake1, snowflake2
    // );
    // println!("x_intersect: {:?}", x_intersect);
    // println!("y_intersect: {:?}", y_intersect);

    x_intersect.is_finite()
        && y_intersect.is_finite()
        && x_intersect >= x_begin
        && x_intersect <= x_end
        && y_intersect >= y_begin
        && y_intersect <= y_end
        && (snowflake1.px - x_intersect) * snowflake1.vx < 0.0
        && (snowflake2.px - x_intersect) * snowflake2.vx < 0.0
}

fn part1(file_name: &str, x_begin: f64, x_end: f64, y_begin: f64, y_end: f64) -> usize {
    let lines = read_lines(file_name);

    let mut snowflakes: Vec<SnowFlake> = vec![];

    for (line_i, line) in lines.clone().into_iter().enumerate() {
        let mut snowflake = SnowFlake::new(&line, line_i);
        snowflakes.push(snowflake)
    }

    let mut number_of_intersections = 0;
    for (i, snowflake) in snowflakes.iter().enumerate() {
        for to_compare_snowflake in snowflakes[i..].into_iter() {
            if snowflakes_intersect(
                &snowflake,
                to_compare_snowflake,
                x_begin,
                x_end,
                y_begin,
                y_end,
            ) {
                number_of_intersections += 1;
                // println!(
                //     "Snowflake:\n\tA: {:?}\n\tB: {:?}",
                //     snowflake, to_compare_snowflake
                // );
            }
            // println!();
        }
    }
    // println!("{:?}", snowflakes);
    number_of_intersections
}

// fn part2(file_name: &str) -> usize {
// }

fn main() {
    let part1_small_res = part1("small_input.txt", 7.0, 27.0, 7.0, 27.0);
    assert_eq!(part1_small_res, 2);

    let part1_res = part1(
        "input.txt",
        200000000000000.0,
        400000000000000.0,
        200000000000000.0,
        400000000000000.0,
    );

    //let part2_small_res = part2("small_input.txt");
    //assert_eq!(part2_small_res, 467835);

    //let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    //println!("Part 2 sum: {part2_res}");
}
