use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    fs,
    hash::Hash,
};

fn load_input(path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let plots = line
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect::<Vec<char>>()[0])
            .collect();

        grid.push(plots);
    }

    grid
}

enum BorderSide {
    Up,
    Down,
    Left,
    Right,
}

fn analyze_plot(
    coord: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> (i32, HashMap<(usize, usize), Vec<BorderSide>>) {
    // if visited return
    // get up, down, left, right
    // if up, down, left, or right out of bounds, add 1 to perimeter
    // set this cell to visited
    // call function on cells that are same
    // with function result, return updated perimeter and area
    // (area is += 1, perimeter is number of sides that are not the same plot)

    if visited.contains(&coord) {
        return (0, HashMap::default());
    }
    visited.insert(coord.clone());

    let plot = grid[coord.1][coord.0];
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let mut borders: HashMap<(usize, usize), Vec<BorderSide>> = HashMap::new();

    // Up
    if coord.0 != 0 {
        neighbors.push((coord.0 - 1, coord.1));
    } else {
        borders.entry(coord).or_default().push(BorderSide::Up);
    }

    // Down
    if coord.0 < grid[0].len() - 1 {
        neighbors.push((coord.0 + 1, coord.1));
    } else {
        borders.entry(coord).or_default().push(BorderSide::Down);
    }

    // Left
    if coord.1 != 0 {
        neighbors.push((coord.0, coord.1 - 1));
    } else {
        borders.entry(coord).or_default().push(BorderSide::Left);
    }

    // Right
    if coord.1 < grid.len() - 1 {
        neighbors.push((coord.0, coord.1 + 1));
    } else {
        borders.entry(coord).or_default().push(BorderSide::Right);
    }

    let mut area = 1;

    for neighbor in neighbors {
        let value_at = grid[neighbor.1][neighbor.0];
        if value_at == plot {
            let res = analyze_plot(neighbor, grid, visited);
            area += res.0;
            borders.extend(res.1);
        } else {
            let x_diff = neighbor.0 as i32 - coord.0 as i32;
            let y_diff = neighbor.1 as i32 - coord.1 as i32;
            let direction = if x_diff < 0 {
                BorderSide::Left
            } else if x_diff > 0 {
                BorderSide::Right
            } else if y_diff < 0 {
                BorderSide::Up
            } else {
                BorderSide::Down
            };

            borders.entry(coord).or_default().push(direction);
        }
    }

    (area, borders)
}

fn puzzle1(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let res = analyze_plot((c, r), grid, &mut visited);
            let borders = res.1;
            let mut perimeter: i32 = 0;
            for border in borders {
                perimeter += border.1.len() as i32;
            }
            sum += res.0 * perimeter; // Area * perimeter
        }
    }

    sum
}

fn puzzle2(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let res = analyze_plot((c, r), grid, &mut visited);
            let borders = res.1;
            let mut corners: i32 = 0;
            for border in borders {
                if border.1.len() > 1 {
                    corners += 1;
                }
            }
            sum += res.0 * corners; // Area * perimeter
        }
    }

    sum
}

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
        assert_eq!(1930, puzzle1(&test_input));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(1206, puzzle2(&test_input));
    }
}
