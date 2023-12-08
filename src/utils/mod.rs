use std::collections::HashMap;
use std::fs::File;
use num_integer::gcd;
use tar::Archive;
use flate2::read::GzDecoder;
use raphtory::algorithms::cores::k_core::k_core;
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
    let mut batches = 1;

    while batches <= total {
        let remainder = (s * batches - o) % (w - o);
        if remainder == 0 {
            return batches;
        }

        batches += 1;
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