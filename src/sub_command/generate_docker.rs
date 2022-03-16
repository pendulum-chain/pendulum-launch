use crate::{
    error::Result,
    node::{AsCommand, Collator, Node, Validator},
    Config,
};
use std::{collections::HashSet, fs};

pub fn generate_docker(config: Config, out_dir: String) -> Result<()> {
    let out_file = format!("{}/docker-compose.yml", out_dir);
    let contents = generate_contents(&config)?;
    fs::write(out_file, contents)?;

    Ok(())
}

fn generate_contents(config: &Config) -> Result<String> {
    let mut docker_compose = String::from(
        r#"version: "3.2"

services:"#,
    );

    write_service(&mut docker_compose, &config.validators)?;
    write_service(&mut docker_compose, &config.collators)?;

    Ok(docker_compose)
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

// Returns a list of unique chain-spec raw paths
//
// TODO: remove Unusued anotation
#[allow(unused)]
fn get_unique_specs(validators: Vec<&Validator>, collators: Vec<&Collator>) -> Result<Vec<String>> {
    let mut specs: HashSet<String> = HashSet::new();

    fn insert_specs(specs: &mut HashSet<String>, node: &impl Node) -> Result<()> {
        for spec in node.specs()? {
            specs.insert(spec);
        }

        Ok(())
    }

    validators
        .into_iter()
        .try_for_each(|v| insert_specs(&mut specs, v))?;

    // let insert_collator_specs = |c| insert_specs(&mut specs, c);
    collators
        .into_iter()
        .try_for_each(|c| insert_specs(&mut specs, c))?;

    Ok(Vec::from_iter(specs))
}
