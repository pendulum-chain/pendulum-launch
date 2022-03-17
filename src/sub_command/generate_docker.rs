use crate::{
    error::Result,
    node::{AsCommand, Node},
    task::Task,
    Config,
};
use nix::unistd::Uid;
use std::{collections::HashSet, fs};

pub struct GenerateDocker {
    config: Config,
    out_dir: String,
    enable_volume: bool,
}

impl GenerateDocker {
    pub fn new(config: Config, out_dir: String, enable_volume: bool) -> Self {
        Self {
            config,
            out_dir,
            enable_volume,
        }
    }

    pub fn execute(self) -> Result<()> {
        if self.enable_volume && !Uid::effective().is_root() {
            panic!("You must have root permissions to enable a shared docker volume");
        }

        let out_file = format!("{}/docker-compose.yml", self.out_dir);
        let contents = self.generate_contents()?;
        fs::write(out_file, contents)?;

        Ok(())
    }

    fn generate_contents(self) -> Result<String> {
        let mut docker_compose = String::from(
            r#"version: "3.2"

services:"#,
        );

        write_service(&mut docker_compose, &self.config.validators)?;
        write_service(&mut docker_compose, &self.config.collators)?;

        Ok(docker_compose)
    }

    fn populate_volume(self) -> Result<()> {
        let container = "temp";

        // Create volume if it doesn't exist
        Task::from(format!("sudo docker volume create {}", container)).execute()?;

        // Run an intermediary container with the mounted volume
        Task::from(format!(
            "sudo docker run -it -v {}:/specs --name {} ubuntu",
            container, container
        ))
        .execute()?;

        // Copy the specs to the mounted volume in the container
        for spec in self.get_unique_specs()? {
            Task::from(format!(
                "sudo docker cp {} {}:/specs/{}",
                spec, container, spec
            ))
            .execute()?;
        }

        // Stop the intermediary container
        Task::from(format!("sudo docker stop {}", container)).execute()?;

        Ok(())
    }

    // Returns a list of unique chain-spec raw paths
    fn get_unique_specs(self) -> Result<Vec<String>> {
        let mut specs: HashSet<String> = HashSet::new();

        fn insert_specs(specs: &mut HashSet<String>, node: &impl Node) -> Result<()> {
            for spec in node.specs()? {
                specs.insert(spec);
            }

            Ok(())
        }

        self.config
            .validators
            .iter()
            .try_for_each(|v| insert_specs(&mut specs, v))?;

        // let insert_collator_specs = |c| insert_specs(&mut specs, c);
        self.config
            .collators
            .iter()
            .try_for_each(|c| insert_specs(&mut specs, c))?;

        Ok(Vec::from_iter(specs))
    }
}

fn write_service<N>(docker_compose: &mut String, nodes: &[N]) -> Result<()>
where
    N: Node + AsCommand,
{
    nodes
        .iter()
        .map(generate_service)
        .collect::<Result<Vec<String>>>()?
        .into_iter()
        .for_each(|service| docker_compose.push_str(&format!("\n{}", service)));

    Ok(())
}

fn generate_service<N>(node: &N) -> Result<String>
where
    N: Node + AsCommand,
{
    let name = node.name();

    let mut service = format!(
        r#"  {}:
    container_name: {} 
    image: pendulumchain/pendulum:latest
    build:
      context: .
      dockerfile: {}
    ports:"#,
        name,
        name,
        node.docker_file()?
    );

    map_ports(node).for_each(|port| service.push_str(&format!("\n      {}", port)));

    service.push_str(&format!(
        "\n    restart: on-failure\n    command: {}",
        node.as_command_external()?
    ));

    Ok(service)
}

fn map_ports<N>(node: &N) -> impl Iterator<Item = String>
where
    N: Node,
{
    node.ports()
        .into_iter()
        .flatten()
        .map(|port| format!("- \"{}:{}\"", port, port))
}
