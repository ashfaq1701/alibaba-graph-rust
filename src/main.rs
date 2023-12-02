mod data;

use std::env;

fn main() {
    let command = env::args().nth(1);

    match command.as_deref() {
        Some("load") => {
            match (env::args().nth(2), env::args().nth(3)) {
                (Some(start_str), Some(end_str)) => {
                    match run_get_data(&start_str, &end_str) {
                        Ok(_) => {
                            println!("Downloaded data successfully");
                        }
                        Err(msg) => {
                            eprintln!("Error running downloader {}", msg);
                        }
                    };
                }
                _ => {
                    eprintln!("Load requires start and end parameters");
                }
            }
        }
        Some(_) => {
            eprintln!("Invalid command");
        }
        _ => {
            eprintln!("No command");
        }
    }
}

fn run_get_data(start_str: &String, end_str: &String) -> Result<(), &'static str> {
    let maybe_start = start_str.parse::<u32>();
    let maybe_end = end_str.parse::<u32>();

    return match (maybe_start, maybe_end) {
        (Ok(start), Ok(end)) => {
            data::get::load_files(start, end);
            Ok(())
        }
        (Err(_), Ok(_)) => {
            Err("Invalid start parameter passed")
        }
        (Ok(_), Err(_)) => {
            Err("Invalid end parameter passed")
        }
        _ => {
            Err("Both invalid parameters passed")
        }
    }
}
