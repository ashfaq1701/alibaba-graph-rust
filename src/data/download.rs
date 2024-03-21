extern crate reqwest;
extern crate anyhow;
use log::{info, error};

use std::{fs::File, io::copy, time::Duration};
use anyhow::{anyhow, Result};

pub fn download(url: &str, destination: &str, retries: u32) -> Result<()> {
    info!("Starting download of {} to the {}", url, destination);

    for attempt in 0..=retries {
        let response = reqwest::blocking::get(url);

        match response {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let mut file = File::create(destination)?;
                    copy(&mut resp, &mut file)?;
                    info!("File downloaded successfully to: {}", destination);
                    return Ok(());
                } else {
                    error!("Download attempt {} failed. Status code: {}", attempt + 1, resp.status());
                }
            }
            Err(e) => {
                error!("Download attempt {} failed. Error: {}", attempt + 1, e)
            }
        }

        let backoff_duration = Duration::from_secs(2u32.pow(attempt) as u64);
        std::thread::sleep(backoff_duration);
    }

    Err(anyhow!("Failed to download {} after {} retries", url, retries))
}
