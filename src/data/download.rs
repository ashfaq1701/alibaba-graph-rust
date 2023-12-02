extern crate reqwest;
extern crate anyhow;

use std::{fs::File, io::copy};
use anyhow::Result;

pub fn download(url: &str, destination: &str) -> Result<()> {
    let mut response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        eprintln!("Failed to download file. Status code: {}", response.status());
        return Ok(());
    }

    let mut file = File::create(destination)?;

    copy(&mut response, &mut file)?;

    println!("File downloaded successfully to: {}", destination);

    Ok(())
}


