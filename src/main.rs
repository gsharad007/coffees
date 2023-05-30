use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NamesData {
    names: Vec<String>,
    greetings: String,
    iteration: usize,
}

fn main() -> io::Result<()> {
    let input_file = "names.json"; // Replace with your input file name

    let names_data = read_json_from_file(input_file);

    let pairs =
        generate_permutations_deterministically(names_data.names.clone(), names_data.iteration);

    let names_data = increment_iteration(names_data);

    let num = names_data.iteration;
    let prefix = names_data.greetings.clone();

    write_json_to_file(names_data, input_file);

    print_results(pairs, num, prefix);

    Ok(())
}

fn generate_permutations_deterministically(
    mut names: Vec<String>,
    iteration: usize,
) -> Vec<Vec<String>> {
    let mut pairs: Vec<Vec<String>> = Vec::with_capacity(names.len() / 2);

    let offset = iteration;

    while names.len() > 1 {
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

fn print_results(pairs: Vec<Vec<String>>, iteration: usize, greeting: String) {
    println!("{}UnRandom Coffees Week {}", greeting, iteration);
    for pair in &pairs {
        println!("* {}", pair.join(" & "));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_generate_permutations_deterministically_even() {
        let names = vec![
            "Leeloo".to_string(),
            "Diva Plavalaguna".to_string(),
            "Korben Dallas".to_string(),
            "Ruby Rhod".to_string(),
            "Jean-Baptiste Zorg".to_string(),
            "Vito Cornelius".to_string(),
        ];
        let comb1 = vec![
            vec!["Leeloo".to_string(), "Diva Plavalaguna".to_string()],
            vec!["Korben Dallas".to_string(), "Ruby Rhod".to_string()],
            vec![
                "Jean-Baptiste Zorg".to_string(),
                "Vito Cornelius".to_string(),
            ],
        ];
        let comb2 = vec![
            vec!["Leeloo".to_string(), "Korben Dallas".to_string()],
            vec![
                "Diva Plavalaguna".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
            vec!["Ruby Rhod".to_string(), "Vito Cornelius".to_string()],
        ];
        let comb3 = vec![
            vec!["Leeloo".to_string(), "Ruby Rhod".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Vito Cornelius".to_string()],
            vec![
                "Korben Dallas".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
        ];
        let comb4 = vec![
            vec!["Leeloo".to_string(), "Jean-Baptiste Zorg".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Korben Dallas".to_string()],
            vec!["Ruby Rhod".to_string(), "Vito Cornelius".to_string()],
        ];
        let comb5 = vec![
            vec!["Leeloo".to_string(), "Vito Cornelius".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Ruby Rhod".to_string()],
            vec![
                "Korben Dallas".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
        ];
        let comb6 = vec![
            vec!["Leeloo".to_string(), "Diva Plavalaguna".to_string()],
            vec!["Korben Dallas".to_string(), "Vito Cornelius".to_string()],
            vec!["Ruby Rhod".to_string(), "Jean-Baptiste Zorg".to_string()],
        ];
        let comb7 = vec![
            vec!["Leeloo".to_string(), "Korben Dallas".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Ruby Rhod".to_string()],
            vec![
                "Jean-Baptiste Zorg".to_string(),
                "Vito Cornelius".to_string(),
            ],
        ];
        let comb8 = vec![
            vec!["Leeloo".to_string(), "Ruby Rhod".to_string()],
            vec![
                "Diva Plavalaguna".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
            vec!["Korben Dallas".to_string(), "Vito Cornelius".to_string()],
        ];
        let comb9 = vec![
            vec!["Leeloo".to_string(), "Jean-Baptiste Zorg".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Vito Cornelius".to_string()],
            vec!["Korben Dallas".to_string(), "Ruby Rhod".to_string()],
        ];
        let comb10 = vec![
            vec!["Leeloo".to_string(), "Vito Cornelius".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Korben Dallas".to_string()],
            vec!["Ruby Rhod".to_string(), "Jean-Baptiste Zorg".to_string()],
        ];
        let comb11 = vec![
            vec!["Leeloo".to_string(), "Diva Plavalaguna".to_string()],
            vec![
                "Korben Dallas".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
            vec!["Ruby Rhod".to_string(), "Vito Cornelius".to_string()],
        ];
        let comb12 = vec![
            vec!["Leeloo".to_string(), "Korben Dallas".to_string()],
            vec!["Diva Plavalaguna".to_string(), "Vito Cornelius".to_string()],
            vec!["Ruby Rhod".to_string(), "Jean-Baptiste Zorg".to_string()],
        ];

        let mut set = HashSet::new();

        let pairs = generate_permutations_deterministically(names.clone(), 0);
        assert_eq!(pairs, comb1);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 1);
        assert_eq!(pairs, comb2);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 2);
        assert_eq!(pairs, comb3);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 3);
        assert_eq!(pairs, comb4);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 4);
        assert_eq!(pairs, comb5);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 5);
        assert_eq!(pairs, comb6);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 6);
        assert_eq!(pairs, comb7);
        assert_eq!(pairs.iter().all(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 7);
        assert_eq!(pairs, comb8);
        assert_eq!(pairs.iter().all(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 8);
        assert_eq!(pairs, comb9);
        assert_eq!(pairs.iter().all(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 9);
        assert_eq!(pairs, comb10);
        assert_eq!(pairs.iter().all(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 10);
        assert_eq!(pairs, comb11);
        assert_eq!(pairs.iter().all(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 11);
        assert_eq!(pairs, comb12);
        assert_eq!(pairs.iter().all(|p| set.contains(p)), true);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });
    }

    #[test]
    fn test_generate_permutations_deterministically_odd() {
        let names = vec![
            "Leeloo".to_string(),
            "Diva Plavalaguna".to_string(),
            "Korben Dallas".to_string(),
            "Ruby Rhod".to_string(),
            "Jean-Baptiste Zorg".to_string(),
        ];
        let comb1 = vec![
            vec!["Leeloo".to_string(), "Diva Plavalaguna".to_string()],
            vec![
                "Korben Dallas".to_string(),
                "Ruby Rhod".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
        ];
        let comb2 = vec![
            vec!["Leeloo".to_string(), "Korben Dallas".to_string()],
            vec![
                "Diva Plavalaguna".to_string(),
                "Jean-Baptiste Zorg".to_string(),
                "Ruby Rhod".to_string(),
            ],
        ];
        let comb3 = vec![
            vec!["Leeloo".to_string(), "Ruby Rhod".to_string()],
            vec![
                "Diva Plavalaguna".to_string(),
                "Korben Dallas".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
        ];
        let comb4 = vec![
            vec!["Leeloo".to_string(), "Jean-Baptiste Zorg".to_string()],
            vec![
                "Diva Plavalaguna".to_string(),
                "Ruby Rhod".to_string(),
                "Korben Dallas".to_string(),
            ],
        ];

        let mut set = HashSet::new();

        let pairs = generate_permutations_deterministically(names.clone(), 0);
        assert_eq!(pairs, comb1);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 1);
        assert_eq!(pairs, comb2);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 2);
        assert_eq!(pairs, comb3);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 3);
        assert_eq!(pairs, comb4);
        assert_eq!(pairs.iter().any(|p| set.contains(p)), false);
        pairs.iter().for_each(|p| {
            set.insert(p.clone());
        });

        let pairs = generate_permutations_deterministically(names.clone(), 4);
        assert_eq!(pairs, comb1);

        let pairs = generate_permutations_deterministically(names.clone(), 5);
        assert_eq!(pairs, comb2);

        let pairs = generate_permutations_deterministically(names.clone(), 6);
        assert_eq!(pairs, comb3);

        let pairs = generate_permutations_deterministically(names.clone(), 7);
        assert_eq!(pairs, comb4);

        let pairs = generate_permutations_deterministically(names.clone(), 8);
        assert_eq!(pairs, comb1);

        let pairs = generate_permutations_deterministically(names.clone(), 9);
        assert_eq!(pairs, comb2);

        let pairs = generate_permutations_deterministically(names.clone(), 10);
        assert_eq!(pairs, comb3);

        let pairs = generate_permutations_deterministically(names.clone(), 11);
        assert_eq!(pairs, comb4);
    }

    #[test]
    fn test_increment_iteration() {
        let names_data = NamesData {
            names: vec!["Leeloo".to_string(), "Diva Plavalaguna".to_string()],
            greetings: "Hello".to_string(),
            iteration: 0,
        };
        let expected_names_data = NamesData {
            names: vec!["Leeloo".to_string(), "Diva Plavalaguna".to_string()],
            greetings: "Hello".to_string(),
            iteration: 1,
        };
        assert_eq!(increment_iteration(names_data), expected_names_data);

        let names_data = NamesData {
            names: vec![
                "Korben Dallas".to_string(),
                "Ruby Rhod".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
            greetings: "Hello".to_string(),
            iteration: 2,
        };
        let expected_names_data = NamesData {
            names: vec![
                "Korben Dallas".to_string(),
                "Ruby Rhod".to_string(),
                "Jean-Baptiste Zorg".to_string(),
            ],
            greetings: "Hello".to_string(),
            iteration: 3,
        };
        assert_eq!(increment_iteration(names_data), expected_names_data);
    }
}
