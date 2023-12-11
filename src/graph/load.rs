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
use raphtory::db::graph::views::deletion_graph::GraphWithDeletions;
use crate::graph::save::window_graph_and_save;
use crate::utils::{get_file_bounds, get_windows_and_next};

pub fn load_event_files(
    file_paths: Vec<String>,
    window_size: u32,
    overlap: u32,
    connection_prop: &ConnectionProp,
    start: u32,
    end: u32
) -> Result<Vec<String>> {
    let file_bounds = get_file_bounds(start, end);
    let mut running_start = start;
    let mut running_window_idx = 0;
    let mut running_graph = GraphWithDeletions::new();

    let maybe_window_files: Result<Vec<Vec<String>>> = file_paths
        .iter()
        .enumerate()
        .map(|(idx, file_path)|
            init_load_event_file(
                idx,
                file_path,
                &file_bounds,
                connection_prop,
                end,
                &mut running_start,
                &mut running_window_idx,
                window_size,
                overlap,
                &mut running_graph
            )
        ).collect();

    let window_files = maybe_window_files.unwrap();

    Ok(window_files.concat())
}

fn init_load_event_file(
    file_idx: usize,
    file_path: &String,
    file_bounds: &Vec<(u32, u32)>,
    connection_prop: &ConnectionProp,
    end: u32,
    running_start: &mut u32,
    running_window_idx: &mut u32,
    window_size: u32,
    overlap: u32,
    graph: &mut GraphWithDeletions
) -> Result<Vec<String>> {
    println!("Graph number of vertices before {}", graph.count_vertices());
    let (file_start, file_end) = file_bounds[file_idx];
    let current_end = min(file_end, end);
    let result = get_windows_and_next(
        *running_start,
        current_end,
        window_size,
        overlap
    );
    let windows = result.0;
    *running_start = result.1;

    let graph_mutex = Mutex::new(&*graph);
    load_event_file(file_path, &graph_mutex, connection_prop)?;

    let window_result = window_graph_and_save(
        &*graph,
        &windows,
        file_start,
        file_end,
        *running_window_idx)?;

    *running_window_idx = *running_window_idx + (windows.len() as u32);
    let window_files = window_result.0;
    *graph = window_result.1;

    Ok(window_files)
}

pub fn load_event_file(
    file_path: &String,
    graph_mutex: &Mutex<&GraphWithDeletions>,
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
    graph_mutex: &Mutex<&GraphWithDeletions>,
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
