use std::path::PathBuf;
use raphtory::prelude::{GraphViewOps, TimeOps};
use anyhow::Result;
use raphtory::db::graph::views::deletion_graph::GraphWithDeletions;
use rayon::prelude::*;

pub fn window_graph_and_save(
    graph: &GraphWithDeletions,
    windows: &Vec<(u32, u32)>,
    file_end: u32,
    start_idx: u32,
    beginning_ptr_for_next: u32
) -> Result<(Vec<String>, GraphWithDeletions)> {

    let maybe_loaded_window_files: Result<Vec<String>> = windows
        .par_iter()
        .enumerate()
        .map(|(idx, (start, end))| {
            let window_idx = start_idx + (idx as u32);
            create_and_save_window(
                graph,
                start,
                end,
                &window_idx
            )
        })
        .collect();

    let loaded_window_files=  maybe_loaded_window_files?;

    let next_graph = graph.window(
        (beginning_ptr_for_next * 1000) as i64,
        (file_end * 1000) as i64
    )
        .materialize()
        .unwrap()
        .into_persistent()
        .unwrap();

    Ok((loaded_window_files, next_graph))
}

pub fn create_and_save_window(
    graph: &GraphWithDeletions,
    start: &u32,
    end: &u32,
    window_idx: &u32
) -> Result<String> {
    println!("Started loading window - {} ({}, {})", window_idx, start, end);

    let pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_name = format!("window_{}_{}_{}", window_idx, start, end);
    let dest_path = format!("{}/data/windows/{}", pathbuf.to_str().unwrap(), file_name);

    let graph_window = graph.window(
        (*start * 1000) as i64,
        (*end * 1000) as i64
    );
    let materialized_graph_window = graph_window.materialize().unwrap();

    match materialized_graph_window.save_to_file(&dest_path) {
        Ok(_) => {
            println!("Saved window - {} ({}, {}) at {}", window_idx, start, end, &dest_path);
        }
        Err(e) => {
            println!("Error while saving window - {} ({}, {}) - {}", window_idx, start, end, e);
        }
    };
    Ok(dest_path)
}
