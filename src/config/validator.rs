use crate::{
    node::{BaseNode, Validator},
    PathBuffer,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidatorConfig {
    bin: PathBuffer,
    nodes: Vec<ValidatorNodeConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ValidatorNodeConfig {
    name: String,
    chain: PathBuffer,
    dockerfile: Option<PathBuffer>,
    args: Vec<String>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

// BaseNode::new parameters
//
// name: Option<&str>,
// bin: &str,
// chain: &str,
// dockerfile: Option<&str>,
// args: Vec<&str>,
// port: u16,
// ws_port: u16,
// rpc_port: Option<u16>,

impl ValidatorNodeConfig {
    fn base_node(bin: PathBuffer) -> BaseNode {
        todo!()
    }
}

impl Into<Vec<Validator>> for ValidatorConfig {
    fn into(self) -> Vec<Validator> {
        todo!()
    }
}
