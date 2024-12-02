use std::{f32::INFINITY, fs};

fn load_input(file: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file).expect("Failed to read file");
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let mut report: Vec<i32> = Vec::new();
        let split: Vec<&str> = line.split(" ").collect();

        for v in split {
            report.push(v.parse::<i32>().unwrap());
        }

        reports.push(report);
    }

    return reports;
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut last_n: i32 = std::i32::MAX;
    let mut last_diff: i32 = std::i32::MAX;

    for n in report {
        // first
        if last_n == std::i32::MAX {
            last_n = *n;
            continue;
        }

        let this_diff = n - last_n;

        // too big or no change
        if this_diff.abs() > 3 || last_n == *n {
            return false;
        }

        if last_diff == std::i32::MAX {
            last_diff = this_diff;
            last_n = *n;
            continue;
        }

        // diff direction
        if last_diff.signum() != this_diff.signum() {
            return false;
        }

        last_diff = this_diff;
        last_n = *n;
    }

    return true;
}

fn puzzle1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut valid: i32 = 0;

    for report in reports {
        if check_report(report) {
            valid += 1;
        }
    }

    return valid;
}

fn puzzle2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut valid: i32 = 0;

    for report in reports {
        // default works
        if check_report(report) {
            valid += 1;
        } else {
            // tomfoolery
            for i in 0..report.len() {
                let mut dampened_report = report.clone();
                dampened_report.remove(i);

                if check_report(&dampened_report) {
                    valid += 1;
                    break;
                }
            }
        }
    }

    return valid;
}

fn main() {
    let test_input = load_input("./test_input.txt");
    let input = load_input("./input.txt");

    println!("(test) Puzzle 1: {}", puzzle1(&test_input));
    println!("Puzzle 1: {}", puzzle1(&input));

    println!("(test) Puzzle 2: {}", puzzle2(&test_input));
    println!("Puzzle 2: {}", puzzle2(&input));
}
