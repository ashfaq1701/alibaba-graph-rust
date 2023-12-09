use std::collections::HashMap;
use std::fs::File;
use tar::Archive;
use flate2::read::GzDecoder;
use crate::data::structs::TimeBreakdown;

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

pub fn calculate_file_batch_size(s: u32, w: u32, o: u32, total: u32) -> u32 {
    for batches in 1..=total {
        let remainder = (s * batches - o) % (w - o);
        if remainder == 0 {
            return batches;
        }
    }

    total
}

pub fn create_windows<T>(data: Vec<T>, window_size: usize) -> Vec<Vec<T>>
    where T: Clone,
{
    let mut windows = Vec::new();

    for i in (0..data.len()).step_by(window_size) {
        let window: Vec<T> = data[i..].iter().cloned().take(window_size).collect();
        windows.push(window);
    }

    windows
}

pub fn get_int_option_value(options: &HashMap<&str, &str>, k: &str) -> Option<u32> {
    match options.get(k) {
        Some(str_value) => {
            match str_value.parse::<u32>() {
                Ok(value) => {
                    Some(value)
                }
                _ => {
                    eprintln!("Couldn't parse integer value from options");
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
    batch_idx: u32,
    batch_size: u32,
    window_size: u32,
    overlap: u32,
    file_start: u32,
    start: u32,
    end: u32
) -> Vec<(u32, u32)> {
    let window_start = file_start + 180 * batch_size * batch_idx;
    let window_end = file_start + window_start + 180 * batch_size;
    let mut windows: Vec<(u32, u32)> = Vec::new();
    let mut current = window_start;

    while current + window_size <= window_end && current + window_size <= end {
        if current < start {
            continue
        }

        windows.push((current, current + window_size - 1));
        current = current + window_size - overlap;
    }

    windows
}

pub fn get_window_count(dataset_size: u32, window_size: u32, overlap: u32) -> u32 {
    ((dataset_size - window_size) as f64 / (window_size - overlap) as f64).ceil() as u32 + 1
}

pub fn get_closest_file_start(start: u32) -> u32 {
    start - (start % 180)
}
