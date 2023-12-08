use raphtory::{prelude::*, };
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use csv::ReaderBuilder;
use raphtory::prelude::Graph;
use crate::data::structs::ConnectionProp;
use crate::utils;
use crate::graph::structs::{Trace};
use anyhow::Result;
use raphtory::core::ArcStr;
use rayon::prelude::*;
use crate::utils::{calculate_file_batch_size, create_windows};

pub fn load_event_files(
    file_paths: Vec<String>,
    window_size: u32,
    overlap: u32,
    connection_prop: &ConnectionProp
) -> Result<Vec<String>> {
    let loaded_graphs = Vec::new();
    let total_files = file_paths.len() as u32;
    let batch_size = calculate_file_batch_size(180, window_size, overlap, total_files);
    println!("Batch Size {}", batch_size);
    let windowed_paths = create_windows(file_paths, batch_size as usize);

    windowed_paths
        .par_iter()
        .enumerate()
        .try_for_each(move |(window_idx, file_window)| -> Result<()> {
            let current_loaded_graph = load_event_file_window(
                file_window,
                window_idx,
                batch_size,
                connection_prop
            )?;

            Ok(())
        }).expect("Failed loading window");

    Ok(loaded_graphs)
}

pub fn load_event_file_window(
    file_window: &Vec<String>,
    window_idx: usize,
    batch_size: u32,
    connection_prop: &ConnectionProp
) -> Result<Graph> {
    println!("Starting to load batch {} of size {}, files are {:?}", window_idx, file_window.len(), file_window);

    let graph = Graph::new();
    let graph_mutex = Mutex::new(&graph);

    file_window
        .par_iter()
        .try_for_each(|file_path|
            load_event_file(file_path, &graph_mutex, connection_prop)
        )?;

    Ok(graph)
}

pub fn load_event_file(
    file_path: &String,
    graph_mutex: &Mutex<&Graph>,
    connection_prop: &ConnectionProp) -> Result<()> {

    println!("Starting to load the file {}", file_path);

    println!("Extracting file {} in /tmp directory", file_path);
    utils::extract_gz_file(file_path, &"/tmp".to_string())
        .expect("Error in extracting file");

    let source_path = Path::new(file_path);
    let file_name = source_path.file_name().unwrap().to_str().unwrap();
    let file_parts: Vec<&str> = file_name.split(".").collect();
    let dst_file_path = format!("{}/{}.csv", "/tmp", file_parts[0]);

    populate_graph(&dst_file_path, graph_mutex, connection_prop)?;

    fs::remove_file(&dst_file_path)?;
    println!("Deleting file {}", dst_file_path);
    Ok(())
}

pub fn populate_graph(
    file_path: &String,
    graph_mutex: &Mutex<&Graph>,
    connection_prop: &ConnectionProp
) -> Result<()> {
    let file = File::open(file_path)?;

    let mut rdr = ReaderBuilder::new().from_reader(file);
    for result in rdr.deserialize::<Trace>() {
        match result {
            Ok(trace) => {
                let (src, dst, props) = match connection_prop {
                    ConnectionProp::MicroserviceId => {
                        if trace.dm.is_empty()
                            || trace.dm == "UNKNOWN"
                            || trace.um.is_empty()
                            || trace.um == "UNKNOWN" {
                            continue
                        }

                        let props = [
                            ("uminstanceid".to_owned(), Prop::Str(ArcStr::from(trace.uminstanceid))),
                            ("dminstanceid".to_owned(), Prop::Str(ArcStr::from(trace.dminstanceid))),
                            ("rt".to_owned(), Prop::F32(trace.rt))
                        ];

                        (trace.um, trace.dm, props)
                    }
                    ConnectionProp::InstanceId => {
                        if trace.dminstanceid.is_empty()
                            || trace.dminstanceid == "UNKNOWN"
                            || trace.uminstanceid.is_empty()
                            || trace.uminstanceid == "UNKNOWN" {
                            continue
                        }

                        let props = [
                            ("um".to_owned(), Prop::Str(ArcStr::from(trace.um))),
                            ("dm".to_owned(), Prop::Str(ArcStr::from(trace.dm))),
                            ("rt".to_owned(), Prop::F32(trace.rt))
                        ];

                        (trace.uminstanceid, trace.dminstanceid, props)
                    }
                };

                let graph = graph_mutex.lock().unwrap();
                graph.add_edge(
                    trace.timestamp,
                    src,
                    dst,
                    props,
                    None
                )?;
            }
            Err(_) => {
                continue
            }
        }
    }

    println!("Loaded file {}", file_path);
    Ok(())
}
