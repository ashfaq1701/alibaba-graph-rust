mod data;
mod utils;
mod graph;
mod process;

use std::env;
use std::collections::HashMap;
use log::{info, error};
use anyhow::{anyhow, Result};
use crate::data::init::run_get_data;
use crate::process::init::run_process_data;
use crate::utils::env_params::load_env_files;

fn main() -> Result<()> {
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
                    Ok(())
                }
                Err(e) => {
                    error!("Error downloading data {:?}", e);
                    Err(anyhow!(e))
                }
            }
        }
        Some("process") => {
            match run_process_data(&options) {
                Ok(result) => {
                    info!("Processed data successfully {:?}", result);
                    Ok(())
                }
                Err(e) => {
                    error!("Error loading data {:?}", e);
                    Err(anyhow!(e))
                }
            }
        }
        Some(_) => {
            Err(anyhow!("Invalid command"))
        }
        _ => {
            Err(anyhow!("No command"))
        }
    }
}


