use regex::Regex;
use std::fs;

fn load_input(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    return contents;
}

fn puzzle1(input: &String) -> i32 {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\))").unwrap();

    let mut total: i32 = 0;

    for (_, [num1, num2]) in re.captures_iter(&input).map(|c| c.extract()) {
        let firstnum = num1.parse::<i32>().unwrap();
        let secondnum = num2.parse::<i32>().unwrap();
        total += firstnum * secondnum;
    }

    return total;
}

fn puzzle2(input: &String) -> i32 {
    let re = Regex::new(r"((?:mul\((\d+),(\d+)\))|(?:do\(\)|don't\(\)))").unwrap();

    let mut total: i32 = 0;
    let mut capture: bool = true;
    for c in re.captures_iter(&input) {
        let c_str = c.get(0).unwrap().as_str();

        if c_str == "do()" {
            capture = true;
        } else if c_str == "don't()" {
            capture = false;
        } else if capture {
            let num1 = c.get(2).unwrap().as_str();
            let num2 = c.get(3).unwrap().as_str();
            let firstnum = num1.parse::<i32>().unwrap();
            let secondnum = num2.parse::<i32>().unwrap();
            total += firstnum * secondnum;
        }
    }

    return total;
}

fn main() {
    let input = load_input("./input.txt");
    println!("puzzle 1: {}", puzzle1(&input));
    println!("puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let test_input = load_input("./test_input1.txt");
        assert_eq!(puzzle1(&test_input), 161);
    }

    #[test]
    fn puzzle2_test() {
        let test_input = load_input("./test_input2.txt");
        assert_eq!(puzzle2(&test_input), 48);
    }
}
