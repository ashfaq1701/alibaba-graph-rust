use std::path::PathBuf;
use raphtory::prelude::{Graph, GraphViewOps, TimeOps};
use anyhow::Result;
use raphtory::db::api::view::internal::GraphOps;
use crate::utils::{get_window_count, get_windows};
use rayon::prelude::*;

pub fn window_graph_and_save(
    graph: &Graph,
    window_size: u32,
    overlap: u32,
    batch_idx: u32,
    batch_size: u32,
    end: u32
) -> Result<Vec<String>> {
    println!("{} Graph total vertices {}", batch_idx, graph.vertices().len());
    let window_files: Vec<String> = Vec::new();
    let windows = get_windows(batch_idx, batch_size, window_size, overlap, end);
    let batch_total_time = batch_size * 180;
    let window_per_batch = get_window_count(batch_total_time, window_size, overlap);

    let starting_window_idx = window_per_batch * batch_idx;

    windows
        .par_iter()
        .enumerate()
        .try_for_each(|(window_idx, (start, end))|
            create_and_save_window(graph, start, end, &(window_idx as u32), &starting_window_idx)
        )?;

    Ok(window_files)
}

pub fn create_and_save_window(
    graph: &Graph,
    start: &u32,
    end: &u32,
    window_idx: &u32,
    start_idx: &u32
) -> Result<()> {
    let total_idx = *start_idx + *window_idx;
    println!("Started loading window - {} ({}, {})", total_idx, start, end);

    let pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_name = format!("window_{}_{}_{}", total_idx, start, end);
    let dest_path = format!("{}/data/windows/{}", pathbuf.to_str().unwrap(), file_name);

    let graph_window = graph.window(
        (*start * 1000) as i64,
        (*end * 1000) as i64
    );
    let materialized_graph_window = graph_window.materialize().unwrap();

    match materialized_graph_window.save_to_file(&dest_path) {
        Ok(_) => {
            println!("Saved window - {} ({}, {}) at {}", total_idx, start, end, &dest_path);
        }
        Err(e) => {
            println!("Error while saving window - {} ({}, {}) - {}", total_idx, start, end, e);
        }
    };
    Ok(())
}