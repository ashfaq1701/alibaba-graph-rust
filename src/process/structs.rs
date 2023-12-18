use pyo3::pyclass;

pub enum OpType {
    AverageDegree
}

impl OpType {
    pub fn from_str(s: &str) -> Option<OpType> {
        match s {
            "average_degree" => Some(OpType::AverageDegree),
            _ => None,
        }
    }
}

pub struct WindowInfo {
    pub stored_window_number: u32,
    pub start: u32,
    pub end: u32
}

#[pyclass]
#[derive(std::fmt::Debug)]
pub struct WindowResult {
    pub result: f64,
    pub window_idx: usize,
    pub stored_window_number: u32,
    pub start: u32,
    pub end: u32
}
