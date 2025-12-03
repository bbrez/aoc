use common::{expect, read_input};

fn part1(input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    input.iter().for_each(|line| {
        if line.len() < 2 {
            panic!("Expected all lines to have at least 2 digits");
        }

        let mut it = line.iter().rev();
        let mut second_largest = it.next().unwrap().clone();
        let mut largest = it.next().unwrap().clone();

        it.for_each(|digit| {
            if *digit >= largest {
                if largest > second_largest {
                    second_largest = largest.clone();
                }
                largest = digit.clone();
            }
        });

        sum += largest * 10 + second_largest;
    });

    sum
}

fn part2(_input: Vec<Vec<u32>>) -> u64 {
    0
}

fn parse_line(line: &str) -> Vec<u32> {
    line.trim().chars().filter_map(|d| d.to_digit(10)).collect()
}

fn main() {
    let example_input = read_input("inputs/day03/example.txt", parse_line);
    let real_input = read_input("inputs/day03/real.txt", parse_line);

    println!("Part 1");
    if let Some(input) = &example_input {
        expect(part1(input.clone()), Some(357));
    }
    if let Some(input) = &real_input {
        expect(part1(input.clone()), None);
    }

    println!("Part 2");
    if let Some(input) = &example_input {
        expect(part2(input.clone()), Some(3121910778619));
    }
    if let Some(input) = &real_input {
        expect(part2(input.clone()), None);
    }
}
