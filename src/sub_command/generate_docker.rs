use crate::{
    error::Result,
    node::{AsCommand, Node},
    Config,
};
use std::fs;

// Static ports exposed in the base image
const INTERNAL_PORTS: [u16; 6] = [8844, 30344, 9944, 8855, 30355, 9955];

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

fn write_service<N>(docker_compose: &mut String, nodes: &Vec<N>) -> Result<()>
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

    map_ports(node)
        .into_iter()
        .for_each(|port| service.push_str(&format!("\n      {}", port)));

    service.push_str(&format!(
        "\n    restart: on-failure\n    command: {}",
        node.as_command_external()?
    ));

    Ok(service)
}

fn map_ports<N>(node: &N) -> Vec<String>
where
    N: Node,
{
    node.ports()
        .into_iter()
        .flatten()
        .zip(INTERNAL_PORTS)
        .map(|(outer_port, inner_port)| format!("- \"{}:{}\"", outer_port, inner_port))
        .collect()
}
