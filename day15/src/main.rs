use std::fs;

fn load_input(path: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    let mut parse_warehouse = true;
    for line in contents.lines() {
        if line.is_empty() {
            parse_warehouse = false;
            continue;
        }

        if parse_warehouse {
            warehouse.push(
                line.split("")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.chars().nth(0).unwrap())
                    .collect(),
            );
        } else {
            movements.append(
                &mut line
                    .split("")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.chars().nth(0).unwrap())
                    .collect::<Vec<char>>(),
            );
        }
    }
    (warehouse, movements)
}

/// Returns as (x, y) or (c, r)
fn find_robot(warehouse: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for r in 0..warehouse.len() {
        let row = &warehouse[r];
        for c in 0..row.len() {
            if row[c] == '@' {
                return Some((c as i32, r as i32));
            }
        }
    }
    return None;
}

fn attempt_movement(
    warehouse: &mut Vec<Vec<char>>,
    robot_pos: &mut (i32, i32),
    movement_mod: (i32, i32),
) {
    let mut adjacent = (robot_pos.0 + movement_mod.0, robot_pos.1 + movement_mod.1);
    let mut queue: Vec<char> = Vec::new();
    queue.push('.');
    queue.push('@');
    while adjacent.0 >= 0 && adjacent.1 >= 0 {
        let value_at = warehouse[adjacent.1 as usize][adjacent.0 as usize];
        if value_at == '.' {
            queue.reverse();
            for obj in &queue {
                warehouse[adjacent.1 as usize][adjacent.0 as usize] = obj.clone();
                adjacent = (adjacent.0 - movement_mod.0, adjacent.1 - movement_mod.1);
            }

            break;
        } else if value_at == '#' {
            break;
        } else if value_at == 'O' {
            queue.push(value_at);
        }
        adjacent = (adjacent.0 + movement_mod.0, adjacent.1 + movement_mod.1);
    }

    let new_robot_pos = find_robot(warehouse).unwrap();
    robot_pos.0 = new_robot_pos.0;
    robot_pos.1 = new_robot_pos.1;
}

fn score(warehouse: &Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;
    for r in 0..warehouse.len() {
        for c in 0..warehouse[r].len() {
            let value_at = warehouse[r][c];
            if value_at == 'O' || value_at == '[' {
                sum += (100 * (r as i32)) + (c as i32);
            }
        }
    }
    sum
}

fn puzzle1(warehouse: &Vec<Vec<char>>, movements: &Vec<char>) -> i32 {
    let mut warehouse_copy = warehouse.clone();
    let mut robot_pos = find_robot(&warehouse).unwrap();
    for movement in movements {
        if *movement == '<' {
            attempt_movement(&mut warehouse_copy, &mut robot_pos, (-1, 0));
        } else if *movement == '^' {
            attempt_movement(&mut warehouse_copy, &mut robot_pos, (0, -1));
        } else if *movement == '>' {
            attempt_movement(&mut warehouse_copy, &mut robot_pos, (1, 0));
        } else if *movement == 'v' {
            attempt_movement(&mut warehouse_copy, &mut robot_pos, (0, 1));
        }
    }

    score(&warehouse_copy)
}

fn widen_warehouse(warehouse: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut widened: Vec<Vec<char>> = Vec::new();
    let mut new_row: Vec<char>;

    for row in warehouse {
        new_row = Vec::new();
        for val in row {
            match val {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => {}
            }
        }
        widened.push(new_row.clone());
    }

    widened
}

fn puzzle2(warehouse: &Vec<Vec<char>>, movements: &Vec<char>) -> i32 {
    let mut wide_warehouse = widen_warehouse(warehouse);

    score(&wide_warehouse)
}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input.0, &input.1));
    println!("Puzzle 2: {}", puzzle2(&input.0, &input.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(10092, puzzle1(&test_input.0, &test_input.1));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(9021, puzzle2(&test_input.0, &test_input.1));
    }
}
