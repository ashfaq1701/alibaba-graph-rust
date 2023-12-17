use crate::process::ops::average_degree::{AverageDegree};
use crate::process::structs::OpType;
use crate::process::ops::base_op::BaseOp;

pub fn get_op_executor(op_type: OpType, window_file_paths: &Vec<String>) -> impl BaseOp + '_ {
    match op_type {
        OpType::AverageDegree => AverageDegree { window_file_paths }
    }
}
