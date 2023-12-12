extern crate anyhow;

use std::fs;
use std::path::PathBuf;
use super::download;
use crate::utils;
use crate::graph;
use anyhow::Result;
use rayon::prelude::*;
use super::structs::{ConnectionProp, TimeBreakdown};
use log::{info};

pub fn load_files<'a>(
    start: u32,
    end: u32,
    window_size: u32,
    connection_prop: &ConnectionProp
) -> Result<Vec<String>> {
    let start_time_breakdown = utils::get_time_breakdown(start);
    let end_time_breakdown = utils::get_time_breakdown(end);
    let mut downloaded_files = download_raw_files(
        &start_time_breakdown,
        &end_time_breakdown
    )?;

    downloaded_files.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split("_").collect();
        let a1 = a_parts[1].to_string();
        let a1_parts: Vec<&str> = a1.split(".").collect();
        let a_i = match a1_parts[0].parse::<u32>(){
            Ok(i) => {
                i
            }
            _ => {
                0
            }
        };

        let b_parts: Vec<&str> = b.split("_").collect();
        let b1 = b_parts[1].to_string();
        let b1_parts: Vec<&str> = b1.split(".").collect();
        let b_i = match b1_parts[0].parse::<u32>(){
            Ok(i) => {
                i
            }
            _ => {
                0
            }
        };

        a_i.cmp(&b_i)
    });

    let loaded_window_files = graph::load::load_event_files(
        downloaded_files,
        window_size,
        connection_prop,
        start,
        end
    )?;

    Ok(loaded_window_files)
}

pub fn download_raw_files<'a>(
    start_time_breakdown: &'a TimeBreakdown,
    end_time_breakdown: &'a TimeBreakdown
) -> Result<Vec<String>> {

    let start_minute = get_callgraph_minute_value(start_time_breakdown);
    let mut end_minute = get_callgraph_minute_value(end_time_breakdown);
    if end_time_breakdown.second > 0 {
        end_minute += 1;
    }

    let start_idx = start_minute / 3;
    let end_idx = (f64::ceil(end_minute as f64 / 3.0) as u32) - 1;

    let base_url = "https://aliopentrace.oss-cn-beijing.aliyuncs.com/v2022MicroservicesTraces/CallGraph".to_string();
    let pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dest_dir = format!("{}/data/raw", pathbuf.to_str().unwrap());

    let maybe_downloaded_files: Result<Vec<String>> = (start_idx..=end_idx)
        .into_par_iter()
        .map(|i: u32| {
            let file_name = format!("CallGraph_{}.tar.gz", i);
            let file_url = format!("{}/{}", &base_url, file_name);
            let dest_file = format!("{}/{}", &dest_dir, file_name);

            if let Ok(_) = fs::metadata(&dest_file) {
                info!("File {} already exists", file_name);
            } else {
                download::download(&file_url, &dest_file)?;
            }

            Ok(dest_file)
        })
        .collect();

    let downloaded_files = maybe_downloaded_files?;
    Ok(downloaded_files)
}

pub fn get_callgraph_minute_value(time_breakdown: &TimeBreakdown) -> u32 {
    let mut minute_value: u32 = 0;
    minute_value += time_breakdown.day * 24 * 60;
    minute_value += time_breakdown.hour * 60;
    minute_value += time_breakdown.minute;
    minute_value
}
