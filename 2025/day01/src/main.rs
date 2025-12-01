use std::{fs::read_to_string, path::Path};

fn part1(commands: Vec<String>) {
    let mut state = 50;
    let mut count = 0;

    commands.iter().for_each(|command| {
        let magnitude = command[1..].parse::<i32>().unwrap();

        if command.starts_with("L") {
            state -= magnitude;
        } else if command.starts_with("R") {
            state += magnitude;
        }

        state = (state + 100) % 100;

        if state == 0 {
            count += 1;
        }
    });

    println!("Final state: {}, Count of 0s: {}", state, count);
}

fn part2(commands: Vec<String>) {
    let mut state = 50;
    let mut count = 0;

    commands.iter().for_each(|command| {
        let magnitude = command[1..].parse::<i32>().unwrap();
        let full_rotations = magnitude / 100;
        let rest = magnitude % 100;

        count += full_rotations;

        if command.starts_with("L") {
            if rest > state && state != 0 {
                count += 1;
            }

            state = (state - rest + 100) % 100;
        } else if command.starts_with("R") {
            if rest > (100 - state) && state != 0 {
                count += 1;
            }

            state = (state + rest) % 100;
        }

        if state == 0 {
            count += 1;
        }
    });

    println!("Final state: {}, Total wraps: {}", state, count);
}

fn process_file(file_path: &str) {
    let path = Path::new(file_path);
    if path.exists() {
        let commands: Vec<String> = read_to_string(path)
            .expect("File can't be read")
            .lines()
            .map(String::from)
            .collect();
        part1(commands.clone());
        part2(commands.clone());
    }
}

fn main() {
    process_file("input/example.txt");
    process_file("input/real.txt");
}
