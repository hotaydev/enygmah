use std::{collections::HashMap, process};

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    exec::{CreateExecOptions, StartExecResults},
    network::{CreateNetworkOptions, ListNetworksOptions},
    secret::HostConfig,
    Docker,
};
use futures_util::StreamExt;
use log::debug;

use crate::subcommands::scan::hooks;

use super::logger;

pub fn get_docker() -> Docker {
    return match Docker::connect_with_local_defaults() {
        Ok(res) => {
            debug!("Docker instance: {:#?}", res);
            res
        }
        Err(err) => {
            match err {
                bollard::errors::Error::SocketNotFoundError(_) => {
                    logger::create_log("It wasn't possible to connect to Docker Socket. Ensure that Docker is installed and running.", logger::EnygmahLogType::Error);
                }
                _ => {
                    logger::create_log(
                        &format!(
                            "An error occurred trying to connect to docker socket: {}",
                            err,
                        ),
                        logger::EnygmahLogType::Error,
                    );
                }
            }
            process::exit(1);
        }
    };
}

pub async fn start_enygmah_container(docker: &Docker) {
    let container_options = Some(StartContainerOptions::<String> {
        ..Default::default()
    });

    match docker.start_container("enygmah", container_options).await {
        Ok(result) => {
            debug!("Starting container: {:?}", result);
            hooks::pre_scan::run(Some(docker)).await;
        }
        Err(err) => {
            logger::create_log(
                &format!(
                    "It wasn't possible to run enygmah docker container. Output:\n{}",
                    err
                ),
                logger::EnygmahLogType::Error,
            );
            process::exit(1);
        }
    }
}

pub async fn create_enygmah_container(docker: &Docker) {
    let container_options = Some(CreateContainerOptions {
        name: "enygmah",
        ..Default::default()
    });

    create_network(docker).await;

    // also mounts /var/run/docker.sock:/var/run/docker.sock
    let container_config = Config {
        image: Some("hotay/enygmah"),
        host_config: Some(HostConfig {
            network_mode: Some(String::from("enygmah-network")),
            binds: Some(vec![String::from(
                "/var/run/docker.sock:/var/run/docker.sock",
            )]),
            ..Default::default()
        }),
        volumes: Some(
            [("/var/run/docker.sock", HashMap::new())]
                .iter()
                .cloned()
                .collect(),
        ),
        ..Default::default()
    };

    match docker
        .create_container(container_options, container_config)
        .await
    {
        Ok(result) => {
            debug!("enygmah docker container created: {:?}", &result);
            start_enygmah_container(docker).await;
        }
        Err(err) => {
            logger::create_log(
                &format!(
                    "An error occured while creating the enygmah docker container. Output:\n{}",
                    err
                ),
                logger::EnygmahLogType::Error,
            );
            process::exit(1);
        }
    };
}

pub async fn execute_command(docker: &Docker, command: String) {
    let exec = docker
        .create_exec(
            "enygmah",
            CreateExecOptions {
                cmd: Some(command.split(" ").collect::<Vec<&str>>()),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;

    if let StartExecResults::Attached { mut output, .. } =
        docker.start_exec(&exec, None).await.unwrap()
    {
        while let Some(Ok(msg)) = output.next().await {
            debug!("{msg}");
        }
    }
}

async fn create_network(docker: &Docker) {
    // First check if the network alreaty exists

    let networks = docker
        .list_networks(Some(ListNetworksOptions {
            filters: HashMap::from([("name", vec!["enygmah-network"])]),
            ..Default::default()
        }))
        .await
        .unwrap();

    if networks.len() == 0 {
        match docker
            .create_network(CreateNetworkOptions {
                name: "enygmah-network",
                ..Default::default()
            })
            .await
        {
            Ok(response) => {
                debug!("Created docker network: {:?}", response);
            }
            Err(err) => {
                logger::create_log(
                    &format!(
                        "An error occured while creating the enygmah docker network. Output:\n{}",
                        err
                    ),
                    logger::EnygmahLogType::Error,
                );
                process::exit(1);
            }
        };
    }
}
