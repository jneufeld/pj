use std::env;
use std::fs;
use std::process;

use serde_json::Value;

fn main() {
    let path = input_file();
    let json = contents(&path);
    let json = pretty(json);

    println!("{}", json);
}

fn input_file() -> String {
    let usage = "Usage: pj <file_name>";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let error = "Error: Expected input file name";
        eprintln!("{}\n{}", error, usage);
        process::exit(1);
    }

    let file_name = &args[1];

    if file_name.len() < 1 {
        let error = "Error: File name required";
        eprintln!("{}\n{}", error, usage);
        process::exit(1);
    }

    file_name.to_owned()
}

fn contents(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            let error = format!("Error: Unable to read '{}'. {}.", path, err);
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

fn pretty(json: String) -> String {
    let result: Value = match serde_json::from_str(&json) {
        Ok(json) => json,
        Err(err) => {
            let error = format!("Error: Unable deserialize JSON. {}.", err);
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let result = match serde_json::to_string_pretty(&result) {
        Ok(json) => json,
        Err(err) => {
            let error = format!("Error: Unable prettify JSON. {}.", err);
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    result
}