use anyhow::Result;
use rayon::prelude::*;
use crate::process::structs::{WindowInfo, WindowResult};

fn run_operation_on_windows<T: BaseOp>(
    instance: &mut T,
    window_paths: &Vec<String>
) -> Result<Vec<WindowResult>> {
    let op_maybe_result_and_indices: Result<Vec<(WindowInfo, usize, f64)>> = window_paths
        .par_iter()
        .enumerate()
        .map(|(idx, window_path)| {
            instance.perform_op_on_single_window_and_idx(idx, window_path)
        })
        .collect();

    let mut op_result_and_indices = op_maybe_result_and_indices?;
    op_result_and_indices.sort_by(|a, b| a.1.cmp(&b.1));
    let result: Vec<WindowResult> = op_result_and_indices.iter().map(
        |(window_info, usize, result)|
            WindowResult {
                result: *result,
                window_idx: *usize,
                stored_window_number: window_info.stored_window_number,
                start: window_info.start,
                end: window_info.end,
            }
    ).collect();

    Ok(result)
}

fn get_window_info(window_file_name: &String) -> Result<WindowInfo> {
    let parts: Vec<&str> = window_file_name.split("_").collect();
    let stored_window_number = parts[1].parse::<u32>()?;
    let start = parts[2].parse::<u32>()?;
    let end = parts[3].parse::<u32>()?;
    Ok(WindowInfo { stored_window_number, start, end })
}

pub trait BaseOp: Sync {
    fn perform_op_on_windows(&mut self, window_paths: &Vec<String>) -> Result<Vec<WindowResult>>
        where Self: Sized {
        run_operation_on_windows(self, window_paths)
    }

    fn perform_op_on_single_window_and_idx(
        &self,
        window_idx: usize,
        window_file_path: &String
    ) -> Result<(WindowInfo, usize, f64)> {
        let result = self.perform_op_on_single_window(window_idx, window_file_path)?;
        let window_info = get_window_info(window_file_path)?;

        Ok((window_info, window_idx, result))
    }

    fn perform_op_on_single_window(
        &self,
        window_idx: usize,
        window_file_path: &String
    ) -> Result<f64>;
}
