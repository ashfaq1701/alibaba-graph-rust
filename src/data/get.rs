extern crate anyhow;

use std::collections::HashMap;
use std::ops::Deref;
use threadpool::ThreadPool;
use std::path::PathBuf;
use super::download;
use std::sync::{Arc, Mutex};
use crate::utils;
use crate::graph;

pub fn load_files<'a>(start: u32, end: u32, window_size: u32, overlap: u32) {
    let start_time_breakdown = utils::get_time_breakdown(start);
    let end_time_breakdown = utils::get_time_breakdown(end);
    let mut downloaded_files = download_raw_files(
        &start_time_breakdown,
        &end_time_breakdown
    );
    downloaded_files.sort();
    let loaded_graph_windows = graph::load::load_event_files(&downloaded_files);
    println!("{:?}", loaded_graph_windows);
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

    let base_url = Arc::new("https://aliopentrace.oss-cn-beijing.aliyuncs.com/v2022MicroservicesTraces/CallGraph");
    let pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dest_dir = Arc::new(format!("{}/data/raw", pathbuf.to_str().unwrap()));

    let mut downloaded_file_paths = Arc::new(Mutex::new(Vec::<String>::new()));

    let n_workers = 5;
    let pool = ThreadPool::new(n_workers);

    for i in start_idx..=end_idx {

        let shared_base_url = Arc::clone(&base_url);
        let shared_dest_dir = Arc::clone(&dest_dir);
        let shared_downloaded_file_paths = Arc::clone(&downloaded_file_paths);

        pool.execute(move || {
            let file_name = format!("CallGraph_{}.tar.gz", i);
            let file_url = format!("{}/{}", shared_base_url.deref(), file_name);
            let dest_file = format!("{}/{}", shared_dest_dir.deref(), file_name);
            match download::download(&file_url, &dest_file) {
                Ok(_) => {
                    let mut unlocked_paths = shared_downloaded_file_paths.lock().unwrap();
                    unlocked_paths.push(dest_file);
                }
                _ => {
                    eprintln!("Error in downloading {}", file_name);
                }
            }
        });
    };

    pool.join();

    let downloaded_files = downloaded_file_paths.lock().unwrap().clone();
    downloaded_files
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
