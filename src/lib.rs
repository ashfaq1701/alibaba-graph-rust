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
use crate::process::structs::WindowResult;
use crate::utils::env_params::load_env_files;

#[pyfunction]
pub fn run_op(op_data: Option<&str>) -> PyResult<Vec<WindowResult>> {
    match run_op_and_return_results(op_data) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyValueError::new_err(format!("{}", e)))
    }
}

#[pymodule]
fn alibaba_graph_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    load_env_files();

    m.add_function(wrap_pyfunction!(run_op, m)?)?;
    Ok(())
}
