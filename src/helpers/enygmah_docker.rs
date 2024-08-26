use std::process;

use bollard::{
    container::StartContainerOptions,
    exec::{CreateExecOptions, StartExecResults},
    Docker,
};
use futures_util::StreamExt;
use log::debug;

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
            execute_command(docker, String::from("mkdir /home/enygmah/_outputs")).await;
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
