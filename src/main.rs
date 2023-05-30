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

    // Read names from JSON file
    let data = fs::read_to_string(input_file).expect("Unable to read names.json file!");
    // let names_json: Values = serde_json::from_str(&data).expect("Unable to parse names.json file!");
    let mut names_data: NamesData =
        serde_json::from_str(&data).expect("Unable to parse names.json file!");

    // Pair up names deterministically
    let mut pairs: Vec<Vec<String>> = Vec::new();
    let mut names = names_data.names.clone();

    let offset =  names_data.iteration;

    while names.len() > 2 {
        let first = names.remove(0);
        let second = names.remove(0 + offset % names.len());
        pairs.push(vec![first, second]);
    }

    if (names.len() % 2) == 1 {
        let last_name = names.last().unwrap().clone();
        let last_pair = pairs.last_mut().unwrap();
        last_pair.push(last_name);
    }

    names_data.iteration += 1;

    let data = serde_json::to_string_pretty(&names_data).expect("Unable to serialize names.json file!");
    fs::write(input_file, data).expect("Unable to write names.json file!");

    // Print results
    for pair in &pairs {
        println!("{}", pair.join(" & "));
    }

    Ok(())
}
