use std::{fmt::Debug, fs::read_to_string, path::Path};

pub fn expect<T: PartialEq + Debug>(result: T, expected: Option<T>) {
    match expected {
        None => println!("RESULT: {:#?} (No expected Value)", result),
        Some(value) => {
            if result == value {
                println!("PASS: {:#?}", result);
            } else {
                println!("FAIL: got {:#?}, expected {:#?}", result, value);
            }
        }
    }
}

pub fn read_input<T>(file_path: &str, parser: impl Fn(&str) -> T) -> Option<Vec<T>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return None;
    }

    Some(
        read_to_string(path)
            .expect("File couldn't be read")
            .lines()
            .map(parser)
            .collect(),
    )
}
