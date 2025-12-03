use common::{expect, read_input};

fn part1(ranges: Vec<Vec<u64>>) -> u64 {
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

    sum
}

fn part2(ranges: Vec<Vec<u64>>) -> u64 {
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

    sum
}

fn parse_line(line: &str) -> Vec<Vec<u64>> {
    line.trim()
        .split(",")
        .map(|range| {
            range
                .split("-")
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let example_input =
        read_input("inputs/day02/example.txt", parse_line).map(|lines| lines[0].clone());
    let real_input = read_input("inputs/day02/real.txt", parse_line).map(|lines| lines[0].clone());

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
