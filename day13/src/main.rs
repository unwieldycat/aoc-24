use regex::Regex;
use std::fs;

#[derive(Default, Debug, Clone, Copy)]
struct OrderedPair {
    pub x: f64,
    pub y: f64,
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

        let is_a = line.starts_with("Button A:");
        let is_b = line.starts_with("Button B:");
        if is_a || is_b {
            let coord_str: &str = &line[10..];
            let coord_capture = delta_re.captures(coord_str).expect("Invalid string");
            let x = coord_capture[1].parse::<f64>().expect("X is not a number");
            let y = coord_capture[2].parse::<f64>().expect("Y is not a number");
            if is_a {
                curr_cabinet.a_delta = OrderedPair { x, y };
            } else {
                curr_cabinet.b_delta = OrderedPair { x, y };
            }
        }

        if line.starts_with("Prize:") {
            let coord_str: &str = &line[7..];
            let coord_capture = coord_re.captures(coord_str).expect("Invalid string");
            let x = coord_capture[1].parse::<f64>().expect("X is not a number");
            let y = coord_capture[2].parse::<f64>().expect("Y is not a number");
            curr_cabinet.prize = OrderedPair { x, y };
        }
    }

    cabinets.push(curr_cabinet);

    cabinets
}

const A_COST: i32 = 3;
const B_COST: i32 = 1;

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
    let mut x = n1 / (a + b * (((m * a) - c) / (d - (m * b))));

    // x == [[x]]
    if (x - x.round()).abs() < 0.00001 {
        x = x.round();

        // By = N1 - Ax
        // y = (N1 - Ax) / B
        let y = ((n1 - (a * x)) / b).round();

        return Some(x * A_COST as f64 + y * B_COST as f64);
    }

    return None;
}

fn puzzle1(cabinets: &Vec<Cabinet>) -> i64 {
    let mut cost = 0;
    for cabinet in cabinets {
        if let Some(res) = solve_system(
            cabinet.a_delta.x,
            cabinet.b_delta.x,
            cabinet.prize.x,
            cabinet.a_delta.y,
            cabinet.b_delta.y,
            cabinet.prize.y,
        ) {
            cost += res as i64;
        }
    }
    cost
}

fn puzzle2(cabinets: &Vec<Cabinet>) -> i64 {
    let mut cost = 0;
    for cabinet in cabinets {
        if let Some(res) = solve_system(
            cabinet.a_delta.x,
            cabinet.b_delta.x,
            cabinet.prize.x + 10000000000000.0,
            cabinet.a_delta.y,
            cabinet.b_delta.y,
            cabinet.prize.y + 10000000000000.0,
        ) {
            cost += res as i64;
        }
    }
    cost
}

fn main() {
    let input = load_input("./input.txt");

    println!("Puzzle 1: {}", puzzle1(&input));
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
