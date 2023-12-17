mod data;
mod utils;
mod graph;
mod process;

use std::env;
use std::collections::HashMap;
use log::{info, error};
use crate::data::init::run_get_data;
use crate::process::init::run_process_data;
use crate::utils::env_params::load_env_files;

fn main() {
    load_env_files();

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let command = env::args().nth(1);
    let args: Vec<String> = env::args().collect();
    let mut options: HashMap<&str, &str> = HashMap::new();
    for arg in args.iter().skip(2) {
        let parts: Vec<&str> = arg
            .split("=")
            .collect();
        if parts.len() != 2 {
            error!("Wrong option passed {}", arg);
            continue;
        }

        options.insert(parts[0], parts[1]);
    }

    match command.as_deref() {
        Some("load") => {
            match run_get_data(&options) {
                Ok(_) => {
                    info!("Downloaded data successfully");
                }
                Err(msg) => {
                    error!("Error running downloader {}", msg);
                }
            }
        }
        Some("process") => {
            match run_process_data(&options) {
                Ok(processing_result) => {
                    info!("Processed data successfully {:?}", processing_result);
                }
                _ => {
                    error!("Error processing the windows");
                }
            }
        }
        Some(_) => {
            error!("Invalid command");
        }
        _ => {
            error!("No command");
        }
    }
}


