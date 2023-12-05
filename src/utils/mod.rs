use std::collections::HashMap;
use std::fs::File;
use tar::Archive;
use flate2::read::GzDecoder;

pub fn get_time_breakdown<'a>(time: u32) -> HashMap<&'a str, u32> {
    let day = time / (24 * 60 * 60);
    let hour = (time - day * 24 * 60 * 60) / (60 * 60);
    let minute = (time - day * 24 * 60 * 60 - hour * 60 * 60) / 60;
    let second = time - day * 24 * 60 * 60 - hour * 60 * 60 - minute * 60;
    let mut time_map: HashMap<&str, u32> = HashMap::new();
    time_map.insert("day", day);
    time_map.insert("hour", hour);
    time_map.insert("minute", minute);
    time_map.insert("second", second);
    time_map
}

pub fn get_start_end_time_given_breakdown(
    start_day: u32,
    start_hour: u32,
    start_minute: u32,
    start_second: u32,
    end_day: u32,
    end_hour: u32,
    end_minute: u32,
    end_second: u32
) -> (u32, u32) {
    let start_time = start_day * 24 * 60 * 60 +
        start_hour * 60 * 60 +
        start_minute * 60 +
        start_second;

    let end_time = end_day * 24 * 60 * 60 +
        end_hour * 60 * 60 +
        end_minute * 60 +
        end_second;

    (start_time, end_time)
}

pub fn extract_gz_file(file_path: &String, dest_path: &String) -> Result<(), &'static str> {
    let file = File::open(file_path).expect("Failed to open file");
    let gz_decoder = GzDecoder::new(file);
    let mut archive = Archive::new(gz_decoder);
    archive.unpack(dest_path).expect("Failed to unpack archive");
    Ok(())
}
