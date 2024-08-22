use bollard::{image::ListImagesOptions, Docker};

use log::{debug, error, info};
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

    let mut docker_image_id: Option<String> = None;
    for image in images {
        if image.repo_tags.contains(&"hotay/enygmah".to_string()) {
            docker_image_id = Some(image.id.clone());
            break;
        }
    }

    let id = match docker_image_id {
        Some(id) => id,
        None => {
            info!("enygmah docker image not found, downloading...");
            process::exit(1); // TODO: download image
        }
    };

    // run the id image
}
