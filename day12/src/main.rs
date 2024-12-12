use std::{
    collections::HashSet,
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

fn analyze_plot_perimeter(
    coord: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    if visited.contains(&coord) {
        return (0, 0);
    }
    visited.insert(coord.clone());

    let plot = grid[coord.1][coord.0];
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let mut perimeter = 0;

    // If you think this is bad, just wait..

    if coord.0 != 0 {
        neighbors.push((coord.0 - 1, coord.1));
    } else {
        perimeter += 1;
    }

    if coord.0 < grid[0].len() - 1 {
        neighbors.push((coord.0 + 1, coord.1));
    } else {
        perimeter += 1;
    }

    if coord.1 != 0 {
        neighbors.push((coord.0, coord.1 - 1));
    } else {
        perimeter += 1;
    }

    if coord.1 < grid.len() - 1 {
        neighbors.push((coord.0, coord.1 + 1));
    } else {
        perimeter += 1;
    }

    let mut area = 1;

    for neighbor in neighbors {
        let value_at = grid[neighbor.1][neighbor.0];
        if value_at == plot {
            let res = analyze_plot_perimeter(neighbor, grid, visited);
            area += res.0;
            perimeter += res.1;
        } else {
            perimeter += 1;
        }
    }

    (area, perimeter)
}

fn puzzle1(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let res = analyze_plot_perimeter((c, r), grid, &mut visited);
            sum += res.0 * res.1; // Area * perimeter
        }
    }

    sum
}

fn analyze_plot_sides(
    coord: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    if visited.contains(&coord) {
        return (0, 0);
    }
    visited.insert(coord.clone());

    let plot = grid[coord.1][coord.0];
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let mut corners: i32 = 0;

    let up_valid = coord.1 != 0;
    let down_valid = coord.1 != grid.len() - 1;
    let left_valid = coord.0 != 0;
    let right_valid = coord.0 != grid[0].len() - 1;

    // ✨ Prepare your eyes, this is about to be absolutely horrible ✨
    // Behold, the yandere technique:

    if left_valid {
        neighbors.push((coord.0 - 1, coord.1));
    }

    if right_valid {
        neighbors.push((coord.0 + 1, coord.1));
    }

    if up_valid {
        neighbors.push((coord.0, coord.1 - 1));
        let up_val = grid[coord.1 - 1][coord.0];
        if left_valid {
            let left_val = grid[coord.1][coord.0 - 1];
            let diag = grid[coord.1 - 1][coord.0 - 1];
            if (left_val != plot && up_val != plot)
                || (diag != plot && left_val == plot && up_val == plot)
            {
                corners += 1;
            }
        } else if up_val != plot {
            corners += 1;
        }

        if right_valid {
            let right_val = grid[coord.1][coord.0 + 1];
            let diag = grid[coord.1 - 1][coord.0 + 1];
            if (right_val != plot && up_val != plot)
                || (diag != plot && right_val == plot && up_val == plot)
            {
                corners += 1;
            }
        } else if up_val != plot {
            corners += 1;
        }
    } else {
        if left_valid && grid[coord.1][coord.0 - 1] != plot {
            corners += 1;
        }

        if right_valid && grid[coord.1][coord.0 + 1] != plot {
            corners += 1
        }

        if !left_valid {
            corners += 1;
        }

        if !right_valid {
            corners += 1;
        }
    }

    if down_valid {
        neighbors.push((coord.0, coord.1 + 1));
        let down_val = grid[coord.1 + 1][coord.0];
        if left_valid {
            let left_val = grid[coord.1][coord.0 - 1];
            let diag = grid[coord.1 + 1][coord.0 - 1];
            if (left_val != plot && down_val != plot)
                || (diag != plot && left_val == plot && down_val == plot)
            {
                corners += 1;
            }
        } else if down_val != plot {
            corners += 1;
        }
        if right_valid {
            let right_val = grid[coord.1][coord.0 + 1];
            let diag = grid[coord.1 + 1][coord.0 + 1];
            if (right_val != plot && down_val != plot)
                || (diag != plot && right_val == plot && down_val == plot)
            {
                corners += 1;
            }
        } else if down_val != plot {
            corners += 1;
        }
    } else {
        if left_valid && grid[coord.1][coord.0 - 1] != plot {
            corners += 1;
        }

        if right_valid && grid[coord.1][coord.0 + 1] != plot {
            corners += 1
        }

        if !left_valid {
            corners += 1;
        }

        if !right_valid {
            corners += 1;
        }
    }

    let mut area = 1;

    for neighbor in neighbors {
        let value_at = grid[neighbor.1][neighbor.0];
        if value_at == plot {
            let res = analyze_plot_sides(neighbor, grid, visited);
            area += res.0;
            corners += res.1;
        }
    }

    (area, corners)
}

fn puzzle2(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let res = analyze_plot_sides((c, r), grid, &mut visited);
            if !(res.0 == 0 && res.1 == 0) {
                println!("In region {}, area: {} sides: {}", grid[r][c], res.0, res.1);
            }

            sum += res.0 * res.1; // Area * sides
        }
    }

    sum
}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
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
