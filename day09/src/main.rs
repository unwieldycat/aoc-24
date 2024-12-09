use std::fs;

fn load_input(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut diskmap: Vec<u32> = Vec::new();
    for char in contents.chars() {
        diskmap.push(char.to_digit(10).expect("not a digit"));
    }

    let mut disk: Vec<String> = Vec::new();
    let mut free_space = false;
    let mut id = 0;
    for val in diskmap {
        if free_space {
            for _ in 0..val {
                disk.push(".".to_string());
            }
        } else {
            for _ in 0..val {
                disk.push(id.to_string());
            }
            id += 1;
        }

        free_space = !free_space;
    }

    disk
}

fn puzzle1(disk: &Vec<String>) -> u64 {
    let mut compacted = disk.clone();

    'outer: for ei in (0..compacted.len()).rev() {
        let end_value = &compacted[ei];
        if end_value == "." {
            continue;
        }
        // get first empty space
        for si in 0..compacted.len() {
            let start_value = &compacted[si];
            if start_value != "." {
                continue;
            }
            if si > ei {
                break 'outer;
            }
            compacted[si] = end_value.clone();
            compacted[ei] = ".".to_string();
            break;
        }
    }

    let mut checksum: u64 = 0;

    for i in 0..compacted.len() {
        if compacted[i] == "." {
            continue;
        }
        checksum += i as u64 * compacted[i].parse::<u64>().expect("Not a digit");
    }

    checksum
}

fn puzzle2(disk: &Vec<String>) -> u64 {
    let mut blocks: Vec<(String, u64)> = Vec::new();

    let mut block: (String, u64) = ("".to_string(), 0);
    let mut new_block = true;

    // problem here somewhere, output is wrong
    for val in disk {
        if new_block {
            block = (val.clone(), 1);
            new_block = false;
            continue;
        }

        if *val == block.0 {
            block.1 += 1;
        } else {
            blocks.push(block.clone());
            new_block = true;
        }
    }

    // start: 00...111...2...333.44.5555.6666.777.888899
    // res:   00992111777.44.333....5555.6666.....8888..

    blocks.push(block.clone());

    // this broken too
    println!("b4: {:?}", blocks);
    for si in 0..blocks.len() {
        println!("{:?}", blocks);
        if !(blocks[si].0 == ".") {
            continue;
        }
        let space_len = blocks[si].1;
        for ei in (0..blocks.len()).rev() {
            if ei < si {
                break;
            }
            let end_val = &blocks[ei].clone();
            if end_val.0 == "." || end_val.1 > space_len {
                continue;
            }

            blocks[si] = end_val.clone();
            blocks[ei] = (".".to_string(), space_len - end_val.1);

            if end_val.1 != space_len {
                println!("Insertion at {}", si + 1);
                blocks.insert(si + 1, (".".to_string(), space_len - end_val.1));
            }

            break;
        }
    }

    println!("{:?}", blocks);

    let mut checksum: u64 = 0;

    for i in 0..blocks.len() {
        if blocks[i].0 == "." {
            continue;
        }
        checksum += i as u64 * blocks[i].0.parse::<u64>().expect("Not a digit");
    }

    checksum
}

fn main() {
    let input = load_input("./test_input.txt");
    println!("Puzzle 1: {}", puzzle1(&input));
    println!("Puzzle 2: {}", puzzle2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(1928, puzzle1(&test_input));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(2858, puzzle2(&test_input));
    }
}
