use std::fs;

fn load_input(file: &str) {
    let contents = fs::read_to_string(file).expect("Failed to read file");
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut split = line.split("\t");
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }
}

fn puzzle1(list1: &mut Vec<i32>, list2: &mut Vec<i32>) {
    list1.sort_by(|a, b| a.cmp(b));
    list2.sort_by(|a, b| a.cmp(b));

    let mut sum = 0;

    for i in 0..list1.len() {
        sum += (list1[i] - list2[i] as i32).abs();
    }

    println!("Sum: {}", sum);
}

fn puzzle2(list1: &Vec<i32>, list2: &Vec<i32>) {
    let mut similarity = 0;
    for ln in list1 {
        let mut in_right = 0;
        for rn in list2 {
            if rn == ln {
                in_right += 1;
            }
        }
        similarity += ln * in_right;
    }

    println!("Similarity: {}", similarity);
}

fn main() {
    let mut test_input = load_input("./test_input.txt");
    let mut input = load_input("./input.txt");

    print!("(test) ");
    puzzle1(&mut test_input.0, &mut test_input.1);
    puzzle1(&mut input.0, &mut input.1);

    print!("(test) ");
    puzzle2(&test_input.0, &test_input.1);
    puzzle2(&input.0, &input.1);
}
