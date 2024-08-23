use bollard::{
    image::{CreateImageOptions, ListImagesOptions},
    secret::ImageSummary,
    Docker,
};

use futures_util::StreamExt;
use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, error, info};
use std::{collections::HashMap, process};

pub async fn install_tools() {
    let docker: Docker = match Docker::connect_with_local_defaults() {
        Ok(res) => {
            debug!("Docker instance: {:#?}", res);
            res
        }
        Err(err) => {
            match err {
                bollard::errors::Error::SocketNotFoundError(_) => {
                    error!("It wasn't possible to connect to Docker Socket. Ensure that Docker is installed and running.");
                }
                _ => {
                    error!(
                        "An error occurred trying to connect to docker socket: {}",
                        err
                    );
                }
            }
            process::exit(1);
        }
    };

    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let docker_image_id: String = match get_docker_image_id(&images) {
        Some(id) => {
            info!("enygmah docker image found.");
            id
        }
        None => {
            println!("enygmah docker image not found, pulling...");

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
                    Err(e) => {
                        error!("Failed pulling enygmah docker image. Error logs:\n{}", e);
                        process::exit(1);
                    }
                }
            }

            multi_progress
                .clear()
                .expect("Failed removing progress bars...");
            println!("Docker image pulled!");

            let updated_images_list = &docker
                .list_images(Some(ListImagesOptions::<String> {
                    all: true,
                    ..Default::default()
                }))
                .await
                .unwrap();

            let new_id: String = match get_docker_image_id(&updated_images_list) {
                Some(id) => {
                    info!("enygmah docker image pulled.");
                    id
                }
                None => {
                    error!("Failed pulling enygmah docker image. Try pulling manually with `docker pull hotay/enygmah`");
                    process::exit(1);
                }
            };

            new_id
        }
    };

    println!("{}", docker_image_id);
    // run the docker_image_id image
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
