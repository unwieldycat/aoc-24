use std::{collections::HashSet, fs, i32::MAX};

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_next_dirs(current_dir: Direction) -> Vec<Direction> {
    let mut directions: Vec<Direction> = vec![current_dir];
    match current_dir {
        Direction::North => {
            directions.push(Direction::East);
            directions.push(Direction::West);
        }
        Direction::South => {
            directions.push(Direction::East);
            directions.push(Direction::West);
        }
        Direction::East => {
            directions.push(Direction::North);
            directions.push(Direction::South);
        }
        Direction::West => {
            directions.push(Direction::North);
            directions.push(Direction::South);
        }
    };
    directions
}

fn get_next_pos(next_dir: Direction, prev_pos: (usize, usize)) -> (usize, usize) {
    let mut next_pos = prev_pos.clone();
    match next_dir {
        Direction::North => {
            next_pos.1 -= 1;
        }
        Direction::South => {
            next_pos.1 += 1;
        }
        Direction::East => {
            next_pos.0 += 1;
        }
        Direction::West => {
            next_pos.0 -= 1;
        }
    }
    next_pos
}

fn find_shortest_path(
    maze: &Vec<Vec<char>>,
    position: (usize, usize),
    mut visited: HashSet<(usize, usize)>,
    current_dir: Direction,
    score: i32,
    lowest_score: &mut i32,
) -> i32 {
    if visited.contains(&position) {
        return MAX;
    }

    visited.insert(position);

    if score > *lowest_score {
        return MAX;
    }

    let this_value = maze[position.1][position.0];
    if this_value == '#' {
        return MAX;
    }

    if this_value == 'E' {
        *lowest_score = score;
        return score;
    }

    if this_value == '.' || this_value == 'S' {
        let dirs = get_next_dirs(current_dir);
        for dir in dirs {
            let next_pos = get_next_pos(dir, position);
            if dir == current_dir {
                find_shortest_path(
                    maze,
                    next_pos,
                    visited.clone(),
                    dir,
                    score + 1,
                    lowest_score,
                );
            } else {
                find_shortest_path(
                    maze,
                    next_pos,
                    visited.clone(),
                    dir,
                    score + 1001,
                    lowest_score,
                );
            }
        }
    }

    *lowest_score
}

fn puzzle1(maze: &Vec<Vec<char>>) -> i32 {
    let mut lowest_score: i32 = MAX;
    let mut start_pos: (usize, usize) = (0, 0);
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if maze[y][x] == 'S' {
                start_pos.0 = x;
                start_pos.1 = y;
            }
        }
    }
    find_shortest_path(
        maze,
        start_pos,
        HashSet::new(),
        Direction::East,
        0,
        &mut lowest_score,
    )
}

fn puzzle2() {}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input));
    // println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_puzzle1() {
        let test_input = load_input("./test1_input.txt");
        assert_eq!(7036, puzzle1(&test_input));
    }

    #[test]
    fn test2_puzzle1() {
        let test_input = load_input("./test2_input.txt");
        assert_eq!(11048, puzzle1(&test_input));
    }

    // #[test]
    // fn test_puzzle2() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(1, puzzle2(&test_input));
    // }
}
