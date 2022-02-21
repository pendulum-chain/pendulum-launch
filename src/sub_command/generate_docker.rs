use crate::{
    error::Result,
    node::{AsCommand, Collator},
    Config,
};
use std::fs;

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

    config
        .collators
        .iter()
        .map(generate_collator_service)
        .collect::<Result<Vec<String>>>()?
        .into_iter()
        .for_each(|service| docker_compose.push_str(&format!("\n{}", service)));

    Ok(docker_compose)
}

fn generate_collator_service(collator: &Collator) -> Result<String> {
    let name = collator.name();
    let mut service = format!(
        r#"  {}:
    container_name: {} 
    image: pendulumchain/pendulum:latest
    build:
      context: .
      dockerfile: dockerfile"#,
        name, name
    );

    map_ports(collator)
        .into_iter()
        .for_each(|port| service.push_str(&format!("\n      {}", port)));

    service.push_str(&format!(
        "\n    restart: on-failure\n    command: {}",
        collator.as_command_external()?
    ));

    Ok(service)
}

fn map_ports(collator: &Collator) -> Vec<String> {
    let map_port = |(i, p): (usize, Option<u16>)| -> String {
        // SAFETY: we've filtered on None
        let outer_port = p.unwrap().to_string();
        let inner_port = match i {
            0 => "1",
            1 => "1",
            2 => "1",
            3 => "1",
            4 => "1",
            5 => "1",
            _ => "0", // we won't ever get here, since collator.ports() returns an array with a known size
        };

        format!("- \"{}:{}\"", outer_port, inner_port)
    };

    collator
        .ports()
        .into_iter()
        .filter(|p| p.is_some())
        .enumerate()
        .map(map_port)
        .collect()
}
