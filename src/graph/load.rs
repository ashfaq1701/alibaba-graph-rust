use std::cmp::min;
use raphtory::{prelude::*, };
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use csv::ReaderBuilder;
use crate::data::structs::ConnectionProp;
use crate::utils;
use crate::graph::structs::{Trace};
use anyhow::Result;
use raphtory::core::ArcStr;
use crate::graph::save::window_graph_and_save;
use crate::utils::{get_file_bounds, get_window_count, get_windows};
use log::{info};
use rayon::prelude::*;

pub fn load_event_files(
    file_paths: Vec<String>,
    window_size: u32,
    connection_prop: &ConnectionProp,
    start: u32,
    end: u32
) -> Result<Vec<String>> {
    let file_bounds = get_file_bounds(start, end);

    let files_count = file_paths.len();
    let first_file_window_count = get_window_count(
        start,
        (start / 180) * 180 + 180,
        window_size
    );

    let window_count = get_window_count(
        0,
        180,
        window_size
    );

    let maybe_window_files: Result<Vec<Vec<String>>> = file_paths
        .par_iter()
        .enumerate()
        .map(|(idx, file_path)| {
            let file_bound = file_bounds[idx];
            let (file_start, file_end) = file_bound;

            let current_file_start = if idx == 0 {
                min(file_start, start)
            } else {
                file_start
            };

            let current_file_end = if idx == files_count - 1 {
                min(end, file_end)
            } else {
                file_end
            };

            let starting_idx = if idx == 0 {
                0
            } else {
                first_file_window_count + (idx - 1) as u32 * window_count
            };

            init_load_event_file(
                file_path,
                connection_prop,
                current_file_start,
                current_file_end,
                starting_idx,
                window_size
            )
        }).collect();

    let window_files = maybe_window_files.unwrap();

    Ok(window_files.concat())
}

fn init_load_event_file(
    file_path: &String,
    connection_prop: &ConnectionProp,
    start: u32,
    end: u32,
    starting_idx: u32,
    window_size: u32
) -> Result<Vec<String>> {
    let windows = get_windows(
        start,
        end,
        window_size
    );

    let graph = Graph::new();
    let graph_mutex = Mutex::new(&graph);
    load_event_file(file_path, &graph_mutex, connection_prop)?;

    let window_files = window_graph_and_save(
        &graph,
        &windows,
        starting_idx
    )?;

    Ok(window_files)
}

pub fn load_event_file(
    file_path: &String,
    graph_mutex: &Mutex<&Graph>,
    connection_prop: &ConnectionProp) -> Result<()> {

    info!("Extracting file {} in /tmp directory", file_path);
    utils::extract_gz_file(file_path, &"/tmp".to_string())
        .expect("Error in extracting file");

    let source_path = Path::new(file_path);
    let file_name = source_path.file_name().unwrap().to_str().unwrap();
    let file_parts: Vec<&str> = file_name.split(".").collect();
    let dst_file_path = format!("{}/{}.csv", "/tmp", file_parts[0]);

    match populate_graph(&dst_file_path, graph_mutex, connection_prop) {
        Ok(_) => {
            info!("Loaded file {}", dst_file_path);
        }
        Err(err) => {
            info!("Error happened while populating graph - {}", err);
        }
    };

    info!("Deleting file {}", dst_file_path);
    fs::remove_file(&dst_file_path)?;

    Ok(())
}

pub fn populate_graph(
    file_path: &String,
    graph_mutex: &Mutex<&Graph>,
    connection_prop: &ConnectionProp
) -> Result<()> {
    info!("Starting to load the file {}", file_path);
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
