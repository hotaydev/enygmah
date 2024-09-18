use bollard::Docker;

use crate::{
    helpers::{
        enygmah_docker::{self, get_docker},
        logger,
    },
    subcommands::scan::hooks,
};
use std::process;

use super::local_repo;

pub async fn analyze(url: &String) {
    if url.starts_with("git@") {
        logger::create_log(
            "The remote analysis work only for HTTPS URLs. If you're using SSH, download the source code first an analyze it locally with `enygmah scan .`",
            logger::EnygmahLogType::Warn,
        );
        logger::create_log(
            "Also, we currently only support the \"main\" branch. For other branches, clone it first and run the analysis locally.",
            logger::EnygmahLogType::Warn,
        );
        process::exit(1);
    }

    let docker: Docker = get_docker();

    download_source_code(url, &docker).await;
    let remote_container_path: String = format!("/home/enygmah/{}", get_repo_name(url).unwrap());

    local_repo::execute_remote_analysis(&remote_container_path, &docker).await;
    hooks::post_scan::delete_created_folder(&docker, &remote_container_path).await;
}

async fn download_source_code(url: &String, docker: &Docker) {
    enygmah_docker::execute_command(docker, format!("git clone --depth=1 {}", url)).await;
}

fn get_repo_name(git_url: &str) -> Option<String> {
    let parts: Vec<&str> = git_url.split('/').collect();

    // Get the last part, which is the repo name with the .git extension
    if let Some(repo) = parts.last() {
        // Remove the .git extension, if present
        if let Some(repo_name) = repo.strip_suffix(".git") {
            return Some(repo_name.to_string());
        }
        return Some(repo.to_string());
    }

    None
}
