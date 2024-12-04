use std::fs;

fn load_input(path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");

    let mut input: Vec<Vec<String>> = Vec::new();

    for line in contents.lines() {
        input.push(
            line.split("")
                .map(|s| String::from(s))
                .filter(|s| s != "")
                .collect::<Vec<String>>(),
        );
    }

    return input;
}

fn find_hor(input: &Vec<Vec<String>>) -> i32 {
    let mut found: i32 = 0;

    for line in input {
        found += line
            .windows(4)
            .map(|w| w.join(""))
            .filter(|w| w.eq("XMAS") || w.eq("SAMX"))
            .count() as i32;
    }

    found
}

fn find_ver(input: &Vec<Vec<String>>) -> i32 {
    let mut found = 0;
    for row in 0..input[0].len() {
        for col in 0..input.len() - 3 {
            let mut col_string = "".to_owned();
            col_string.push_str(&input[col][row]);
            col_string.push_str(&input[col + 1][row]);
            col_string.push_str(&input[col + 2][row]);
            col_string.push_str(&input[col + 3][row]);

            if col_string.eq("XMAS") || col_string.eq("SAMX") {
                found += 1;
            }
        }
    }

    found
}

fn find_diag_right(input: &Vec<Vec<String>>) -> i32 {
    let mut found = 0;
    for row in 0..input[0].len() - 3 {
        for col in 0..input.len() - 3 {
            let mut col_string = "".to_owned();
            col_string.push_str(&input[col][row]);
            col_string.push_str(&input[col + 1][row + 1]);
            col_string.push_str(&input[col + 2][row + 2]);
            col_string.push_str(&input[col + 3][row + 3]);

            if col_string.eq("XMAS") || col_string.eq("SAMX") {
                found += 1;
            }
        }
    }

    found
}

fn find_diag_left(input: &Vec<Vec<String>>) -> i32 {
    let mut found = 0;
    for row in 0..input[0].len() - 3 {
        for col in 0..input.len() - 3 {
            let mut col_string = "".to_owned();
            col_string.push_str(&input[col][row + 3]);
            col_string.push_str(&input[col + 1][row + 2]);
            col_string.push_str(&input[col + 2][row + 1]);
            col_string.push_str(&input[col + 3][row]);

            if col_string.eq("XMAS") || col_string.eq("SAMX") {
                found += 1;
            }
        }
    }

    found
}

fn puzzle1(input: &Vec<Vec<String>>) -> i32 {
    find_diag_left(input) + find_diag_right(input) + find_hor(input) + find_ver(input)
}

fn puzzle2(input: &Vec<Vec<String>>) -> i32 {
    let mut found = 0;
    for row in 0..input[0].len() - 2 {
        for col in 0..input.len() - 2 {
            let mut left_str = "".to_owned();
            left_str.push_str(&input[col][row + 2]);
            left_str.push_str(&input[col + 1][row + 1]);
            left_str.push_str(&input[col + 2][row]);

            let mut right_str = "".to_owned();
            right_str.push_str(&input[col][row]);
            right_str.push_str(&input[col + 1][row + 1]);
            right_str.push_str(&input[col + 2][row + 2]);

            if (left_str.eq("MAS") || left_str.eq("SAM"))
                && (right_str.eq("MAS") || right_str.eq("SAM"))
            {
                found += 1;
            }
        }
    }

    found
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
    fn puzzle1_test() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(18, puzzle1(&test_input));
    }

    #[test]
    fn puzzle2_test() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(9, puzzle2(&test_input));
    }
}
