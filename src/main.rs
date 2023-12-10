mod data;
mod utils;
mod graph;

use std::env;
use std::collections::HashMap;
use crate::data::structs::ConnectionProp;
use crate::utils::get_int_option_value;
use anyhow::{anyhow, Result};

fn main() {
    let command = env::args().nth(1);
    let args: Vec<String> = env::args().collect();
    let mut options: HashMap<&str, &str> = HashMap::new();
    for arg in args.iter().skip(2) {
        let parts: Vec<&str> = arg
            .split("=")
            .collect();
        if parts.len() != 2 {
            eprintln!("Wrong option passed {}", arg);
            continue;
        }

        options.insert(parts[0], parts[1]);
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

fn run_get_data(options: &HashMap<&str, &str>) -> Result<()> {
    let mut start_time: u32 = 0;
    let mut end_time: u32 = 0;

    match (get_int_option_value(options, "start_time"), get_int_option_value(options,"end_time")) {
        (Some(start), Some(end)) => {
            start_time = start;
            end_time = end;
        }
        (Some(start), None) => {
            start_time = start;
        }
        (None, Some(end)) => {
            end_time = end;
        }
        _ => {}
    }

    let start_day = match get_int_option_value(options, "start_day") {
        Some(sd) => { sd }
        _ => { 0 }
    };

    let start_hour = match get_int_option_value(options, "start_hour") {
        Some(sh) => { sh }
        _ => { 0 }
    };

    let start_minute = match get_int_option_value(options, "start_minute") {
        Some(sm) => { sm }
        _ => { 0 }
    };

    let start_second = match get_int_option_value(options, "start_second") {
        Some(ss) => { ss }
        _ => { 0 }
    };

    let end_day = match get_int_option_value(options, "end_day") {
        Some(ed) => { ed }
        _ => { 0 }
    };

    let end_hour = match get_int_option_value(options, "end_hour") {
        Some(eh) => { eh }
        _ => { 0 }
    };

    let end_minute = match get_int_option_value(options, "end_minute") {
        Some(em) => { em }
        _ => { 0 }
    };

    let end_second = match get_int_option_value(options, "end_second") {
        Some(es) => { es }
        _ => { 0 }
    };

    let connection_prop = if let Some(connection_prop_str) = options.get("connection_prop") {
        if *connection_prop_str == "instance_id" {
            ConnectionProp::InstanceId
        } else {
            ConnectionProp::MicroserviceId
        }
    } else {
        ConnectionProp::MicroserviceId
    };

    let start = data::structs::TimeBreakdown {
        day: start_day,
        hour: start_hour,
        minute: start_minute,
        second: start_second
    };

    let end = data::structs::TimeBreakdown {
        day: end_day,
        hour: end_hour,
        minute: end_minute,
        second: end_second
    };

    let (calc_start_time, calc_end_time) = utils::get_start_end_time_given_breakdown(
        &start,
        &end
    );

    if start_time == 0 {
        start_time = calc_start_time;
    }

    if end_time == 0 {
        end_time = calc_end_time;
    }

    if end_time <= start_time {
        return Err(anyhow!("Invalid start or end parameter passed"));
    }

    match (get_int_option_value(options, "window_size"), get_int_option_value(options, "overlap")) {
        (Some(window_size), Some(overlap)) => {
            let loaded_window_files = data::get::load_files(
                start_time,
                end_time,
                window_size,
                overlap,
                &connection_prop
            )?;

            println!("Windowed graphs are stored in the following files {:?}", loaded_window_files);

            Ok(())
        }
        (Some(window_size), None) => {
            data::get::load_files(start_time, end_time, window_size, 0, &connection_prop)?;
            Ok(())
        }
        _ => {
            Err(anyhow!("Window size is a required parameter"))
        }
    }
}
