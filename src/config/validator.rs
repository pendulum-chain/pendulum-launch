use crate::{
    node::{BaseNode, Validator},
    PathBuffer,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidatorConfig {
    bin: PathBuffer,
    dockerfile: Option<PathBuffer>,
    nodes: Vec<ValidatorNodeConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ValidatorNodeConfig {
    name: String,
    chain: PathBuffer,
    args: Vec<String>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl ValidatorNodeConfig {
    fn base_node(&self, bin: &PathBuffer, dockerfile: &Option<PathBuffer>) -> BaseNode {
        BaseNode::new(
            self.name.to_owned(),
            bin.clone(),
            self.chain.to_owned(),
            dockerfile.clone(),
            self.args.to_owned(),
            self.port.to_owned(),
            self.ws_port.to_owned(),
            self.rpc_port.to_owned(),
        )
    }
}

impl Into<Vec<Validator>> for ValidatorConfig {
    fn into(self) -> Vec<Validator> {
        self.nodes
            .into_iter()
            .map(|validator_config| {
                Validator::new(validator_config.base_node(&self.bin, &self.dockerfile))
            })
            .collect()
    }
}
