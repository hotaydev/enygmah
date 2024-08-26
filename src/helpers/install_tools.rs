use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions, StartContainerOptions},
    image::{CreateImageOptions, ListImagesOptions},
    secret::{ContainerSummary, ImageSummary},
    Docker,
};

use futures_util::StreamExt;
use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};
use log::debug;
use std::{collections::HashMap, process};

use super::{docker::get_docker, logger};

pub async fn install_tools() {
    let docker = get_docker();

    pull_docker_image_if_needed(&docker).await;
    run_enygmah_docker_image(&docker).await;
}

async fn pull_docker_image_if_needed(docker: &Docker) -> String {
    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let docker_image_id: String = match get_docker_image_id(&images) {
        Some(id) => {
            debug!("enygmah docker image found: {}", id);
            id
        }
        None => {
            logger::create_log(
                "enygmah docker image not found, pulling...",
                logger::EnygmahLogType::Warn,
            );

            let pull_options = Some(CreateImageOptions {
                from_image: "hotay/enygmah",
                tag: "latest",
                ..Default::default()
            });

            let mut stream = docker.create_image(pull_options, None, None);

            // Initialize MultiProgress to manage multiple progress bars
            let multi_progress = MultiProgress::new();
            let progress_style: ProgressStyle = ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {wide_bar:.cyan/blue} {msg}")
                .unwrap()
                .progress_chars("##-");

            // HashMap to store progress bars by id
            let mut progress_bars: HashMap<String, (ProgressBar, i64, i64)> = HashMap::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(create_image_info) => {
                        if let Some(id) = &create_image_info.id {
                            if let Some(progress_detail) = &create_image_info.progress_detail {
                                let current = progress_detail.current.unwrap_or(0);
                                let total = progress_detail.total.unwrap_or(0);

                                if total != 0 && total != 0 {
                                    // Get or create a progress bar for this id
                                    let (progress_bar, _, _) =
                                        progress_bars.entry(id.clone()).or_insert_with(|| {
                                            let pb =
                                                multi_progress.add(ProgressBar::new(total as u64));
                                            pb.set_style(progress_style.clone());
                                            pb.set_message(format!("layer {}", id.clone()));
                                            (pb, 0, 0)
                                        });

                                    // Update the progress bar with the current value
                                    progress_bar.set_position(current as u64);
                                    progress_bar.set_length(total as u64);

                                    if current == total {
                                        progress_bar.finish_with_message("Done");
                                    } else {
                                        progress_bar.set_message(format!(
                                            "{}/{}",
                                            HumanBytes(current as u64),
                                            HumanBytes(total as u64)
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        logger::create_log(
                            &format!("Failed pulling enygmah docker image. Error logs:\n{}", err,),
                            logger::EnygmahLogType::Error,
                        );
                        process::exit(1);
                    }
                }
            }

            multi_progress
                .clear()
                .expect("Failed removing progress bars...");
            logger::create_log("Docker image pulled!\n", logger::EnygmahLogType::Success);

            let updated_images_list = &docker
                .list_images(Some(ListImagesOptions::<String> {
                    all: true,
                    ..Default::default()
                }))
                .await
                .unwrap();

            let new_id: String = match get_docker_image_id(&updated_images_list) {
                Some(id) => {
                    debug!("enygmah docker image pulled: {}", id);
                    id
                }
                None => {
                    logger::create_log(
                        "Failed pulling enygmah docker image. Try pulling manually with `docker pull hotay/enygmah`",
                        logger::EnygmahLogType::Error,
                    );
                    process::exit(1);
                }
            };

            new_id
        }
    };

    docker_image_id
}

fn get_docker_image_id(images: &Vec<ImageSummary>) -> Option<String> {
    let search_tag = "hotay/enygmah".to_string();
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
        let container_image_name = container.image.as_deref().expect("");
        let container_name = container.names.as_deref().expect("").first().expect("");
        if container_name.contains(&"enygmah".to_string())
            && container_image_name.contains(&"hotay/enygmah".to_string())
        {
            docker_container_id = Some(container.id.clone().expect(""));
            break;
        }
    }

    docker_container_id
}

async fn run_enygmah_docker_image(docker: &Docker) {
    let containers = &docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    match get_docker_container_id(&containers) {
        Some(_) => {
            let container_options = Some(StartContainerOptions::<String> {
                ..Default::default()
            });
            match docker.start_container("enygmah", container_options).await {
                Ok(result) => {
                    debug!("Starting container: {:?}", result);
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
        None => {
            let container_options = Some(CreateContainerOptions {
                name: "enygmah",
                ..Default::default()
            });

            let container_config = Config {
                image: Some("hotay/enygmah"),
                ..Default::default()
            };

            let container_id = match docker
                .create_container(container_options, container_config)
                .await
            {
                Ok(result) => {
                    debug!("enygmah docker container created: {:?}", &result);
                    result.id
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

            match docker.start_container::<String>(&container_id, None).await {
                Ok(result) => {
                    debug!("Started docker container: {:?}", result);
                }
                Err(err) => {
                    logger::create_log(
                        &format!(
                            "Failed starting the enygmah docker container. Output:\n{}",
                            err
                        ),
                        logger::EnygmahLogType::Error,
                    );
                    process::exit(1);
                }
            }
        }
    }

    logger::create_log("Container ready to use.", logger::EnygmahLogType::Info);
}
