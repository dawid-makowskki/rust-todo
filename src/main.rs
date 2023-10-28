use clap::{arg, command, ArgMatches, Command};
use std::{
    fs::{write, File},
    io::{BufRead, BufReader, Write},
};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("new")
                .about("It create new task in todo list.")
                .arg(arg!(-m --msg <MESSAGE> "Todo content").required(true)),
        )
        .subcommand(
            Command::new("done")
                .about("It marks a task as done")
                .arg(arg!(-n --number <NUMBER> "Number of task").required(true)),
        )
        .subcommand(
            Command::new("undone")
                .about("It marks a task as undone")
                .arg(arg!(-n --number <NUMBER> "Number of task").required(true)),
        )
        .subcommand(
            Command::new("del")
                .about("It deletes a task")
                .arg(arg!(-n --number <NUMBER> "Number of task").required(true)),
        )
        .get_matches();

    const FILE_PATH: &str = "todo.txt";

    let mut file_lines = read_lines_from_file(FILE_PATH);

    let last_task_number = file_lines
        .last()
        .and_then(|line| line.split("_").next())
        .and_then(|number| number.parse().ok())
        .unwrap_or(0);

    if let Some(subcommand) = matches.subcommand() {
        match subcommand {
            ("new", new_matches) => {
                if let Some(msg) = new_matches.get_one::<String>("msg") {
                    file_lines.push(format!("{}_{}", last_task_number + 1, msg));
                }
            }
            ("done" | "undone", sub_matches) => {
                if let Some(number) = sub_matches.get_one::<String>("number") {
                    let parsed_number = number.parse().ok().unwrap_or(-1);
                    if is_in_range(parsed_number, 0, last_task_number) {
                        for line in &mut file_lines {
                            let updated_task = get_updated_task(subcommand, number, line);
                            *line = updated_task;
                        }
                    } else {
                        eprintln!("wrong number");
                    }
                }
            }
            ("del", del_matches) => {
                if let Some(number) = del_matches.get_one::<String>("number") {
                    let parsed_number = number.parse().ok().unwrap_or(-1);
                    if is_in_range(parsed_number, 0, last_task_number) {
                        file_lines.retain(|line| {
                            let split: Vec<&str> = line.split("_").collect();
                            if split.len() > 0 {
                                return split[0] != number;
                            }
                            false
                        });
                        file_lines = file_lines
                            .iter()
                            .enumerate()
                            .map(|(idx, line)| {
                                let split: Vec<&str> = line.split("_").collect();
                                if split.len() > 0 {
                                    if split.len() == 2 {
                                        return format!("{}_{}", idx + 1, split[1]);
                                    } else if split.len() == 3 {
                                        return format!("{}_{}_{}", idx + 1, split[1], split[2]);
                                    } else {
                                        return line.to_string();
                                    }
                                }
                                return line.to_string();
                            })
                            .collect();
                    } else {
                        eprintln!("wrong number");
                    }
                }
            }
            _ => {}
        }
    }

    write(FILE_PATH, file_lines.join("\n")).expect("");

    if file_lines.len() == 0 {
        println!("There are no tasks yet! Add task with `todo new`")
    } else {
        for line in file_lines {
            println!("{}", format_task(line))
        }
    }
}

fn read_lines_from_file(path: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let file_result = File::open(path);

    match file_result {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    lines.push(line);
                }
            }
        }
        Err(_) => {
            let mut file = File::create(path).expect("Filed to create file");
            file.write_all(b"").expect("Failed to write to file");
        }
    }

    lines
}

fn is_in_range(number: i32, min: i32, max: i32) -> bool {
    return number >= min && number <= max;
}

fn get_updated_task(subcommand: (&str, &ArgMatches), number: &String, line: &String) -> String {
    let mut split: Vec<&str> = line.split("_").collect();
    if split.len() > 0 && split[0] == number {
        if subcommand.0 == "done" {
            if split.len() == 2 {
                split.push("#DONE");
            }
            if split.len() == 3 {
                split[2] = "#DONE";
            }
        } else if split.len() == 3 {
            split.pop();
        }
    }
    split.join("_")
}

fn format_task(line: String) -> String {
    let split: Vec<&str> = line.split("_").collect();
    if split.len() == 2 {
        return format!("{}. {}", split[0], split[1]);
    }
    format!("{}. {} {}", split[0], split[1], split[2])
}
