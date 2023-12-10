use serde::{Deserialize};

#[derive(Deserialize, std::fmt::Debug)]
pub struct Trace {
    pub timestamp: i64,
    _traceid: String,
    _service: String,
    _rpc_id: String,
    _rpctype: String,
    pub um: String,
    pub uminstanceid: String,
    _interface: String,
    pub dm: String,
    pub dminstanceid: String,
    pub rt: f32
}
