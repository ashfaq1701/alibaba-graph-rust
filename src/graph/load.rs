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
use crate::graph::save::window_graph_and_save;
use crate::utils::{calculate_file_batch_size, create_windows, get_closest_file_start};

pub fn load_event_files(
    file_paths: Vec<String>,
    window_size: u32,
    overlap: u32,
    connection_prop: &ConnectionProp,
    start: u32,
    end: u32
) -> Result<Vec<String>> {
    let total_files = file_paths.len() as u32;
    let batch_count_files = calculate_file_batch_size(180, window_size, overlap, total_files);
    let windowed_paths = create_windows(file_paths, batch_count_files as usize);

    let maybe_loaded_files: Result<Vec<Vec<String>>> = windowed_paths
        .par_iter()
        .enumerate()
        .map(move |(batch_idx, file_batch)| {
            let file_start = get_closest_file_start(start);
            load_event_file_window(
                file_batch,
                batch_idx as u32,
                batch_count_files,
                connection_prop,
                window_size,
                overlap,
                file_start,
                start,
                end
            )
        })
        .collect();

    let loaded_files = maybe_loaded_files?;

    Ok(loaded_files.concat())
}

pub fn load_event_file_window(
    file_batch: &Vec<String>,
    batch_idx: u32,
    batch_count_files: u32,
    connection_prop: &ConnectionProp,
    window_size: u32,
    overlap: u32,
    file_start: u32,
    start: u32,
    end: u32
) -> Result<Vec<String>> {
    println!("Starting to load batch {} of size {}, files are {:?}", batch_idx, file_batch.len(), file_batch);

    let graph = Graph::new();
    let graph_mutex = Mutex::new(&graph);

    file_batch
        .par_iter()
        .try_for_each(|file_path|
            load_event_file(file_path, &graph_mutex, connection_prop)
        )?;

    let current_window_files = window_graph_and_save(
        &graph,
        window_size,
        overlap,
        batch_idx,
        batch_count_files,
        file_start,
        start,
        end
    )?;

    Ok(current_window_files)
}

pub fn load_event_file(
    file_path: &String,
    graph_mutex: &Mutex<&Graph>,
    connection_prop: &ConnectionProp) -> Result<()> {

    println!("Extracting file {} in /tmp directory", file_path);
    utils::extract_gz_file(file_path, &"/tmp".to_string())
        .expect("Error in extracting file");

    let source_path = Path::new(file_path);
    let file_name = source_path.file_name().unwrap().to_str().unwrap();
    let file_parts: Vec<&str> = file_name.split(".").collect();
    let dst_file_path = format!("{}/{}.csv", "/tmp", file_parts[0]);

    match populate_graph(&dst_file_path, graph_mutex, connection_prop) {
        Ok(_) => {
            println!("Loaded file {}", dst_file_path);
        }
        Err(err) => {
            println!("Error happened while populating graph - {}", err);
        }
    };

    println!("Deleting file {}", dst_file_path);
    fs::remove_file(&dst_file_path)?;

    Ok(())
}

pub fn populate_graph(
    file_path: &String,
    graph_mutex: &Mutex<&Graph>,
    connection_prop: &ConnectionProp
) -> Result<()> {
    println!("Starting to load the file {}", file_path);
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

    Ok(())
}
