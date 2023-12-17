use crate::process::ops::base_op::BaseOp;
use anyhow::Result;
use log::info;
use raphtory::{prelude::*, };
use raphtory::algorithms::metrics::degree::average_degree;

pub struct AverageDegree<'a> {
    pub window_file_paths: &'a Vec<String>
}

impl<'a> BaseOp for AverageDegree<'a> {
    fn perform_op_on_single_window(
        &self,
        _window_idx: usize,
        window_file_path: &String
    ) -> Result<f64> {
        info!("Started loading {}", window_file_path);
        let graph = Graph::load_from_file(window_file_path)?;
        Ok(average_degree(&graph))
    }
}
