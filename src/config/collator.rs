use crate::{
    node::{BaseNode, Collator, CollatorRelay},
    PathBuffer,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CollatorConfig {
    bin: PathBuffer,
    dockerfile: Option<PathBuffer>,
    nodes: Vec<CollatorNodeConfig>,
}

// impl CollatorConfig {
//     fn collator(&self, node: CollatorNodeConfig) -> Collator {
//         let base_node = node.base_node(&self.bin, &self.dockerfile);
//         let relay = node.relay();

//         Collator::new(base_node, relay)
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
struct CollatorNodeConfig {
    name: String,
    chain: PathBuffer,
    args: Vec<String>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
    relay: CollatorNodeRelayConfig,
}

impl CollatorNodeConfig {
    fn base_node(&self, bin: &PathBuffer, dockerfile: &Option<PathBuffer>) -> BaseNode {
        BaseNode::new(
            self.name.to_owned(),
            bin.clone(),
            self.chain.clone(),
            dockerfile.clone(),
            self.args.to_owned(),
            self.port.to_owned(),
            self.ws_port.to_owned(),
            self.rpc_port.to_owned(),
        )
    }

    fn relay(&self) -> CollatorRelay {
        CollatorRelay::new(
            self.relay.chain.to_owned(),
            self.relay.args.to_owned(),
            self.relay.port,
            self.relay.ws_port,
            self.relay.rpc_port,
        )
    }

    fn collator(&self, bin: &PathBuffer, dockerfile: &Option<PathBuffer>) -> Collator {
        Collator::new(self.base_node(bin, dockerfile), self.relay())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollatorNodeRelayConfig {
    chain: PathBuffer,
    args: Option<Vec<String>>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

// impl CollatorNodeRelayConfig {
//     fn relay(&self) -> CollatorRelay {
//         CollatorRelay::new(
//             self.chain.to_owned(),
//             self.args.to_owned(),
//             self.port.to_owned(),
//             self.ws_port.to_owned(),
//             self.rpc_port.to_owned(),
//         )
//     }
// }

impl Into<Vec<Collator>> for CollatorConfig {
    fn into(self) -> Vec<Collator> {
        self.nodes
            .into_iter()
            .map(|validator_config| validator_config.collator(&self.bin, &self.dockerfile))
            .collect()
    }
}
