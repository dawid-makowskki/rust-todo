use clap::{arg, command, Command};
use std::{
    fs::{OpenOptions, write},
    io::{BufRead, BufReader},
};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("new")
                .about("It create new task in todo list.")
                .arg(
                    arg!(-m --msg <MESSAGE> "Todo content")
                        .required(true)
                ),
        )
        .get_matches();

    const FILE_PATH: &str = "todo.txt";

    let mut file_lines = read_lines_from_file(FILE_PATH);
    let last_todo_number = file_lines
    .last()
    .and_then(|line| line.split("_").next())
    .and_then(|number| number.parse().ok())
    .unwrap_or(0);

    println!("{:#?}", last_todo_number);


    if let Some(new_matches) = matches.subcommand_matches("new") {
        println!("there is a match for subcommand");
        if let Some(msg) = new_matches.get_one::<String>("msg") {
        println!("there is a match for arg for subcommand");
            file_lines.push(format!("{}_{}", last_todo_number + 1, msg));

            write(FILE_PATH, file_lines.join("\n")).expect("");
        }
    }

    if file_lines.len() == 0 {
        println!("There are no todo items yet! Add task with `todo new`")
    } else {
        for line in file_lines {
            println!("{}", line)
        }
    }
}

fn read_lines_from_file(path: &str) -> Vec<String> {
    // Open the file, creating it if it doesn't exist
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|_| {
            // If there was an error opening the file, return an empty vector
            return OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Failed to create file");
        });

    // Create a BufReader to efficiently read lines from the file
    let reader = BufReader::new(&file);

    // Read lines into a Vec<String>
    reader
        .lines()
        .map(|line| line.unwrap_or_else(|_| String::new()))
        .collect()
}
