pub mod env_params;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;
use flate2::read::GzDecoder;
use crate::data::structs::{TimeBreakdown, WindowIndexingType};
use log::{error};
use anyhow::{anyhow, Result};
use crate::utils::env_params::{get_file_duration_in_seconds, get_windows_directory};

pub fn get_time_breakdown<'a>(time: u32) -> TimeBreakdown {
    let day = time / (24 * 60 * 60);
    let hour = (time - day * 24 * 60 * 60) / (60 * 60);
    let minute = (time - day * 24 * 60 * 60 - hour * 60 * 60) / 60;
    let second = time - day * 24 * 60 * 60 - hour * 60 * 60 - minute * 60;
    TimeBreakdown { day, hour, minute, second }
}

pub fn get_start_end_time_given_breakdown(
    start: &TimeBreakdown,
    end: &TimeBreakdown
) -> (u32, u32) {
    let start_time = start.day * 24 * 60 * 60 +
        start.hour * 60 * 60 +
        start.minute * 60 +
        start.second;

    let end_time = end.day * 24 * 60 * 60 +
        end.hour * 60 * 60 +
        end.minute * 60 +
        end.second;

    (start_time, end_time)
}

pub fn extract_gz_file(file_path: &String, dest_path: &String) -> Result<(), &'static str> {
    let file = File::open(file_path).expect("Failed to open file");
    let gz_decoder = GzDecoder::new(file);
    let mut archive = Archive::new(gz_decoder);
    archive.unpack(dest_path).expect("Failed to unpack archive");
    Ok(())
}

pub fn get_int_option_value(options: &HashMap<&str, &str>, k: &str) -> Option<u32> {
    match options.get(k) {
        Some(str_value) => {
            match str_value.parse::<u32>() {
                Ok(value) => {
                    Some(value)
                }
                _ => {
                    error!("Couldn't parse integer value from options");
                    None
                }
            }
        }
        _ => {
            None
        }
    }
}

pub fn get_windows(
    start: u32,
    end: u32,
    window: u32
) -> Vec<(u32, u32)> {
    let mut windows: Vec<(u32, u32)> = Vec::new();
    let mut current = start;

    while current + window <= end {
        windows.push((current, current + window));
        current = current + window;
    }

    windows
}

pub fn get_window_count(
    start: u32,
    end: u32,
    window: u32
) -> u32 {
    (end - start) / window
}

pub fn get_file_bounds(start: u32, end: u32) -> Vec<(u32, u32)> {
    let file_duration = get_file_duration_in_seconds();
    let file_start = (start / file_duration) * file_duration;
    let file_end = (end / file_duration) * file_duration;

    let mut file_bounds: Vec<(u32, u32)> = Vec::new();
    let mut current = file_start;

    while current + file_duration <= file_end {
        file_bounds.push((current, current + file_duration));
        current += file_duration;
    }

    file_bounds
}

pub fn get_starting_window_idx(
    idx: usize,
    first_file_window_count: u32,
    window_count: u32,
    import_start_time: u32,
    indexing_type: &WindowIndexingType,
    window_size: u32
) -> u32 {
    let offset_index = match indexing_type {
        WindowIndexingType::FromZero => 0,
        WindowIndexingType::SeqFromStart => import_start_time / window_size
    };

    if idx == 0 {
        offset_index
    } else {
        offset_index + first_file_window_count + (idx - 1) as u32 * window_count
    }
}

pub fn get_resolved_windows_dir() -> String {
    match get_windows_directory() {
        Some(windows_dir) => windows_dir,
        _ => {
            let pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            format!("{}/data/windows", pathbuf.to_str().unwrap())
        }
    }
}

pub fn get_files_in_directory(dir_path: &String) -> Result<Vec<String>> {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let file_names = entries
                .filter(|entry| entry.is_ok())
                .map(|entry| entry.unwrap())
                .filter(|entry| entry.path().is_file())
                .map(|entry| entry.file_name().into_string().unwrap())
                .collect();

            Ok(file_names)
        }
        Err(_) => {
            Err(anyhow!("Error reading files from directory {}", dir_path))
        }
    }

}
