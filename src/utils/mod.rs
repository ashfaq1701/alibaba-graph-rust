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
