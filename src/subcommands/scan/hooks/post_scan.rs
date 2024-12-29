use bollard::{container::DownloadFromContainerOptions, Docker};
use futures_util::StreamExt;
use std::io;
use tar::Archive;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::helpers::{
    enygmah_docker::{self, get_docker},
    logger,
};

pub async fn clean_up() {
    let docker: Docker = get_docker();

    // Delete the results folder if it already exists
    if let Err(e) = std::fs::remove_dir_all("_outputs") {
        // Ignore the error if the directory does not exist
        if e.kind() != std::io::ErrorKind::NotFound {
            panic!("Failed to remove directory: {}", e);
        }
    }

    // TODO: this "merge" commands aren't working yet...
    // enygmah_docker::execute_command(&docker, String::from("jq -s '{\"version\": \"2.1.0\", \"$schema\": \"https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json\", \"runs\": map(.runs) | add}' /home/enygmah/_outputs/*.sarif > output.sarif")).await;
    // enygmah_docker::execute_command(&docker, String::from("find /home/enygmah/_outputs/ -maxdepth 1 -type f -name \"*.sarif\" ! -name \"output.sarif\" -exec rm {} +")).await;

    let mut file: File = File::create("analysis-results.tar").await.unwrap();

    // Download the content from the container as a stream
    let mut stream = docker.download_from_container(
        "enygmah",
        Some(DownloadFromContainerOptions {
            path: "/home/enygmah/_outputs/",
        }),
    );

    // Write the stream to the file
    while let Some(chunk) = stream.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).await.unwrap();
    }

    // Ensure the file is properly flushed
    file.flush().await.unwrap();

    enygmah_docker::execute_command(&docker, String::from("rm -rf /home/enygmah/_outputs/")).await;

    untar_file("analysis-results.tar").unwrap();
    std::fs::remove_file("analysis-results.tar").unwrap();

    println!(""); // Just add a space
    logger::create_log(
        "Results exported to `_outputs` folder.",
        logger::EnygmahLogType::Info,
    );
    logger::create_log(
        "To see Sarif results on VS Code, install the extension: https://marketplace.visualstudio.com/items?itemName=MS-SarifVSCode.sarif-viewer",
        logger::EnygmahLogType::Info,
    );
}

pub async fn delete_created_folder(docker: &Docker, folder: &str) {
    enygmah_docker::execute_command(docker, format!("rm -rf {}", folder)).await;
}

// TODO: define the output path from a variable
fn untar_file(tar_path: &str) -> io::Result<()> {
    let tar_file = std::fs::File::open(tar_path).unwrap();
    let mut archive = Archive::new(tar_file);
    archive.unpack(".").unwrap();
    Ok(())
}
