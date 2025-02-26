use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use clap::{Arg, App};

enum EntryType {
    All,
    File,
    Directory,
}

fn main() {
    let matches = App::new("BFS Regex Finder")
        .version("1.0")
        .about("Search for files/directories matching a regex pattern using BFS")
        .arg(
            Arg::with_name("regex")
                .required(true)
                .help("The regex pattern to match file/directory names")
                .index(1),
        )
        .arg(
            Arg::with_name("max_depth")
                .required(true)
                .help("Maximum directory depth to search")
                .index(2),
        )
        .arg(
            Arg::with_name("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .possible_values(&["all", "file", "dir"])
                .default_value("all")
                .help("Type of entries to search for (all, file, dir)"),
        )
        .arg(
            Arg::with_name("start_dir")
                .short('d')
                .long("dir")
                .takes_value(true)
                .default_value(".")
                .help("Starting directory for search"),
        )
        .get_matches();

    let pattern = matches.value_of("regex").unwrap();
    let max_depth: usize = matches.value_of("max_depth").unwrap()
        .parse()
        .expect("Invalid max depth value. Must be a non-negative integer.");
    
    let entry_type = match matches.value_of("type").unwrap() {
        "file" => EntryType::File,
        "dir" => EntryType::Directory,
        _ => EntryType::All,
    };
    
    let start_dir = matches.value_of("start_dir").unwrap();

    match Regex::new(pattern) {
        Ok(regex) => {
            bfs_search(start_dir, regex, max_depth, entry_type);
        }
        Err(e) => {
            eprintln!("Invalid regex pattern: {}", e);
            std::process::exit(1);
        }
    }
}

fn bfs_search(start_dir: &str, regex: Regex, max_depth: usize, entry_type: EntryType) {
    let mut queue = VecDeque::new();
    queue.push_back((PathBuf::from(start_dir), 0));

    while let Some((path, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }

        if let Ok(metadata) = fs::metadata(&path) {
            let is_dir = metadata.is_dir();
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            // Check if the entry matches the regex and type criteria
            let type_matches = match entry_type {
                EntryType::All => true,
                EntryType::File => !is_dir,
                EntryType::Directory => is_dir,
            };

            if type_matches && regex.is_match(&name) {
                println!("{}", path.display());
            }

            // If it's a directory and we haven't reached max depth, add its contents to the queue
            if is_dir && depth < max_depth {
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.filter_map(Result::ok) {
                        queue.push_back((entry.path(), depth + 1));
                    }
                }
            }
        }
    }
}
