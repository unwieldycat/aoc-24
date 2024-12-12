use std::{
    fmt::{self, Display},
    fs,
};

#[derive(Clone, Debug, Eq)]
struct Plot {
    pub letter: String,
    pub visited: bool,
}

impl Plot {
    fn new(letter: String) -> Self {
        Self {
            letter,
            visited: false,
        }
    }
}

impl Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.letter)
    }
}

impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.letter == other.letter
    }
}

fn load_input(path: &str) -> Vec<Vec<Plot>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut grid: Vec<Vec<Plot>> = Vec::new();

    for line in contents.lines() {
        let plots = line
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| Plot::new(s.to_string()))
            .collect();

        grid.push(plots);
    }

    grid
}

fn analyze_plot(
    coord: (usize, usize),
    grid: &Vec<Vec<Plot>>,
    area_perimeter: (i32, i32),
) -> (i32, i32) {
    // if visited return
    // get up, down, left, right
    // if up, down, left, or right out of bounds, add 1 to perimeter
    // set this cell to visited
    // call function on cells that are same
    // with function result, return updated perimeter and area
    // (area is += 1, perimeter is number of sides that are not the same plot)

    let plot = &grid[coord.1][coord.0];

    if plot.visited {
        return (0, 0);
    }

    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if coord.0 != 0 {
        neighbors.push((coord.0 - 1, coord.1));
    }

    if coord.0 != grid[0].len() {
        neighbors.push((coord.0 + 1, coord.1));
    }

    if coord.1 != 0 {
        neighbors.push((coord.0, coord.1 - 1));
    }

    if coord.1 != grid.len() {
        neighbors.push((coord.0, coord.1 + 1));
    }

    (1, 1)
}

fn puzzle1(grid: &Vec<Vec<Plot>>) -> i32 {
    let mut sum = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let plot = &grid[r][c];

            if plot.visited {
                continue;
            }

            let res = analyze_plot((c, r), grid, (0, 0));
            sum += res.0 * res.1; // Area * perimeter
        }
    }

    sum
}

fn puzzle2() {}

fn main() {
    let input = load_input("./test_input.txt");
    println!("{:?}", &input);
    //println!("Puzzle 1: {}", puzzle1(&input));
    //println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_puzzle1() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(1, puzzle1(&test_input));
    // }

    // #[test]
    // fn test_puzzle2() {
    //     let test_input = load_input("./test_input.txt");
    //     assert_eq!(1, puzzle2(&test_input));
    // }
}
