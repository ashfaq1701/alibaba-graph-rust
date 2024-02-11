use log::info;
use raphtory::prelude::{Graph, GraphViewOps};
use crate::process::ops::base_op::BaseOp;

pub struct NumberOfVertices<'a> {
    pub window_file_paths: &'a Vec<String>
}

impl<'a> BaseOp for NumberOfVertices<'a> {
    fn perform_op_on_single_window(
        &self,
        _window_idx: usize,
        window_file_path: &String
    ) -> anyhow::Result<f64> {
        info!("Started loading {}", window_file_path);
        let graph = Graph::load_from_file(window_file_path)?;
        Ok(graph.nodes().len() as f64)
    }
}