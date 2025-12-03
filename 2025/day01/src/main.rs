use common::{expect, read_input};

fn part1(commands: Vec<String>) -> i32 {
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

    count
}

fn part2(commands: Vec<String>) -> i32 {
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

    count
}

fn main() {
    let example_input = read_input("inputs/day01/example.txt", |s| String::from(s));
    let real_input = read_input("inputs/day01/real.txt", |s| String::from(s));

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
