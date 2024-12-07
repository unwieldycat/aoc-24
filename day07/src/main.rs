use std::{collections::HashMap, fs};

fn load_input(path: &str) -> HashMap<i64, Vec<i64>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut nummap: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in contents.lines() {
        let st: Vec<&str> = line.split(":").collect();
        let result_num: i64 = st[0].parse::<i64>().unwrap();
        let nums: Vec<i64> = st[1]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        nummap.insert(result_num, nums);
    }

    nummap
}

fn gen_operators(num: usize, third_op: bool) -> Vec<Vec<char>> {
    let mut combos: Vec<Vec<char>> = Vec::new();
    let mut combo: Vec<char> = ['+'].repeat(num);

    loop {
        combos.push(combo.clone());

        if !combo.contains(&'+') && (!third_op || !combo.contains(&'*')) {
            break;
        }

        for i in 0..combo.len() {
            if combo[i] == '+' {
                combo[i] = '*';
                break;
            } else if combo[i] == '*' && third_op {
                combo[i] = '|';
                break;
            } else if combo[i] == '|' && third_op {
                combo[i] = '+'
            } else if combo[i] == '*' {
                combo[i] = '+';
            }
        }
    }

    combos
}

fn evaluate(part2: bool, data: &HashMap<i64, Vec<i64>>) -> i64 {
    let mut sum = 0;

    for line in data {
        let nums = line.1;
        let combos = gen_operators(nums.len() - 1, part2);

        for combo in combos {
            let mut this_sum: i64 = nums[0];

            for i in 0..combo.len() {
                if combo[i] == '*' {
                    this_sum *= nums[i + 1];
                } else if combo[i] == '|' {
                    this_sum = format!("{}{}", this_sum, nums[i + 1])
                        .parse::<i64>()
                        .unwrap();
                } else {
                    this_sum += nums[i + 1];
                }
            }

            if this_sum == *line.0 {
                sum += this_sum;
                break;
            }
        }
    }

    sum
}

fn puzzle1(data: &HashMap<i64, Vec<i64>>) -> i64 {
    evaluate(false, data)
}

fn puzzle2(data: &HashMap<i64, Vec<i64>>) -> i64 {
    evaluate(true, data)
}

fn main() {
    let input: HashMap<i64, Vec<i64>> = load_input("./input.txt");

    println!("Puzzle 1: {}", puzzle1(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(3749, puzzle1(&test_input));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(11387, puzzle2(&test_input));
    }
}
