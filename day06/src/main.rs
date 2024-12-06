use std::fs;

fn load_input(path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut grid: Vec<Vec<String>> = Vec::new();

    for line in contents.lines() {
        grid.push(
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| String::from(s))
                .collect(),
        )
    }

    grid
}

fn get_guard_pos(grid: &Vec<Vec<String>>) -> (usize, usize) {
    let mut guard_loc = (0, 0); // row, col

    // Find initial guard position
    for i in 0..grid.len() {
        let row = &grid[i];
        let pos_opt = row
            .iter()
            .position(|pos| pos == "^" || pos == "v" || pos == "<" || pos == ">");

        if pos_opt.is_none() {
            continue;
        } else {
            guard_loc.0 = i;
            guard_loc.1 = pos_opt.unwrap();
        }
    }

    guard_loc
}

fn rotate(guard: &str) -> &str {
    return match guard {
        "^" => ">",
        ">" => "v",
        "v" => "<",
        "<" => "^",
        _ => {
            panic!("invalid rotation");
        }
    };
}

fn get_next(guard: &str, pos: (usize, usize)) -> (usize, usize) {
    match guard {
        "^" => (pos.0 - 1, pos.1),
        ">" => (pos.0, pos.1 + 1),
        "v" => (pos.0 + 1, pos.1),
        "<" => (pos.0, pos.1 - 1),
        _ => {
            panic!("invalid guard");
        }
    }
}

fn get_value(grid: &Vec<Vec<String>>, row: usize, col: usize) -> Option<&String> {
    let row_val_opt = grid.get(row);
    if row_val_opt.is_none() {
        return None;
    } else {
        let col_val_opt = row_val_opt.unwrap().get(col);
        if col_val_opt.is_none() {
            return None;
        }
        return Some(col_val_opt.unwrap());
    }
}

fn puzzle1(grid: &mut Vec<Vec<String>>) -> i32 {
    let mut guard_pos = get_guard_pos(grid);
    let mut guard: String = grid
        .get(guard_pos.0)
        .expect("bruh 1")
        .get(guard_pos.1)
        .expect("bruh 2")
        .clone();

    loop {
        let next_pos = get_next(&guard, guard_pos);
        let next_val = get_value(grid, next_pos.0, next_pos.1);

        if next_val.is_none() {
            println!("break!");
            grid[guard_pos.0][guard_pos.1] = "X".to_owned();
            break;
        } else if next_val.unwrap() == "#" {
            println!("rotating at {} {}", guard_pos.0, guard_pos.1);
            guard = rotate(guard.as_str()).to_owned();
            grid[guard_pos.0][guard_pos.1] = guard.clone();
        } else {
            println!("going to {} {}", next_pos.0, next_pos.1);
            grid[guard_pos.0][guard_pos.1] = "X".to_owned();
            grid[next_pos.0][next_pos.1] = guard.clone();
            guard_pos = (next_pos.0, next_pos.1);
        }
    }

    let mut sum = 0;
    for row in grid {
        sum += row.iter().filter(|s| s == &&"X".to_owned()).count();
    }

    sum as i32
}

fn puzzle2() {}

fn main() {
    let mut input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&mut input));
    //println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let mut test_input = load_input("./test_input.txt");
        assert_eq!(41, puzzle1(&mut test_input));
    }

    /* #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(1, puzzle2(&test_input));
    } */
}
