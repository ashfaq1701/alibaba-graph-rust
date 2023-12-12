extern crate reqwest;
extern crate anyhow;
use log::{info, error};

use std::{fs::File, io::copy};
use anyhow::{anyhow, Result};

pub fn download(url: &str, destination: &str) -> Result<()> {
    info!("Starting download of {} to the {}", url, destination);
    let maybe_response = reqwest::blocking::get(url);

    let mut response = if let Ok(resp) = maybe_response {
        resp
    } else {
        return Err(anyhow!("Failed to get response of {}", url));
    };

    if !response.status().is_success() {
        error!("Failed to download file. Status code: {}", response.status());
        return Err(anyhow!("Failed to download {}", url));
    }

    let mut file = File::create(destination)?;

    if let Ok(_) = copy(&mut response, &mut file) {
        info!("File downloaded successfully to: {}", destination);
        Ok(())
    } else {
        Err(anyhow!("Failed to copy response from {} to {}", url, destination))
    }
}
