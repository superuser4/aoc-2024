use std::process::exit;

pub fn day1_part1() {
    let content = match std::fs::read_to_string("./src/day1/day1.txt") {
        Ok(c) => {c},
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    
    for line in lines {
        let two: Vec<&str> = line.split_whitespace().collect();
        if two.len() < 2 {
            continue;
        }

        let left: i32 = two[0].parse().unwrap();
        let right: i32 = two[1].parse().unwrap();
        left_vec.push(left);
        right_vec.push(right);
    }

    left_vec.sort();
    right_vec.sort();

    let mut count: i32 = 0;
    for (left, right) in left_vec.iter().zip(right_vec.iter()) {
        let sum = left - right;
        count += sum.abs();
    }

    println!("{}", count);
}
