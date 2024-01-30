use std::env;
use dotenv::from_filename;

pub fn load_env_files() {
    from_filename(".env").ok();
    from_filename(".env.local").ok();
}

pub fn get_file_duration_in_seconds() -> u32 {
    let file_len_in_seconds_str = env::var("FILE_DURATION_IN_SECONDS")
        .expect("FILE_DURATION_IN_SECONDS environment variable is required");
    let file_len_in_seconds = file_len_in_seconds_str.parse::<u32>()
        .expect("FILE_DURATION_IN_SECONDS should be integer");
    file_len_in_seconds
}

pub fn get_raw_trace_directory() -> Option<String> {
    match env::var("RAW_TRACE_DIR") {
        Ok(raw_trace_dir) => Some(raw_trace_dir),
        _ => None
    }
}

pub fn get_windows_directory() -> Option<String> {
    match env::var("WINDOWS_DIR") {
        Ok(windows_dir) => Some(windows_dir),
        _ => None
    }
}

pub fn get_tmp_directory() -> Option<String> {
    match env::var("TMP_DIR") {
        Ok(windows_dir) => Some(windows_dir),
        _ => None
    }
}
