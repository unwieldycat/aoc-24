use std::fs;

fn load_input(path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        grid.push(
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().nth(0).unwrap())
                .collect(),
        );
    }
    grid
}

fn puzzle1() {}

fn puzzle2() {}

fn main() {
    let input = load_input("./input.txt");
    // println!("Puzzle 1: {}", puzzle1(&input));
    // println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_puzzle1() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(7036, puzzle1(&test_input));
    // }

    // #[test]
    // fn test_puzzle2() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(1, puzzle2(&test_input));
    // }
}
