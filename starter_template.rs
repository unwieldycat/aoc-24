fn load_input(path: &str) {
	let contents = fs::read_to_string(file).expect("Failed to read file");
}

fn puzzle1() {}

fn puzzle2() {}

fn main() {
    let input = load_input("./input.txt");
    puzzle1(&input);
    puzzle2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

	let test_input = load_input("./test_input.txt");

    #[test]
    fn test_case() {
        assert_eq!(1, 1);
    }
}
