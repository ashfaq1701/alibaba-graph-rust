use std::collections::HashMap;
use std::sync::Arc;
use crate::process::structs::OpType;
use anyhow::{anyhow, Result};
use crate::process::op_mapper::get_op_executor;
use crate::utils::{get_files_in_directory, get_resolved_windows_dir};
use crate::process::ops::base_op::BaseOp;

pub fn run_process_data(options: &HashMap<&str, &str>) -> Result<Vec<f64>> {
    let op = match options.get("op") {
        Some(&"average_degree") => OpType::AverageDegree,
        _ => return Err(anyhow!("Error parsing the op type."))
    };

    run_op_and_return_results(op)
}

pub fn run_op_and_return_results(op: OpType) -> Result<Vec<f64>> {
    let window_file_paths = get_window_file_paths()?;
    let mut op_executor = get_op_executor(op, &window_file_paths);

    op_executor.perform_op_on_windows(&window_file_paths)
}

pub fn get_window_file_paths() -> Result<Vec<String>> {
    let windows_dir = get_resolved_windows_dir();
    let windows_dir_arc = Arc::new(&windows_dir);
    let files_in_windows_dir = get_files_in_directory(&windows_dir)?;

    let mut window_files: Vec<&String> = files_in_windows_dir
        .iter()
        .filter(|file_name| (*file_name).starts_with("window_"))
        .collect();

    window_files.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split("_").collect();
        let a_i = match a_parts[1].to_string().parse::<u32>() {
            Ok(i) => i,
            Err(_) => u32::MAX
        };

        let b_parts: Vec<&str> = b.split("_").collect();
        let b_i = match b_parts[1].to_string().parse::<u32>() {
            Ok(i) => i,
            Err(_) => u32::MAX
        };

        a_i.cmp(&b_i)
    });

    let window_file_paths: Vec<String> = window_files
        .iter()
        .map(|file_name| {
            let window_dir_clone = Arc::clone(&windows_dir_arc);
            format!("{}/{}", window_dir_clone, file_name)
        })
        .collect();

    Ok(window_file_paths)
}
