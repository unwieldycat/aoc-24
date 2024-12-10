use std::{collections::HashSet, fs};

fn load_input(path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut vec: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        vec.push(
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }
    vec
}

fn traverse(prev_value: i32, pos: (i32, i32), grid: &Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    if pos.0 < 0
        || pos.1 < 0
        || pos.1 >= grid.len() as i32
        || pos.0 >= grid[pos.1 as usize].len() as i32
    {
        return HashSet::new();
    }

    let this_value: i32 = grid[pos.1 as usize][pos.0 as usize];

    let mut ends: HashSet<(i32, i32)> = HashSet::new();
    if this_value != prev_value + 1 {
        return HashSet::new();
    } else if this_value == 9 {
        let mut new_set = HashSet::new();
        new_set.insert(pos);
        return new_set;
    } else {
        let directions = vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ];

        for dir in directions {
            if dir.0 >= 0
                && dir.1 >= 0
                && dir.1 < grid.len() as i32
                && dir.0 < grid[dir.1 as usize].len() as i32
            {
                for val in traverse(this_value, dir, grid) {
                    ends.insert(val);
                }
            }
        }
    }

    return ends;
}

fn puzzle1(grid: &Vec<Vec<i32>>) -> i32 {
    // Find zeroes
    let mut zeroes: Vec<(i32, i32)> = Vec::new();
    for y in 0..grid.len() {
        let line = &grid[y];
        for x in 0..line.len() {
            if line[x] != 0 {
                continue;
            }
            zeroes.push((x as i32, y as i32));
        }
    }

    let mut score_sum: i32 = 0;
    for zero in zeroes {
        score_sum += traverse(-1, zero, grid).iter().count() as i32;
    }

    score_sum
}

fn puzzle2() {}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input));
    //println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(36, puzzle1(&test_input));
    }

    // #[test]
    // fn test_puzzle2() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(1, puzzle2(&test_input));
    // }
}
