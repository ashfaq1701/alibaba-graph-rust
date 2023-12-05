use crate::utils;

pub fn load_event_files(file_paths: &Vec<String>) -> Vec<String> {
    let mut loaded_graphs = Vec::new();

    for file_path in file_paths.iter() {
        let current_loaded_graphs = load_event_file(file_path);
        loaded_graphs.extend(current_loaded_graphs);
    }

    loaded_graphs
}

pub fn load_event_file(file_path: &String) -> Vec<String> {
    utils::extract_gz_file(file_path, &"/Users/ashfaq/Documents/extracts".to_string())
        .expect("Could not extract event file");
    Vec::new()
}
