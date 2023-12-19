use crate::process::ops::average_degree::{AverageDegree};
use crate::process::structs::OpType;
use crate::process::ops::base_op::BaseOp;
use crate::process::ops::number_of_vertices::NumberOfVertices;

pub fn get_op_executor(op_type: OpType, window_file_paths: &Vec<String>) -> Box<dyn BaseOp + '_> {
    match op_type {
        OpType::AverageDegree => Box::new(AverageDegree { window_file_paths }),
        OpType::NumberOfVertices => Box::new(NumberOfVertices { window_file_paths })
    }
}
