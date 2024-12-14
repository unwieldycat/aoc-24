use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    fs,
    i32::MAX,
};

use regex::Regex;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
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

fn get_prize(
    position: OrderedPair,
    cost: i32,
    a_i: i32,
    b_i: i32,
    cabinet: &Cabinet,
    cache: &mut HashMap<(OrderedPair, i32, i32), Option<i32>>,
) -> Option<i32> {
    if let Some(cached_val) = cache.get(&(position, a_i, b_i)) {
        return *cached_val;
    }

    if a_i > MAX_ITER || b_i > MAX_ITER {
        return None;
    }

    if position == cabinet.prize {
        cache.insert((position, a_i, b_i), Some(cost));
        return Some(cost);
    } else if position > cabinet.prize {
        cache.insert((position, a_i, b_i), None);
        return None;
    }

    let a_pushed = get_prize(
        position.sum(&cabinet.a_delta),
        cost + A_COST,
        a_i + 1,
        b_i,
        cabinet,
        cache,
    );
    let b_pushed = get_prize(
        position.sum(&cabinet.b_delta),
        cost + B_COST,
        a_i,
        b_i + 1,
        cabinet,
        cache,
    );

    if a_pushed.is_none() && b_pushed.is_none() {
        cache.insert((position, a_i, b_i), None);
        return None;
    }

    let retval = Some(cmp::min(a_pushed.unwrap_or(MAX), b_pushed.unwrap_or(MAX)));
    cache.insert((position, a_i, b_i), retval);
    return retval;
}

fn puzzle1(cabinets: &Vec<Cabinet>) -> i32 {
    let mut cost = 0;
    for cabinet in cabinets {
        let mut cache: HashMap<(OrderedPair, i32, i32), Option<i32>> = HashMap::new();
        let res = get_prize(OrderedPair::new(0, 0), 0, 0, 0, cabinet, &mut cache);
        if let Some(c) = res {
            cost += c;
        }
    }
    cost
}

fn solve_system(a: f64, b: f64, n1: f64, c: f64, d: f64, n2: f64) -> Option<f64> {
    // Ax + By = N1
    // Cx + Dy = N2
    // m = N2 / N1
    let m = n2 / n1;

    // mAx + mBy = mN1
    // mAx + mBy = N2
    // mAx + mBy = Cx + Dy
    // mAx - Cx = Dy - mBy
    // x(mA - C) = y(D - mB)
    // x(mA - C) / (D - mB) = y
    // Ax + B(x(mA - C) / (D - mB)) = N1
    // x(A + B((mA - C) / (D - mB))) = N1
    // x = N1 / (A + B((mA - C) / (D - mB)))
    let x = n1 / (a + b * (((m * a) - c) / (d - (m * b))));

    // x == [[x]]
    if (x - x.round()).abs() < 0.0000001 {
        // By = N1 - Ax
        // y = (N1 - Ax) / B
        let y = (n1 - (a * x)) / b;

        return Some(x.round() * A_COST as f64 + y.round() * B_COST as f64);
    }

    return None;
}

fn puzzle2(cabinets: &Vec<Cabinet>) -> i64 {
    let mut cost = 0;
    for cabinet in cabinets {
        if let Some(res) = solve_system(
            cabinet.a_delta.x as f64,
            cabinet.b_delta.x as f64,
            (cabinet.prize.x as i64 + 10000000000000) as f64,
            cabinet.a_delta.y as f64,
            cabinet.b_delta.y as f64,
            (cabinet.prize.y as i64 + 10000000000000) as f64,
        ) {
            cost += res as i64;
        }
    }
    cost
}

fn main() {
    let input = load_input("./input.txt");

    //println!("Puzzle 1: {}", puzzle1(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(480, puzzle1(&test_input));
    }
}
