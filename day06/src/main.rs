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

fn get_next(guard: &str, pos: (usize, usize)) -> Option<(usize, usize)> {
    match guard {
        "^" => {
            if pos.0 == 0 {
                return None;
            }
            Some((pos.0 - 1, pos.1))
        }
        ">" => Some((pos.0, pos.1 + 1)),
        "v" => Some((pos.0 + 1, pos.1)),
        "<" => {
            if pos.1 == 0 {
                return None;
            }
            Some((pos.0, pos.1 - 1))
        }
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

fn sim_guard(grid: &mut Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut guard_pos = get_guard_pos(grid);
    let mut guard: String = grid
        .get(guard_pos.0)
        .expect("bruh 1")
        .get(guard_pos.1)
        .expect("bruh 2")
        .clone();

    loop {
        let next_pos_opt = get_next(&guard, guard_pos);

        if next_pos_opt.is_none() {
            grid[guard_pos.0][guard_pos.1] = "X".to_owned();
            break;
        }

        let next_pos = next_pos_opt.unwrap();
        let next_val = get_value(grid, next_pos.0, next_pos.1);

        if next_val.is_none() {
            grid[guard_pos.0][guard_pos.1] = "X".to_owned();
            break;
        } else if next_val.unwrap() == "#" {
            guard = rotate(guard.as_str()).to_owned();
            grid[guard_pos.0][guard_pos.1] = guard.clone();
        } else {
            grid[guard_pos.0][guard_pos.1] = "X".to_owned();
            grid[next_pos.0][next_pos.1] = guard.clone();
            guard_pos = (next_pos.0, next_pos.1);
        }
    }

    grid.clone()
}

fn puzzle1(grid: &Vec<Vec<String>>) -> i32 {
    let guard_traversed = sim_guard(&mut grid.clone());

    let mut sum = 0;
    for row in guard_traversed {
        sum += row.iter().filter(|s| s == &&"X".to_owned()).count();
    }

    sum as i32
}

fn generate_grids(grid: &Vec<Vec<String>>) -> Vec<Vec<Vec<String>>> {
    let mut simulated = sim_guard(&mut grid.clone());
    let mut grids: Vec<Vec<Vec<String>>> = vec![];
    let mut guard_origin = get_guard_pos(grid);
    for r in 0..simulated.len() {
        for c in 0..simulated.len() {
            if simulated[r][c] != "X" || (r == guard_origin.0 && c == guard_origin.1) {
                continue;
            }
            let mut new_grid = grid.clone();
            new_grid[r][c] = "O".to_owned();
            grids.push(new_grid);
        }
    }
    grids
}

fn puzzle2(grid: &Vec<Vec<String>>) -> i32 {
    let mut grids = generate_grids(grid);
    let mut good = 0;

    for possible_grid in &mut grids {
        let mut guard_pos = get_guard_pos(&possible_grid);
        let mut guard: String = possible_grid
            .get(guard_pos.0)
            .expect("bruh 1")
            .get(guard_pos.1)
            .expect("bruh 2")
            .clone();
        let mut follow = 0;

        loop {
            let next_pos_opt = get_next(&guard, guard_pos);

            if next_pos_opt.is_none() {
                break;
            }

            let next_pos = next_pos_opt.unwrap();
            let next_val = get_value(&possible_grid, next_pos.0, next_pos.1);

            if next_val.is_none() {
                break;
            } else if next_val.unwrap() == "#" || next_val.unwrap() == "O" {
                guard = rotate(guard.as_str()).to_owned();
                possible_grid[guard_pos.0][guard_pos.1] = guard.clone();
            } else {
                if possible_grid[next_pos.0][next_pos.1] == "X" {
                    follow += 1;
                } else {
                    follow = 0;
                }

                // 💀
                if follow > 9999 {
                    good += 1;
                    break;
                }

                possible_grid[guard_pos.0][guard_pos.1] = "X".to_owned();
                possible_grid[next_pos.0][next_pos.1] = guard.clone();
                guard_pos = (next_pos.0, next_pos.1);
            }
        }
    }

    good
}

fn main() {
    let mut input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let mut test_input = load_input("./test_input.txt");
        assert_eq!(41, puzzle1(&mut test_input));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(6, puzzle2(&test_input));
    }
}
