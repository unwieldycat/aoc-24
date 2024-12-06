use std::fs;

#[derive(Debug)]
enum GridPosition {
    Empty,
    EmptyVisited,
    Wall,
    Guard(Direction),
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn load_input(path: &str) -> Vec<Vec<GridPosition>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut grid: Vec<Vec<GridPosition>> = Vec::new();

    for line in contents.lines() {
        grid.push(
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| match s {
                    "." => GridPosition::Empty,
                    "#" => GridPosition::Wall,
                    "^" => GridPosition::Guard(Direction::Up),
                    "v" => GridPosition::Guard(Direction::Down),
                    "<" => GridPosition::Guard(Direction::Left),
                    ">" => GridPosition::Guard(Direction::Right),
                    other => {
                        println!("Unexpected character {}", other);
                        GridPosition::Empty
                    }
                })
                .collect(),
        );
    }

    grid
}

fn get_guard(grid: &Vec<Vec<GridPosition>>) -> ((usize, usize), Direction) {
    let mut guard_loc = (0, 0); // row, col

    // TODO: Refactor to get position of guard

    // Find initial guard position
    for i in 0..grid.len() {
        let row = &grid[i];
        let pos_opt = row
            .iter()
            .position(|pos| matches!(pos, GridPosition::Guard(_)));

        if pos_opt.is_none() {
            continue;
        } else {
            guard_loc.0 = i;
            guard_loc.1 = pos_opt.unwrap();
        }
    }

    guard_loc
}

fn puzzle1(grid: &Vec<Vec<GridPosition>>) -> i32 {
    // Get initial guard position
    let mut guard = get_guard(grid);

    loop {
        // TODO:
        // get type of guard
        //  for each direction:
        //
        //    if can move in that direction
        //      if out of bounds then break loop
        //      move and set previous location to visited
        //
        //    if cannot move, rotate guard in place and continue
    }

    // count all visited spots in grid
    // return that

    1
}

fn puzzle2() {}

fn main() {
    let input = load_input("./test_input.txt");
    //println!("{:?}", input);
    println!("Puzzle 1: {}", puzzle1(&input));
    //println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(41, puzzle1(&test_input));
    }

    /* #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(1, puzzle2(&test_input));
    } */
}
