use super::{enygmah_docker, logger};
use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions, StartContainerOptions},
    exec::{CreateExecOptions, StartExecResults},
    image::{CreateImageOptions, ListImagesOptions},
    secret::{ContainerSummary, HostConfig, ImageSummary},
    Docker,
};
use futures_util::StreamExt;
use log::debug;
use std::{path::Path, process, time::Duration};
use tokio::time::sleep;

// We will use the Sonarqube only when it's really needed (for local/remote source code).
// TODO: here is a lot of duplicated code from `enygmah_docker.rs` and `install_tools.rs`, fix it.

// TODO: before starting we need to follow these instructions depending on the user platform: https://docs.sonarsource.com/sonarqube/latest/setup-and-upgrade/pre-installation/linux/
pub async fn start(docker: &Docker, container_path: &str) {
    let containers = &docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    match get_docker_container_id(&containers) {
        Some(_) => {
            start_sonarqube_container(docker).await;
        }
        None => {
            let container_options = Some(CreateContainerOptions {
                name: "sonarqube-enygmah",
                ..Default::default()
            });

            let container_config = Config {
                image: Some("sonarqube:community"),
                host_config: Some(HostConfig {
                    network_mode: Some(String::from("enygmah-network")),
                    ..Default::default()
                }),
                ..Default::default()
            };

            match docker
                .create_container(container_options, container_config)
                .await
            {
                Ok(result) => {
                    debug!("sonarqube docker container created: {:?}", &result);
                    start_sonarqube_container(docker).await;
                }
                Err(err) => {
                    logger::create_log(
                    &format!(
                        "An error occured while creating the Sonarqube docker container. Output:\n{}",
                        err
                    ),
                    logger::EnygmahLogType::Error,
                );
                    process::exit(1);
                }
            };
        }
    }

    execute_command_when_ready(docker, container_path).await;
}

async fn start_sonarqube_container(docker: &Docker) {
    let container_options = Some(StartContainerOptions::<String> {
        ..Default::default()
    });

    match docker
        .start_container("sonarqube-enygmah", container_options)
        .await
    {
        Ok(result) => {
            debug!("Starting Sonarqube container: {:?}", result);
            // TODO: Await it to completelly start and then execute the following command:
            // curl -u admin:admin -X POST "http://localhost:9000/api/users/change_password?login=admin&previousPassword=admin&password=sonar"
            // The scan is actually working without changing the password... se if we can get results without changing the pass (it would be less steps to run :) )
        }
        Err(err) => {
            logger::create_log(
                &format!(
                    "It wasn't possible to run Sonarqube docker container. Output:\n{}",
                    err
                ),
                logger::EnygmahLogType::Error,
            );
            process::exit(1);
        }
    }
}

pub async fn download_sonarqube_if_needed(docker: &Docker) {
    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    match get_docker_image_id(&images) {
        Some(id) => {
            debug!("Sonarqube docker image found: {}", id);
        }
        None => {
            let pull_options = Some(CreateImageOptions {
                from_image: "sonarqube",
                tag: "community",
                ..Default::default()
            });

            let mut stream = docker.create_image(pull_options, None, None);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(_) => {}
                    Err(err) => {
                        logger::create_log(
                            &format!(
                                "Failed pulling Sonarqube docker image. Error logs:\n{}",
                                err,
                            ),
                            logger::EnygmahLogType::Error,
                        );
                        process::exit(1);
                    }
                }
            }
        }
    };
}

fn get_docker_image_id(images: &Vec<ImageSummary>) -> Option<String> {
    let search_tag = "sonarqube".to_string();
    let mut docker_image_id: Option<String> = None;
    for image in images {
        for tag in image.repo_tags.iter() {
            if tag.contains(&search_tag) {
                docker_image_id = Some(image.id.clone());
                break;
            }
        }
    }

    docker_image_id
}

fn get_docker_container_id(containers: &Vec<ContainerSummary>) -> Option<String> {
    let mut docker_container_id: Option<String> = None;
    for container in containers {
        let container_name = container.names.as_deref().expect("").first().expect("");
        if container_name.contains(&"sonarqube-enygmah".to_string()) {
            docker_container_id = Some(container.id.clone().expect(""));
            break;
        }
    }

    docker_container_id
}

async fn execute_command_when_ready(docker: &Docker, container_path: &str) {
    // TODO: run this loop 20 times, if this isn't enough to Sonarqube to be ready, then break the loop
    loop {
        let response = check_if_sonarqube_is_ready(docker).await;
        if response.contains("\"status\":\"UP\"") {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }

    run_sonarqube_scan(docker, container_path).await;
}

async fn run_sonarqube_scan(docker: &Docker, container_path: &str) {
    enygmah_docker::execute_command(
        docker,
        format!(
            "sonar-scanner -D sonar.login=admin -D sonar.password=admin -D sonar.host.url=http://sonarqube-enygmah:9000 -D sonar.projectKey={} -D sonar.sources={}",
            Path::new(container_path).file_name().unwrap().to_str().unwrap(),
            container_path,
        ),
    ).await;
}

async fn check_if_sonarqube_is_ready(docker: &Docker) -> String {
    let command = Vec::from([
        "curl",
        "-q",
        "-u",
        "admin:admin",
        "-X",
        "GET",
        "http://localhost:9000/api/system/status",
    ]);

    let exec = docker
        .create_exec(
            "sonarqube-enygmah",
            CreateExecOptions {
                cmd: Some(command),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;

    let mut response: String = String::new();

    if let StartExecResults::Attached { mut output, .. } =
        docker.start_exec(&exec, None).await.unwrap()
    {
        while let Some(Ok(msg)) = output.next().await {
            response = msg.to_string();
            debug!("{msg}");
        }
    }

    response.to_string()
}
