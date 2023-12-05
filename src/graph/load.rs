use raphtory::{
    graph_loader::source::csv_loader::CsvLoader, prelude::*,
};
use std::fs;
use std::path::Path;
use raphtory::prelude::Graph;
use crate::utils;
use crate::graph::structs::TraceRaw;
use crate::utils::{calculate_file_batch_size, create_windows};

pub fn load_event_files(file_paths: Vec<String>, window_size: u32, overlap: u32) -> Vec<String> {
    let mut loaded_graphs = Vec::new();
    let batch_size = calculate_file_batch_size(180, window_size, overlap);
    let windowed_paths = create_windows(file_paths, batch_size as usize);

    for file_window in windowed_paths.iter() {
        let current_loaded_graph = load_event_file_window(file_window);
    }

    loaded_graphs
}

pub fn load_event_file_window(file_window: &Vec<String>) -> Graph {
    let graph = Graph::new();

    for file_path in file_window {
        utils::extract_gz_file(file_path, &"/tmp".to_string())
            .expect("Could not extract event file");

        let source_path = Path::new(file_path);
        let file_name = source_path.file_name().unwrap().to_str().unwrap();
        let file_parts: Vec<&str> = file_name.split(".").collect();
        let dst_file_path = format!("{}/{}.csv", "/tmp", file_parts[0]);

        populate_graph(&dst_file_path, &graph);

        fs::remove_file(dst_file_path).expect("Error in deleting file");
    }

    graph
}

pub fn populate_graph(file_path: &String, graph: &Graph) {
    println!("{}", file_path);
    CsvLoader::new(file_path)
        .set_header(true)
        .load_into_graph(graph, |trace_raw: TraceRaw, g: &Graph| {
            println!("{:?}", trace_raw);
        })
        .expect("Failed to load graph from CSV data files")
}
