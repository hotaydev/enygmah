use crate::helpers::{
    enygmah_docker::{self, get_docker},
    logger,
};
use bollard::{container::UploadToContainerOptions, Docker};
use log::debug;
use std::{path::Path, process};
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

    let file = create_tarball_from_folder(&code_path, folder_name);

    match docker
        .upload_to_container("enygmah", upload_container_options, file.into())
        .await
    {
        Ok(_) => {
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
    // Static Code Analisys can be made using: Trivy, Sonarqube, CppCheck, OsvScanner, GoSec, Semgrep and SpotBugs

    tokio::join!(
        enygmah_docker::execute_command(
            docker,
            format!("trivy fs --scanners vuln,misconfig,secret -f json -o /home/enygmah/_outputs/trivy.json {}", container_path),
        ),
    
        enygmah_docker::execute_command(
            docker,
            format!(
                "osv-scanner scan --format json --output /home/enygmah/_outputs/osv-scanner.json {}",
                container_path
            ),
        ),
    
        // TODO: see a way to allow users to do `semgrep login`, being able to run more advanced scans.
        enygmah_docker::execute_command(
            docker,
            format!(
                "semgrep scan --json --output /home/enygmah/_outputs/semgrep.json {}",
                container_path
            ),
        ),
    );
}

async fn cleanup_copied_folder(container_path: &str, docker: &Docker) {
    enygmah_docker::execute_command(docker, format!("rm -rf {}", container_path)).await;
    // enygmah_docker::execute_command(docker, String::from("rm -rf /home/enygmah/_outputs/")).await;
}
