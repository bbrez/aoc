use common::expect;
use std::{fs::read_to_string, path::Path};

fn part1(_input: String) -> i32 {
    0
}

fn part2(_input: String) -> i32 {
    0
}

fn get_input(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return None;
    }
    Some(read_to_string(path).expect("File can't be read"))
}

fn main() {
    let example_input = get_input("inputs/day03/example.txt");
    let real_input = get_input("inputs/day03/real.txt");

    println!("Part 1");
    if let Some(input) = &example_input {
        expect(part1(input.clone()), None);
    }
    if let Some(input) = &real_input {
        expect(part1(input.clone()), None);
    }

    println!("Part 2");
    if let Some(input) = &example_input {
        expect(part2(input.clone()), None);
    }
    if let Some(input) = &real_input {
        expect(part2(input.clone()), None);
    }
}
