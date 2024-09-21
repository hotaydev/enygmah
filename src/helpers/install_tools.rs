use bollard::{
    container::ListContainersOptions,
    image::ListImagesOptions,
    secret::{ContainerSummary, ImageSummary},
    Docker,
};

use log::debug;

use super::{
    docker::pull_docker_image,
    enygmah_docker::{create_enygmah_container, get_docker, start_enygmah_container},
    logger,
};

pub async fn install_tools() {
    let docker = get_docker();

    pull_docker_image_if_needed(&docker).await;
    run_enygmah_docker_image(&docker).await;
}

async fn pull_docker_image_if_needed(docker: &Docker) {
    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    match get_docker_image_id(&images) {
        Some(id) => {
            debug!("enygmah docker image found: {}", id);
        }
        None => {
            pull_docker_image(docker, "hotay/enygmah").await;
        }
    };
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
            start_enygmah_container(docker).await;
        }
        None => {
            create_enygmah_container(docker).await;
        }
    }

    logger::create_log("Container ready to use.", logger::EnygmahLogType::Info);
}
