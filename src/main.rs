use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    let path = get_path_or_quit();
    let input = read_file_or_quit(path);
    let json = deserialize_or_quit(input);

    prettify_or_quit(json);
}

/// Naive argument parsing that returns a single input file from which to read.
/// If any of the previously described actions fail then the program halts and
/// outputs a help message.
fn get_path_or_quit() -> String {
    let usage = "Usage: pj <file_name>";
    let args: Vec<String> = env::args().collect();

    // Expect the first argument to be the program name and the second to be the
    // input string (i.e. file name)
    if args.len() != 2 {
        let error = "Error: expected single input file name";
        eprintln!("{}\n{}", error, usage);
        process::exit(1);
    }

    let file_name = args[1].clone();

    if file_name.is_empty() {
        let error = "Error: File name required";
        eprintln!("{}\n{}", error, usage);
        process::exit(1);
    }

    file_name
}

/// Returns the contents of the given file name as a string or terminates the
/// program with a non-zero exit code and an error message
fn read_file_or_quit(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(why) => {
            eprintln!("Error: Unable to read file ({})", why);
            process::exit(1);
        }
    }
}

/// Returns a deserialized version of the input or terminates the program with
/// a non-zero exit code and an error message
fn deserialize_or_quit(input: String) -> serde_json::Value {
    match serde_json::from_str(&input) {
        Ok(json) => json,
        Err(why) => {
            eprintln!("Error: Unable to deserialize input ({}):\n{}", why, input);
            process::exit(1);
        }
    }
}

/// Prints pretty-formatted version of input or terminates the program with a
/// non-zero exit code and an error message
fn prettify_or_quit(json: serde_json::Value) {
    let writer = io::BufWriter::new(io::stdout());
    let result = serde_json::to_writer_pretty(writer, &json);

    if let Err(why) = result {
        eprintln!("Error: Unable to prettify JSON ({}):\n{}", why, json);
        process::exit(1);
    };

    // The serde writer doesn't add a polite newline. This ensures the user's
    // cursor is returned to the start of a newline in the terminal.
    println!();
}
