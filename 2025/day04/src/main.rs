use common::{expect, read_input};

fn part1(input: Vec<Vec<char>>) -> i32 {
    let mut total_count = 0;

    for i in 0..input.len() {
        if let Some(line) = input.get(i) {
            for j in 0..line.len() {
                if input[i][j] != '@' {
                    continue;
                }

                let mut this_count = 0;

                for off_row in [-1, 0, 1] {
                    for off_col in [-1, 0, 1] {
                        if off_row == 0 && off_col == 0 {
                            continue;
                        }

                        let r = i as isize + off_row;
                        let c = j as isize + off_col;

                        if r >= 0 && r < input.len() as isize && c >= 0 && c < line.len() as isize {
                            if input[r as usize][c as usize] == '@' {
                                this_count += 1;
                            }
                        }
                    }
                }

                if this_count < 4 {
                    total_count += 1;
                }
            }
        }
    }

    total_count
}

fn part2(input: Vec<Vec<char>>) -> i32 {
    0
}

fn parse_input(line: &str) -> Vec<char> {
    line.chars().map(|c| c.clone()).collect()
}

fn main() {
    let example_input = read_input("inputs/day04/example.txt", parse_input);
    let real_input = read_input("inputs/day04/real.txt", parse_input);

    println!("Part 1");
    if let Some(input) = &example_input {
        expect(part1(input.clone()), Some(13));
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
