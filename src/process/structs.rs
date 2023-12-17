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
