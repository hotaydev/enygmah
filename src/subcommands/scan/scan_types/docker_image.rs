use std::time::Duration;

use crate::helpers::{
    docker::pull_docker_image, enygmah_docker::get_docker, logger, scan, tools::Tools,
};
use bollard::{image::ListImagesOptions, Docker};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub async fn analyze(docker_image: &String) {
    let docker: Docker = get_docker();

    let image_exists = check_if_image_exists_locally(&docker, docker_image).await;

    if image_exists.is_none() {
        pull_docker_image(&docker, docker_image).await;
    }

    run_scan(&docker, docker_image).await;
}

async fn run_scan(docker: &Docker, docker_image: &String) {
    println!(""); // add a space

    logger::create_log("Starting analysis...\n", logger::EnygmahLogType::MainStep);

    let m: MultiProgress = MultiProgress::new();

    tokio::join!(
        create_progress_bar_and_run_scan(Tools::TrivyDocker, docker_image, docker, &m),
        create_progress_bar_and_run_scan(Tools::Grype, docker_image, docker, &m),
    );
}

async fn create_progress_bar_and_run_scan(
    tool: Tools,
    docker_image: &str,
    docker: &Docker,
    m: &MultiProgress,
) {
    let spinner: ProgressBar = m.add(ProgressBar::new_spinner());
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⨁︎", "⨂︎", "⨁︎", "⨂︎"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    scan::run_scan(tool, docker_image, docker, &spinner).await;
}

async fn check_if_image_exists_locally(docker: &Docker, asset: &String) -> Option<String> {
    let images = docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let mut docker_image_id: Option<String> = None;
    for image in images {
        for tag in image.repo_tags.iter() {
            if tag.contains(&*asset) {
                docker_image_id = Some(image.id.clone());
                break;
            }
        }
    }

    docker_image_id
}
