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
    let mut first_run = true;

    for val in disk {
        if first_run {
            block = (val.clone(), 1);
            first_run = false;
            continue;
        }

        if *val == block.0 {
            block.1 += 1;
        } else {
            blocks.push(block.clone());
            block = (val.clone(), 1);
        }
    }

    blocks.push(block.clone());

    let mut ei = blocks.len() - 1;
    'outer: loop {
        let end_val = blocks[ei].clone();
        if end_val.0 == "." {
            ei -= 1;
            continue;
        }
        let mut si = 0;
        while si < blocks.len() {
            if ei < si {
                break;
            }
            let start_val = blocks[si].clone();
            if start_val.0 != "." || start_val.1 < end_val.1 {
                si += 1;
                continue;
            }

            // swap success
            blocks[si] = end_val.clone();
            blocks[ei] = (".".to_string(), end_val.1);
            if end_val.1 != start_val.1 {
                blocks.insert(si + 1, (".".to_string(), start_val.1 - end_val.1));
                continue 'outer;
            }
            break;
        }
        if ei == 0 {
            break;
        }
        ei -= 1;
    }

    // Merge redundant entries
    let mut index = 0;
    let mut prev_value = ("".to_string(), 0);
    first_run = true;
    while index < blocks.len() {
        if first_run {
            prev_value = blocks[index].clone();
            first_run = false;
            index += 1;
            continue;
        }
        let value = blocks[index].clone();
        if value.0 == prev_value.0 {
            // update prev index, remove this index, don't iterate
            blocks[index - 1] = (value.0.to_string(), value.1 + prev_value.1);
            blocks.remove(index);
            prev_value = blocks[index - 1].clone();
            continue;
        }
        prev_value = value;
        index += 1;
    }
    let mut checksum: u64 = 0;
    let mut true_index: u64 = 0;
    for i in 0..blocks.len() {
        let len = blocks[i].1;
        if blocks[i].0 == "." {
            true_index += len;
            continue;
        }
        let cloned_t_index = true_index.clone();
        while true_index < cloned_t_index + len {
            checksum += true_index as u64 * blocks[i].0.parse::<u64>().expect("Not a digit");
            true_index += 1;
        }
    }

    checksum
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
        assert_eq!(1928, puzzle1(&test_input));
    }

    #[test]
    fn test_puzzle2() {
        let test_input = load_input("./test_input.txt");
        assert_eq!(2858, puzzle2(&test_input));
    }
}
