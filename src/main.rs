use lib_pendulum_launch::{
    error::Result,
    node::{Collator, CollatorRelay, Node, Validator},
    Config, Launcher,
};

fn init() -> Launcher {
    let validator_bin = "./bin/polkadot";
    let validator_chain = "./specs/rococo-custom-2-raw.json";

    let collator_bin = "./bin/pendulum-collator";
    let collator_chain = "./specs/rococo-local-parachain-raw.json";

    let validator = {
        let name = Some("validator_node");
        let args = vec![];
        let port = 30343;
        let ws_port = 9944;
        let rpc_port = None;

        let node = Node::new(
            name,
            validator_bin,
            validator_chain,
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

        let relay = CollatorRelay::new(validator_chain, 30345, 9955, None);
        let inner = Node::new(
            name,
            collator_bin,
            collator_chain,
            args,
            port,
            ws_port,
            rpc_port,
        );

        Collator::new(inner, relay)
    };

    let config = Config::new(
        Some("Pendulum"),
        Some("xiuxiu"),
        vec![validator],
        vec![collator],
    );

    // Generate toml config
    println!("{}", toml::to_string_pretty(&config).unwrap());
    // Generate json config
    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    Launcher::new(config)
}

fn main() -> Result<()> {
    let mut launcher = init();
    launcher.run()
}
