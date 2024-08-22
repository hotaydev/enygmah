use bollard::{
    image::{CreateImageOptions, ListImagesOptions},
    secret::ImageSummary,
    Docker,
};

use futures_util::StreamExt;
use log::{debug, error, info, trace, warn};
use std::process;

pub async fn install_tools() {
    // Progress bars: https://docs.rs/indicatif/latest/indicatif/

    // Here we need to download the Docker image containing all the tools and run the container
    // https://github.com/fussybeaver/bollard?tab=readme-ov-file

    let docker: Docker = match Docker::connect_with_local_defaults() {
        Ok(res) => {
            debug!("Docker instance: {:#?}", res);
            res
        }
        Err(err) => {
            // TODO: handle each case apart
            error!(
                "Ensure that Docker is installed and running.\nError: {}",
                err
            );
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
            warn!("enygmah docker image not found, trying to pull...");

            let pull_options = Some(CreateImageOptions {
                from_image: "hotay/enygmah",
                // tag: "latest",
                ..Default::default()
            });

            let mut stream = docker.create_image(pull_options, None, None);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(create_image_info) => {
                        trace!("Docker pull update: {:?}", create_image_info);
                    }
                    Err(e) => {
                        error!("Failed pulling enygmah docker image. Error logs:\n{}", e);
                        process::exit(1);
                    }
                }
            }

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
