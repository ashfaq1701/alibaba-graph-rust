mod data;

use std::env;
use std::collections::HashMap;

fn main() {
    let command = env::args().nth(1);
    let args: Vec<String> = env::args().collect();
    let mut options: HashMap<&str, u32> = HashMap::new();
    for arg in args.iter().skip(2) {
        let parts: Vec<&str> = arg
            .split("=")
            .collect();
        if parts.len() != 2 {
            eprintln!("Wrong option passed {}", arg);
            continue;
        }

        let key = parts[0];
        let maybe_value = parts[1].parse::<u32>();
        match maybe_value {
            Ok(value) => {
                options.insert(key, value);
            }
            _ => {
                eprintln!("Invalid option passed {}", parts[1]);
                continue;
            }
        }
    }

    match command.as_deref() {
        Some("load") => {
            match run_get_data(&options) {
                Ok(_) => {
                    println!("Downloaded data successfully");
                }
                Err(msg) => {
                    eprintln!("Error running downloader {}", msg);
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

fn run_get_data(options: &HashMap<&str, u32>) -> Result<(), &'static str> {
    let mut start_time: u32 = 0;
    let mut end_time: u32 = 0;

    match (options.get("start_time"), options.get("end_time")) {
        (Some(start), Some(end)) => {
            start_time = *start;
            end_time = *end;
        }
        (Some(start), None) => {
            start_time = *start;
        }
        (None, Some(end)) => {
            end_time = *end;
        }
        _ => {}
    }

    let start_day = match options.get("start_day") {
        Some(sd) => { *sd }
        _ => { 0 }
    };

    let start_hour = match options.get("start_hour") {
        Some(sh) => { *sh }
        _ => { 0 }
    };

    let start_minute = match options.get("start_minute") {
        Some(sm) => { *sm }
        _ => { 0 }
    };

    let start_second = match options.get("start_second") {
        Some(ss) => { *ss }
        _ => { 0 }
    };

    let end_day = match options.get("end_day") {
        Some(ed) => { *ed }
        _ => { 0 }
    };

    let end_hour = match options.get("end_hour") {
        Some(eh) => { *eh }
        _ => { 0 }
    };

    let end_minute = match options.get("end_minute") {
        Some(em) => { *em }
        _ => { 0 }
    };

    let end_second = match options.get("end_second") {
        Some(es) => { *es }
        _ => { 0 }
    };

    let (calc_start_time, calc_end_time) = data::get::get_start_end_time_given_breakdown(
        start_day,
        start_hour,
        start_minute,
        start_second,
        end_day,
        end_hour,
        end_minute,
        end_second
    );

    if start_time == 0 {
        start_time = calc_start_time;
    }

    if end_time == 0 {
        end_time = calc_end_time;
    }

    if end_time <= start_time {
        Err("Invalid start or end parameter passed")
    } else {
        data::get::load_files(start_time, end_time);
        Ok(())
    }
}
