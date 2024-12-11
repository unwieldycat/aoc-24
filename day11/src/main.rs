use std::{collections::HashMap, fs};

fn load_input(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut stones: Vec<i64> = Vec::new();
    for stone in contents.split(" ") {
        stones.push(stone.parse::<i64>().expect("Not a number!"));
    }
    stones
}

fn process_stone(
    blink_times: i32,
    blinks: i32,
    stone: i64,
    cache: &mut HashMap<(i64, i32), i64>,
) -> i64 {
    let map_val = cache.get(&(stone, blinks));

    if blinks >= blink_times {
        return 1;
    }

    if !map_val.is_none() {
        return map_val.unwrap().clone();
    }

    let as_str = stone.to_string();

    if stone == 0 {
        let retval = process_stone(blink_times, blinks + 1, 1, cache);
        cache.insert((stone, blinks), retval);
        return retval;
    } else if as_str.len() % 2 == 0 {
        let first_s = &as_str[..(as_str.len() / 2)];
        let second_s = &as_str[(as_str.len() / 2)..as_str.len()];
        let first = first_s.parse::<i64>().expect("First not a number!");
        let second = second_s.parse::<i64>().expect("Second not a number!");
        let retval = process_stone(blink_times, blinks + 1, first, cache)
            + process_stone(blink_times, blinks + 1, second, cache);
        cache.insert((stone, blinks), retval);
        return retval;
    } else {
        let retval = process_stone(blink_times, blinks + 1, stone * 2024, cache);
        cache.insert((stone, blinks), retval);
        return retval;
    }
}

fn puzzle(stones: &Vec<i64>, blink_times: i32) -> i64 {
    let mut cache: HashMap<(i64, i32), i64> = HashMap::new();
    let mut sum = 0;

    for stone in stones {
        sum += process_stone(blink_times, 0, *stone, &mut cache);
    }

    sum
}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle(&input, 25));
    println!("Puzzle 2: {}", puzzle(&input, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(55312, puzzle(&test_input, 25));
    }
}
