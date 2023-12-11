extern crate reqwest;
extern crate anyhow;
use log::{info, error};

use std::{fs::File, io::copy};
use anyhow::Result;

pub fn download(url: &str, destination: &str) -> Result<()> {
    info!("Staring download of {} to the {}", url, destination);
    let mut response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        error!("Failed to download file. Status code: {}", response.status());
        return Ok(());
    }

    let mut file = File::create(destination)?;

    copy(&mut response, &mut file)?;

    info!("File downloaded successfully to: {}", destination);

    Ok(())
}
