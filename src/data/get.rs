use std::collections::HashMap;
use std::path::PathBuf;
use super::download;

pub fn load_files<'a>(start: u32, end: u32, window_size: u32, overlap: u32) {
    let start_time_breakdown = get_time_breakdown(start);
    let end_time_breakdown = get_time_breakdown(end);
    download_raw_files(&start_time_breakdown, &end_time_breakdown);
}

pub fn download_raw_files<'a>(
    start_time_map: &'a HashMap<&'a str, u32>,
    end_time_map: &'a HashMap<&'a str, u32>) -> Vec<String> {
    let start_minute = get_callgraph_minute_value(start_time_map);
    let mut end_minute = get_callgraph_minute_value(end_time_map);
    if let Some(second) = end_time_map.get("second") {
        if *second > 0 {
            end_minute += 1;
        }
    }

    let start_idx = start_minute / 3;
    let end_idx = (f64::ceil(end_minute as f64 / 3.0) as u32) - 1;

    let base_url = "https://aliopentrace.oss-cn-beijing.aliyuncs.com/v2022MicroservicesTraces/CallGraph";
    let pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dest_dir = format!("{}/data/raw", pathbuf.to_str().unwrap());

    let mut downloaded_file_paths: Vec<String> = Vec::new();

    for i in start_idx..=end_idx {
        let file_name = format!("CallGraph_{}.tar.gz", i);
        let file_url = format!("{}/{}", base_url, file_name);
        let dest_file = format!("{}/{}", dest_dir, file_name);
        match download::download(&file_url, &dest_file) {
            Ok(_) => {
                downloaded_file_paths.push(dest_file);
            }
            _ => {
                eprintln!("Error in downloading {}", file_name);
            }
        }
    }

    downloaded_file_paths
}

pub fn get_callgraph_minute_value(time_map: &HashMap<&str, u32>) -> u32 {
    let mut minute_value: u32 = 0;

    if let Some(day) = time_map.get("day") {
        minute_value += day * 24 * 60;
    };

    if let Some(hour) = time_map.get("hour") {
        minute_value += hour * 60;
    };

    if let Some(minute) = time_map.get("minute") {
        minute_value += minute;
    };

    minute_value
}

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