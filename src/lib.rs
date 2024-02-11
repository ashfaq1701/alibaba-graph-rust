#[allow(dead_code)]
mod utils;

#[allow(dead_code)]
mod data;

#[allow(dead_code)]
mod graph;

#[allow(dead_code)]
mod process;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use process::init::run_op_and_return_results;
use crate::utils::env_params::load_env_files;

#[pyfunction]
pub fn run_op(py: Python, op_data: Option<&str>, windows_dir: Option<&str>) -> PyResult<Vec<(f64, u32, u32, u32, u32)>> {
    py.allow_threads(|| match run_op_and_return_results(op_data, windows_dir) {
        Ok(result) => {
            let result_tuples = result
                .iter()
                .map(|result| (
                    result.result,
                    result.window_idx as u32,
                    result.stored_window_number,
                    result.start,
                    result.end
                ))
                .collect();
            Ok(result_tuples)
        },
        Err(e) => Err(PyValueError::new_err(format!("{}", e)))
    })
}

#[pymodule]
fn alibaba_graph_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    load_env_files();

    m.add_function(wrap_pyfunction!(run_op, m)?)?;
    Ok(())
}
