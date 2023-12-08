use serde::{Deserialize, Deserializer};

#[derive(Deserialize, std::fmt::Debug)]
pub struct Trace {
    pub timestamp: i64,
    traceid: String,
    service: String,
    rpc_id: String,
    rpctype: String,
    pub um: String,
    pub uminstanceid: String,
    interface: String,
    pub dm: String,
    pub dminstanceid: String,
    pub rt: f32
}