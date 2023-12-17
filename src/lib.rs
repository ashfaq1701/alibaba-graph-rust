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

#[pyfunction]
pub fn run_op(op_data: Option<&str>) -> PyResult<Vec<f64>> {
    match run_op_and_return_results(op_data) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyValueError::new_err(format!("{}", e)))
    }
}

#[pymodule]
fn alibaba_graph_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_op, m)?)?;
    Ok(())
}
