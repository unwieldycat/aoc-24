use std::{
    cmp::{self, Ordering},
    fs,
    i32::MAX,
};

use regex::Regex;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct OrderedPair {
    pub x: i32,
    pub y: i32,
}

impl OrderedPair {
    fn new(x: i32, y: i32) -> OrderedPair {
        OrderedPair { x, y }
    }
    fn sum(&self, other: &OrderedPair) -> OrderedPair {
        OrderedPair {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Ord for OrderedPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Cabinet {
    pub a_delta: OrderedPair,
    pub b_delta: OrderedPair,
    pub prize: OrderedPair,
}

fn load_input(path: &str) -> Vec<Cabinet> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut cabinets: Vec<Cabinet> = Vec::new();
    let delta_re = Regex::new(r"^X\+(\d+), Y\+(\d+)$").expect("Regex 1 broken");
    let coord_re = Regex::new(r"^X=(\d+), Y=(\d+)$").expect("Regex 2 broken");

    let mut curr_cabinet = Cabinet::default();
    for line in contents.lines() {
        if line.is_empty() {
            cabinets.push(curr_cabinet);
            curr_cabinet = Cabinet::default();
            continue;
        }

        if line.starts_with("Button A:") {
            let coord_str: &str = &line[10..];
            let coord_capture = delta_re.captures(coord_str).expect("Invalid string");
            let x = coord_capture[1].parse::<i32>().expect("X is not a number");
            let y = coord_capture[2].parse::<i32>().expect("Y is not a number");
            curr_cabinet.a_delta = OrderedPair { x, y };
        }

        if line.starts_with("Button B:") {
            let coord_str: &str = &line[10..];
            let coord_capture = delta_re.captures(coord_str).expect("Invalid string");
            let x = coord_capture[1].parse::<i32>().expect("X is not a number");
            let y = coord_capture[2].parse::<i32>().expect("Y is not a number");
            curr_cabinet.b_delta = OrderedPair { x, y };
        }

        if line.starts_with("Prize:") {
            let coord_str: &str = &line[7..];
            let coord_capture = coord_re.captures(coord_str).expect("Invalid string");
            let x = coord_capture[1].parse::<i32>().expect("X is not a number");
            let y = coord_capture[2].parse::<i32>().expect("Y is not a number");
            curr_cabinet.prize = OrderedPair { x, y };
        }
    }

    cabinets.push(curr_cabinet);

    cabinets
}

const A_COST: i32 = 3;
const B_COST: i32 = 1;
const MAX_ITER: i32 = 100;

fn get_prize(position: OrderedPair, cost: i32, i: i32, cabinet: &Cabinet) -> Option<i32> {
    if i >= MAX_ITER {
        return None;
    }

    if position == cabinet.prize {
        return Some(cost);
    } else if position > cabinet.prize {
        return None;
    }

    let a_pushed = get_prize(
        position.sum(&cabinet.a_delta),
        cost + A_COST,
        i + 1,
        cabinet,
    );
    let b_pushed = get_prize(
        position.sum(&cabinet.b_delta),
        cost + B_COST,
        i + 1,
        cabinet,
    );

    if a_pushed.is_none() && b_pushed.is_none() {
        return None;
    }

    return Some(cmp::min(a_pushed.unwrap_or(MAX), b_pushed.unwrap_or(MAX)));
}

fn puzzle1(cabinets: &Vec<Cabinet>) -> i32 {
    let mut cost = 0;
    for cabinet in cabinets {
        let res = get_prize(OrderedPair::new(0, 0), 0, 0, cabinet);
        if let Some(c) = res {
            cost += c;
        }
    }
    cost
}

fn puzzle2() {}

fn main() {
    let input = load_input("./test_input.txt");
    println!("Puzzle 1: {}", puzzle1(&input));
    //println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(480, puzzle1(&test_input));
    }

    // #[test]
    // fn test_puzzle2() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(1, puzzle2(&test_input));
    // }
}
