use console::{Style, StyledObject, Term};
use core::time;
use regex::Regex;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Robot {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
}

impl Robot {
    fn fix_value(val: i32, upper_limit: i32) -> i32 {
        if val >= upper_limit {
            val % upper_limit
        } else if val < 0 {
            val + upper_limit
        } else {
            val
        }
    }

    fn sim_next(&mut self, field_size: (i32, i32)) {
        self.x = Self::fix_value(self.x + self.dx, field_size.0);
        self.y = Self::fix_value(self.y + self.dy, field_size.1);
    }

    fn sim_prev(&mut self, field_size: (i32, i32)) {
        self.x = Self::fix_value(self.x - self.dx, field_size.0);
        self.y = Self::fix_value(self.y - self.dy, field_size.1);
    }
}

fn load_input(path: &str) -> Vec<Robot> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let regex = Regex::new(r"p=([\d-]+),([\d-]+) v=([\d-]+),([\d-]+)").expect("Regex error");
    let mut robots: Vec<Robot> = Vec::new();
    for line in contents.lines() {
        let captures = regex.captures(line).expect("Capture error");
        let x = captures[1].parse::<i32>().unwrap();
        let y = captures[2].parse::<i32>().unwrap();
        let dx = captures[3].parse::<i32>().unwrap();
        let dy = captures[4].parse::<i32>().unwrap();
        robots.push(Robot { x, y, dx, dy });
    }
    robots
}

const TEST_MAP: (i32, i32) = (11, 7);
const REAL_MAP: (i32, i32) = (101, 103);
const ITERATIONS: i32 = 100;

fn puzzle1(field_size: (i32, i32), initial_bots: &Vec<Robot>) -> i32 {
    let mut robots: Vec<Robot> = initial_bots.clone();
    // 0 | 1
    // -----
    // 2 | 3
    let mut quads: (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mid_x = (field_size.0) / 2;
    let mid_y = (field_size.1) / 2;
    for robot in &mut robots {
        for _ in 0..ITERATIONS {
            robot.sim_next(field_size);
        }
        if robot.x < mid_x && robot.y < mid_y {
            quads.0 += 1;
        } else if robot.x > mid_x && robot.y < mid_y {
            quads.1 += 1;
        } else if robot.x < mid_x && robot.y > mid_y {
            quads.2 += 1;
        } else if robot.x > mid_x && robot.y > mid_y {
            quads.3 += 1;
        }
    }

    quads.0 * quads.1 * quads.2 * quads.3
}

fn puzzle2(field_size: (i32, i32), initial_bots: &Vec<Robot>) {
    // State
    let mut robots: Vec<Robot> = initial_bots.clone();
    let mut i = 0;

    // Terminal goodies
    let empty_space = Style::new().black();
    let occupied_space = Style::new().yellow().bright().blink();

    'outer: loop {
        for robot in &mut robots {
            robot.sim_next(field_size);
        }

        i += 1;

        let default_inner = vec![0; field_size.0 as usize];
        let mut bot_map = vec![default_inner.clone(); field_size.1 as usize];

        for robot in &robots {
            if bot_map[robot.y as usize][robot.x as usize] > 0 {
                continue 'outer;
            }
            bot_map[robot.y as usize][robot.x as usize] += 1;
        }

        let map_as_str = bot_map
            .iter()
            .map(|v| {
                v.iter()
                    .map(|n| {
                        if *n == 0 {
                            empty_space.apply_to(".").to_string()
                        } else {
                            occupied_space.apply_to(n.to_string()).to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", map_as_str);
        println!("seconds: {}", i);
        break;
    }
}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(REAL_MAP, &input));
    println!("Puzzle 2:");
    puzzle2(REAL_MAP, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(12, puzzle1(TEST_MAP, &test_input));
    }
}
