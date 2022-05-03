use crate::PathBuffer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CollatorConfig {
    bin: PathBuffer,
    nodes: Vec<CollatorNodeConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CollatorNodeConfig {
    name: String,
    chain: PathBuffer,
    dockerfile: Option<PathBuffer>,
    args: Vec<String>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}
