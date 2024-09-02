use crate::helpers::{
    enygmah_docker::{self, get_docker},
    logger, scan,
    tools::Tools,
};
use bollard::{container::UploadToContainerOptions, Docker};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::debug;
use std::{path::Path, process, time::Duration};
use tar::Builder;

pub async fn analyze(path: &String) {
    let code_path = match Path::new(path).canonicalize() {
        Ok(value) => value,
        Err(err) => {
            logger::create_log(
                "It seems that this folder path is strange...",
                logger::EnygmahLogType::Error,
            );
            debug!("Error Output: {}", err);
            process::exit(1);
        }
    };

    if !code_path.is_dir() {
        logger::create_log("Folder path not found", logger::EnygmahLogType::Error);
        process::exit(1);
    }

    let docker = get_docker();

    let folder_name = match code_path.file_name() {
        Some(value) => value
            .to_str()
            .expect("Generic error, failed to convert OsStr to str"),
        None => {
            process::exit(1);
        }
    };

    let remote_container_path = format!("/home/enygmah/{}", folder_name);
    let upload_container_options = Some(UploadToContainerOptions {
        path: "/home/enygmah/",
        ..Default::default()
    });

    logger::create_log(
        "Creating tarball from the directory...",
        logger::EnygmahLogType::Info,
    );

    let file = create_tarball_from_folder(&code_path, folder_name);

    match docker
        .upload_to_container("enygmah", upload_container_options, file.into())
        .await
    {
        Ok(_) => {
            logger::create_log(
                "Uploaded tarball to the container to be analysed",
                logger::EnygmahLogType::Info,
            );
            execute_remote_analysis(&remote_container_path, &docker).await;
            cleanup_copied_folder(&remote_container_path, &docker).await;
        }
        Err(err) => {
            logger::create_log("Ocurred and error while sending the folder to be analyzed in the container. Re-run with -vvv to see the verbose debug output.", logger::EnygmahLogType::Error);
            debug!("{}", err);
            process::exit(1);
        }
    };
}

// TODO: avoid using .unwrap();
fn create_tarball_from_folder(path: &Path, folder_name: &str) -> Vec<u8> {
    let mut tarball = Builder::new(Vec::new());
    tarball.append_dir_all(folder_name, path).unwrap();

    tarball.into_inner().unwrap()
}

async fn execute_remote_analysis(container_path: &str, docker: &Docker) {
    // TODO: add analysis with Sonarqube, CppCheck, GoSec and SpotBugs

    println!(""); // add a space

    logger::create_log("Starting analysis...\n", logger::EnygmahLogType::MainStep);

    let m = MultiProgress::new();

    tokio::join!(
        create_progress_bar_and_run_scan(Tools::Trivy, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::OsvScanner, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::Semgrep, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::Sonarqube, container_path, docker, &m),
    );
}

async fn create_progress_bar_and_run_scan(
    tool: Tools,
    asset: &str,
    docker: &Docker,
    m: &MultiProgress,
) {
    let spinner = m.add(ProgressBar::new_spinner());
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⨁︎", "⨂︎", "⨁︎", "⨂︎"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    scan::run_scan(tool, asset, docker, &spinner).await;
}

async fn cleanup_copied_folder(container_path: &str, docker: &Docker) {
    enygmah_docker::execute_command(docker, format!("rm -rf {}", container_path)).await;
    // TODO: organize results before deleting the folder below
    // enygmah_docker::execute_command(docker, String::from("rm -rf /home/enygmah/_outputs/")).await;
}
