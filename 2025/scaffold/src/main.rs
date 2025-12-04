use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
struct Args {
    /// Day number to scaffold (e.g. 1, 2, 25)
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let day_str = format!("day{:02}", day);

    println!("Scaffolding {}", day_str);

    // 1. Create directory structure
    let package_path = Path::new(&day_str);
    if package_path.exists() {
        eprintln!("Directory {} already exists", day_str);
        return;
    }

    fs::create_dir(package_path).expect("Failed to create package directory");
    fs::create_dir(package_path.join("src")).expect("Failed to create src directory");

    // 2. Create Cargo.toml
    let cargo_toml_content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
common = {{ path = "../common" }}
"#,
        day_str
    );
    fs::write(package_path.join("Cargo.toml"), cargo_toml_content)
        .expect("Failed to write Cargo.toml");

    // 3. Create src/main.rs
    let main_rs_content = format!(
        r#"use common::{{expect, read_input}};

fn part1(input: Vec<String>) -> i32 {{
    0
}}

fn part2(input: Vec<String>) -> i32 {{
    0
}}

fn main() {{
    let example_input = read_input("inputs/{}/example.txt", |s| String::from(s));
    let real_input = read_input("inputs/{}/real.txt", |s| String::from(s));

    println!("Part 1");
    if let Some(input) = &example_input {{
        expect(part1(input.clone()), None);
    }}
    if let Some(input) = &real_input {{
        expect(part1(input.clone()), None);
    }}

    println!("Part 2");
    if let Some(input) = &example_input {{
        expect(part2(input.clone()), None);
    }}
    if let Some(input) = &real_input {{
        expect(part2(input.clone()), None);
    }}
}}
"#,
        day_str, day_str
    );
    fs::write(package_path.join("src/main.rs"), main_rs_content)
        .expect("Failed to write src/main.rs");

    // 4. Create inputs
    let inputs_path = Path::new("inputs").join(&day_str);
    if !inputs_path.exists() {
        fs::create_dir_all(&inputs_path).expect("Failed to create inputs directory");
    }
    fs::write(inputs_path.join("example.txt"), "").expect("Failed to create example.txt");
    fs::write(inputs_path.join("real.txt"), "").expect("Failed to create real.txt");

    // 5. Update workspace Cargo.toml
    let workspace_cargo_path = Path::new("Cargo.toml");
    let workspace_cargo_content =
        fs::read_to_string(workspace_cargo_path).expect("Failed to read workspace Cargo.toml");

    if let Some(start_idx) = workspace_cargo_content.find("members = [") {
        if let Some(relative_end_idx) = workspace_cargo_content[start_idx..].find(']') {
            let absolute_end_idx = start_idx + relative_end_idx;

            // Check preceding non-whitespace char to decide on comma
            let slice_before = &workspace_cargo_content[start_idx..absolute_end_idx];
            let trimmed = slice_before.trim_end();

            let needs_comma = !trimmed.ends_with(',') && !trimmed.ends_with('[');

            let insert_str = if needs_comma {
                format!(", \"{}\"", day_str)
            } else {
                format!("\"{}\"", day_str)
            };

            let mut new_content = workspace_cargo_content.clone();
            new_content.insert_str(absolute_end_idx, &insert_str);
            fs::write(workspace_cargo_path, new_content)
                .expect("Failed to update workspace Cargo.toml");
            println!("Added {} to workspace members", day_str);
        } else {
            eprintln!("Could not find closing bracket for members in Cargo.toml");
        }
    } else {
        eprintln!(
            "Could not find members list in Cargo.toml. Please add {} manually.",
            day_str
        );
    }

    println!("Done! Created {} and updated workspace.", day_str);
}
