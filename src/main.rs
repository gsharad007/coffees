use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};

#[derive(Serialize, Deserialize)]
struct NamesData {
    names: Vec<String>,
    iteration: usize,
}

fn main() -> io::Result<()> {
    let input_file = "names.json"; // Replace with your input file name

    let names_data = read_json_from_file(input_file);

    let pairs =
        generate_permutations_deterministically(names_data.names.clone(), names_data.iteration);

    let names_data = increment_iteration(names_data);

    write_json_to_file(names_data, input_file);

    print_results(pairs);

    Ok(())
}

fn generate_permutations_deterministically(
    mut names: Vec<String>,
    iteration: usize,
) -> Vec<Vec<String>> {
    let mut pairs: Vec<Vec<String>> = Vec::new();

    let offset = iteration;

    while names.len() > 2 {
        let first = names.remove(0);
        let second = names.remove(0 + offset % names.len());
        pairs.push(vec![first, second]);
    }
    if names.len() == 1 {
        let last = names.remove(0);
        let last_pair = pairs.last_mut().unwrap();
        last_pair.push(last);
    }
    pairs
}

fn increment_iteration(names_data: NamesData) -> NamesData {
    let mut names_data = names_data;
    names_data.iteration += 1;
    names_data
}

fn read_json_from_file(input_file: &str) -> NamesData {
    // Read names from JSON file
    let data = fs::read_to_string(input_file).expect("Unable to read names.json file!");
    // let names_json: Values = serde_json::from_str(&data).expect("Unable to parse names.json file!");
    let names_data: NamesData =
        serde_json::from_str(&data).expect("Unable to parse names.json file!");
    names_data
}

fn write_json_to_file(names_data: NamesData, input_file: &str) {
    let data =
        serde_json::to_string_pretty(&names_data).expect("Unable to serialize names.json file!");
    fs::write(input_file, data).expect("Unable to write names.json file!");
}

fn print_results(pairs: Vec<Vec<String>>) {
    // Print results
    for pair in &pairs {
        println!("{}", pair.join(" & "));
    }
}
