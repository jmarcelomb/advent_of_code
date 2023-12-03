use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Play {
    blue: usize,
    red: usize,
    green: usize,
}

impl Play {
    fn default() -> Self {
        Play {
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    fn parse(cube_str: &str) -> Self {
        let mut play = Play::default();

        if cube_str.contains("blue") {
            play.blue = cube_str
                .replace("blue", "")
                .trim()
                .parse::<usize>()
                .unwrap();
        } else if cube_str.contains("red") {
            play.red = cube_str.replace("red", "").trim().parse::<usize>().unwrap();
        } else if cube_str.contains("green") {
            play.green = cube_str
                .replace("green", "")
                .trim()
                .parse::<usize>()
                .unwrap();
        }
        play
    }

    fn get_minimum_cubes_from_str(game_str: &str) -> Self {
        let mut play = Play::default();

        for play_str in game_str.split(";") {
            for cube_str in play_str.split(",") {
                let mut blue: usize = 0;
                let mut red: usize = 0;
                let mut green: usize = 0;

                if cube_str.contains("blue") {
                    blue = cube_str
                        .replace("blue", "")
                        .trim()
                        .parse::<usize>()
                        .unwrap();
                } else if cube_str.contains("red") {
                    red = cube_str.replace("red", "").trim().parse::<usize>().unwrap();
                } else if cube_str.contains("green") {
                    green = cube_str
                        .replace("green", "")
                        .trim()
                        .parse::<usize>()
                        .unwrap();
                }
                if blue > play.blue {
                    play.blue = blue;
                }
                if red > play.red {
                    play.red = red;
                }
                if green > play.green {
                    play.green = green;
                }
            }
        }
        play
    }
}

fn part1(file_name: &str, max_blue: usize, max_red: usize, max_green: usize) -> usize {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut sum: usize = 0;

    for (i, line_res) in reader.lines().enumerate() {
        let game_number: usize = i + 1;
        let line = line_res.unwrap();
        let colon_pos = line.find(":").unwrap();
        let mut valid_play = true;

        for play_str in line[colon_pos + 1..].split(";") {
            for cube_str in play_str.split(",") {
                let play = Play::parse(cube_str);
                if play.blue > max_blue || play.red > max_red || play.green > max_green {
                    valid_play = false;
                    break;
                }
            }
        }
        if valid_play {
            sum += game_number;
        }
    }
    sum
}

fn part2(file_name: &str) -> usize {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut sum: usize = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let colon_pos = line.find(":").unwrap();

        let play = Play::get_minimum_cubes_from_str(&line[colon_pos + 1..]);
        sum += play.blue * play.red * play.green;
    }
    sum
}

fn main() {
    let part1_res = part1("input.txt", 14, 12, 13);
    let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}
