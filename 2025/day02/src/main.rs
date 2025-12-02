use std::{fs::read_to_string, path::Path};

fn part1(ranges: Vec<Vec<u64>>) {
    let mut sum = 0;

    ranges.iter().for_each(|range| {
        if range.len() != 2 {
            panic!("Expect all ranges to have .len() = 2")
        }

        let start = range[0];
        let end = range[1];

        for num in start..=end {
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                let mid = num_str.len() / 2;
                let first_part = &num_str[..mid];
                let second_part = &num_str[mid..];

                if first_part == second_part {
                    sum += num;
                }
            }
        }
    });

    println!("Part 1: Sum of all double numbers in ranges = {}", sum);
}

fn part2(ranges: Vec<Vec<u64>>) {
    let mut sum = 0;

    ranges.iter().for_each(|range| {
        if range.len() != 2 {
            panic!("Expected all ranges to have .len() = 2");
        }

        let start = range[0];
        let end = range[1];

        for num in start..=end {
            let num_str = num.to_string();

            let max_len = num_str.len() / 2;
            for len in 1..=max_len {
                if num_str.len() % len != 0 {
                    continue;
                }

                let splits = num_str
                    .as_bytes()
                    .chunks(len)
                    .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
                    .collect::<Vec<String>>();

                let first_part = &splits[0];
                if splits.iter().all(|part| part == first_part) {
                    sum += num;
                    break;
                }
            }
        }
    });

    println!("Part 2: Sum of all repeated numbers in ranges = {}", sum);
}

fn process_file(file_path: &str) {
    println!("File: {}", file_path);

    let path = Path::new(file_path);
    if !path.exists() {
        println!("File not found!");
        return;
    }

    let lines: Vec<Vec<Vec<u64>>> = read_to_string(path)
        .expect("File can't be read")
        .lines()
        .map(String::from)
        .map(|line| {
            line.trim()
                .to_string()
                .split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    if lines.len() > 1 {
        panic!("Only a single line was expected")
    }

    let ranges = &lines[0];
    part1(ranges.clone());
    part2(ranges.clone());
}

fn main() {
    process_file("input/day02/example.txt");
    process_file("input/day02/real.txt");
}
