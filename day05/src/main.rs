use std::collections::HashMap;
use std::fs;

fn load_input(path: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(path).expect("Failed to read file");

    let mut second: bool = false;

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];

    for line in contents.lines() {
        if line == "" {
            second = true;
            continue;
        }

        if !second {
            let nums: Vec<i32> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
            rules
                .entry(nums[1])
                .or_insert(Vec::<i32>::new())
                .push(nums[0]);
        } else {
            updates.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
        }
    }

    return (rules, updates);
}

fn get_valid_and_invalid(
    rules: &HashMap<i32, Vec<i32>>,
    updates: &Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut good_updates: Vec<Vec<i32>> = vec![];
    let mut bad_updates: Vec<Vec<i32>> = vec![];

    'updates: for update in updates {
        let mut shouldnt_be_after: Vec<i32> = vec![];

        for u in update {
            if shouldnt_be_after.contains(u) {
                bad_updates.push(update.clone());
                continue 'updates;
            }

            if rules.get(u).is_some() {
                let not_after = rules.get(u).unwrap();
                for n in not_after {
                    if update.contains(n) {
                        shouldnt_be_after.push(*n);
                    }
                }
            }
        }

        good_updates.push(update.clone());
    }

    (good_updates, bad_updates)
}

fn get_valid(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    get_valid_and_invalid(rules, updates).0
}

fn get_invalid(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    get_valid_and_invalid(rules, updates).1
}

fn puzzle1(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let valid = get_valid(rules, updates);
    let mut sum = 0;
    for vec in valid {
        let index = vec.len() / 2;
        sum += vec[index];
    }
    sum
}

fn puzzle2(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut invalid = get_invalid(rules, updates);

    for update in &mut invalid {
        let mut again = false;
        loop {
            for i in (0..(update.len() - 1)).rev() {
                let first = update[i];
                let second = update[i + 1];
                if rules.get(&first).is_some() {
                    if rules.get(&first).unwrap().contains(&second) {
                        update[i] = second;
                        update[i + 1] = first;
                        again = true;
                    }
                }
            }

            if !again {
                break;
            } else {
                again = false;
            }
        }
    }

    let mut sum = 0;
    for vec in invalid {
        let index = vec.len() / 2;
        sum += vec[index];
    }
    sum
}

fn main() {
    let input: (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input.0, &input.1));
    println!("Puzzle 2: {}", puzzle2(&input.0, &input.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13]
            ],
            get_valid(&test_input.0, &test_input.1)
        );
    }

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(143, puzzle1(&test_input.0, &test_input.1));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(123, puzzle2(&test_input.0, &test_input.1));
    }
}
