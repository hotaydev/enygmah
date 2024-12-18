use bollard::{container::DownloadFromContainerOptions, Docker};
use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::helpers::enygmah_docker::{self, get_docker};

pub async fn clean_up() {
    let docker: Docker = get_docker();

    // TODO: Instead of downloading the tarball, we should add this to a local .enygmah folder and serve the results in a web server.
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
}

pub async fn delete_created_folder(docker: &Docker, folder: &str) {
    enygmah_docker::execute_command(docker, format!("rm -rf {}", folder)).await;
}
