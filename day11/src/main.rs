use std::fs;

fn load_input(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut stones: Vec<i64> = Vec::new();
    for stone in contents.split(" ") {
        stones.push(stone.parse::<i64>().expect("Not a number!"));
    }
    stones
}

const BLINK_TIMES: i64 = 75;

fn puzzle(stones: &Vec<i64>) -> i64 {
    let mut new_stones = stones.clone();

    for i in 0..BLINK_TIMES {
        let mut s = 0;
        while s < new_stones.len() {
            let val = new_stones[s];
            let as_str = val.to_string();
            if val == 0 {
                new_stones[s] = 1;
                s += 1;
            } else if as_str.len() % 2 == 0 {
                let first = &as_str[..(as_str.len() / 2)];
                let second = &as_str[(as_str.len() / 2)..as_str.len()];
                new_stones[s] = first.parse::<i64>().expect("First not a number!");
                new_stones.insert(s + 1, second.parse::<i64>().expect("Second not a number!"));
                s += 2;
            } else {
                new_stones[s] *= 2024 as i64;
                s += 1;
            }
        }
    }

    new_stones.len() as i64
}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(55312, puzzle(&test_input));
    }
}
