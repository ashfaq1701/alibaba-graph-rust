use std::collections::HashMap;
use std::fs::File;
use tar::Archive;
use flate2::read::GzDecoder;
use crate::data::structs::TimeBreakdown;
use log::{error};

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
    let file_start = (start / 180) * 180;
    let file_end = (end / 180) * 180;

    let mut file_bounds: Vec<(u32, u32)> = Vec::new();
    let mut current = file_start;

    while current + 180 <= file_end {
        file_bounds.push((current, current + 180));
        current += 180;
    }

    file_bounds
}
