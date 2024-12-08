use gcd::Gcd;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn load_input(path: &str) -> (Point, HashMap<char, Vec<Point>>) {
    let contents = fs::read_to_string(path).expect("Failed to read file");

    let mut antennae_map: HashMap<char, Vec<Point>> = HashMap::new();

    let mut max_point: Point;
    let mut max_x = 0;

    let mut y = 0;
    for line in contents.lines() {
        let chars: Vec<char> = line
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect::<Vec<char>>()[0])
            .collect();

        max_x = chars.len();

        let mut x = 0;
        for char in chars {
            if char != '.' {
                let antennae = antennae_map.entry(char).or_default();
                antennae.push(Point { x, y });
            }

            x += 1;
        }

        y += 1;
    }

    (
        Point {
            x: (max_x - 1) as i32,
            y: (contents.lines().count() - 1) as i32,
        },
        antennae_map,
    )
}

fn get_distance(p1: &Point, p2: &Point) -> (i32, i32) {
    ((p2.x - p1.x), (p2.y - p1.y))
}

fn get_distance_least(p1: &Point, p2: &Point) -> (i32, i32) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let d = (dy.abs() as u32).gcd(dx.abs() as u32);

    println!("{}/{} for {}/{}", dy / d as i32, dx / d as i32, dy, dx);

    (dx / d as i32, dy / d as i32)
}

fn within_bounds(point: &Point, upper: &Point) -> bool {
    point.x <= upper.x && point.y <= upper.y && point.x >= 0 && point.y >= 0
}

fn puzzle1(data: &HashMap<char, Vec<Point>>, max_point: &Point) -> i32 {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for map in data {
        for p0 in map.1 {
            for p1 in map.1 {
                if p0.x == p1.x && p0.y == p1.y {
                    continue;
                }

                let dist = get_distance(p0, p1);

                let antinode = Point {
                    x: p0.x + (dist.0 * 2),
                    y: p0.y + (dist.1 * 2),
                };

                if !within_bounds(&antinode, &max_point) {
                    continue;
                }

                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len() as i32
}

fn puzzle2(data: &HashMap<char, Vec<Point>>, max_point: &Point) -> i32 {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for map in data {
        for p0 in map.1 {
            for p1 in map.1 {
                if p0.x == p1.x && p0.y == p1.y {
                    continue;
                }

                let dist = get_distance_least(p0, p1);

                antinodes.insert(p0.clone());
                antinodes.insert(p1.clone());

                let mut antinode = Point {
                    x: p0.x + dist.0,
                    y: p0.y + dist.1,
                };

                while within_bounds(&antinode, max_point) {
                    antinodes.insert(antinode);
                    antinode = Point {
                        x: antinode.x + dist.0,
                        y: antinode.y + dist.1,
                    };
                }

                antinode = Point {
                    x: p1.x - dist.0,
                    y: p1.y - dist.1,
                };

                while within_bounds(&antinode, max_point) {
                    antinodes.insert(antinode);
                    antinode = Point {
                        x: antinode.x - dist.0,
                        y: antinode.y - dist.1,
                    };
                }
            }
        }
    }

    antinodes.len() as i32
}

fn main() {
    let input = load_input("./input.txt");
    println!("Puzzle 1: {}", puzzle1(&input.1, &input.0));
    println!("Puzzle 2: {}", puzzle2(&input.1, &input.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(14, puzzle1(&test_input.1, &test_input.0));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(34, puzzle2(&test_input.1, &test_input.0));
    }
}
