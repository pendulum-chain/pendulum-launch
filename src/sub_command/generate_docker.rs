use crate::{
    error::Result,
    node::{AsCommand, Node},
    task::Task,
    util, Config,
};
use std::{collections::HashSet, fs};

pub struct GenerateDocker {
    name: String,
    config: Config,
    out_dir: String,
    enable_volume: bool,
}

impl GenerateDocker {
    pub fn new(config: Config, out_dir: String, enable_volume: bool) -> Self {
        let name = config.name.clone().unwrap_or("pendulum-launch".to_owned());

        Self {
            name,
            config,
            out_dir,
            enable_volume,
        }
    }

    pub fn execute(&self) -> Result<()> {
        if self.enable_volume {
            self.populate_volume()?;
        }

        let out_file = format!("{}/docker-compose.yml", self.out_dir);
        let contents = self.generate_contents()?;
        fs::write(out_file, contents)?;

        Ok(())
    }

    fn generate_contents(&self) -> Result<String> {
        let mut docker_compose = String::from(
            r#"version: "3.2"

services:"#,
        );

        self.write_service(&mut docker_compose, &self.config.validators)?;
        self.write_service(&mut docker_compose, &self.config.collators)?;

        Ok(docker_compose)
    }

    fn populate_volume(&self) -> Result<()> {
        // TODO: add crate::Error for non_root execution
        if !util::is_root() {
            panic!("You must have root permissions to enable a shared docker volume");
        };

        let container = self.name.as_str();

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
    fn get_unique_specs(&self) -> Result<Vec<String>> {
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

    fn write_service<N>(&self, docker_compose: &mut String, nodes: &[N]) -> Result<()>
    where
        N: Node + AsCommand,
    {
        for node in nodes {
            let service = self.generate_service(node)?;
            docker_compose.push_str(format!("\n{}", service).as_str());
        }

        Ok(())
    }

    fn generate_service<N>(&self, node: &N) -> Result<String>
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

        // Maps internal chain ports
        self.map_ports(node)
            .for_each(|port| service.push_str(&format!("\n      {}", port)));

        // Mounts shared volume to each node
        if self.enable_volume {
            service.push_str(format!(r#"\n    volumes:"#).as_str());
            service.push_str(format!(r#"\n      - {}:/specs"#, self.name.as_str()).as_str());
        }

        // Mounts shared volume if `--enable-volume` is passed
        service.push_str(&format!(
            "\n    restart: on-failure\n    command: {}",
            node.as_command_external(self.enable_volume)?
        ));

        Ok(service)
    }

    fn map_ports<N>(&self, node: &N) -> impl Iterator<Item = String>
    where
        N: Node,
    {
        node.ports()
            .into_iter()
            .flatten()
            .map(|port| format!("- \"{}:{}\"", port, port))
    }
}
