use anyhow::Result;
use rayon::prelude::*;

fn run_operation_on_windows<T: BaseOp>(
    instance: &mut T,
    window_paths: &Vec<String>
) -> Result<Vec<f64>> {
    let op_maybe_result_and_indices: Result<Vec<(usize, f64)>> = window_paths
        .par_iter()
        .enumerate()
        .map(|(idx, window_path)| {
            instance.perform_op_on_single_window_and_idx(idx, window_path)
        })
        .collect();

    let mut op_result_and_indices = op_maybe_result_and_indices?;
    op_result_and_indices.sort_by(|a, b| a.0.cmp(&b.0));
    let result: Vec<f64> = op_result_and_indices.iter().map(|a_pair|
        a_pair.1
    ).collect();

    Ok(result)
}

pub trait BaseOp: Sync {
    fn perform_op_on_windows(&mut self, window_paths: &Vec<String>) -> Result<Vec<f64>>
        where Self: Sized {
        run_operation_on_windows(self, window_paths)
    }

    fn perform_op_on_single_window_and_idx(
        &self,
        window_idx: usize,
        window_file_path: &String
    ) -> Result<(usize, f64)> {
        let result = self.perform_op_on_single_window(window_idx, window_file_path)?;
        Ok((window_idx, result))
    }

    fn perform_op_on_single_window(
        &self,
        window_idx: usize,
        window_file_path: &String
    ) -> Result<f64>;
}
