use serde::Deserialize;

#[derive(Deserialize, std::fmt::Debug)]
pub struct TraceRaw {
    timestamp: String,
    traceid: String,
    service: String,
    rpc_id: String,
    rpctype: String,
    um: String,
    uminstanceid: String,
    interface: String,
    dm: String,
    dminstanceid: String,
    rt: String
}

#[derive(std::fmt::Debug)]
pub struct Trace {
    timestamp: u32,
    um: String,
    uminstanceid: Option<String>,
    dm: String,
    dminstanceid: Option<String>,
    rt: String
}
