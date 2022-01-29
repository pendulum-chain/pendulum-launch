use lib_pendulum_launch::{Collator, CollatorRelay, Node, Validator};
use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    let validator_bin: PathBuf = PathBuf::from("./bin/polkadot");
    let validator_chain: PathBuf = PathBuf::from("./specs/rococo-custom-2-raw.json");

    let collator_bin: PathBuf = PathBuf::from("./bin/pendulum-collator");
    let collator_chain: PathBuf = PathBuf::from("./specs/rococo-local-parachain-raw.json");

    let validator = {
        let name = Some("validator_node");
        let args = vec![];
        let port = 30343;
        let ws_port = 9944;
        let rpc_port = None;

        let node = Node::new(
            name,
            &validator_bin,
            &validator_chain,
            args,
            port,
            ws_port,
            rpc_port,
        );

        Validator::new(node)
    };

    let collator = {
        let name = Some("collator_node");
        let args = vec!["--force-authoring"];
        let port = 30344;
        let ws_port = 8844;
        let rpc_port = None;

        let relay = CollatorRelay::new(&validator_chain, 30345, 9955, None);
        let inner = Node::new(
            name,
            &collator_bin,
            &collator_chain,
            args,
            port,
            ws_port,
            rpc_port,
        );

        Collator::new(inner, relay)
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let mut validator_handle = validator.run()?;
    let mut collator_handle = collator.run()?;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        validator_handle.kill().expect("Failed to kill validator");
        collator_handle.kill().expect("Failed to kill collator");
    })?;

    while running.load(Ordering::SeqCst) {}

    Ok(())
}
