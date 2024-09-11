use crate::helpers::{
    enygmah_docker::{self, get_docker},
    logger, scan,
    tools::Tools,
};
use bollard::{
    container::{DownloadFromContainerOptions, UploadToContainerOptions},
    Docker,
};
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::debug;
use std::{
    io::Write,
    path::{Path, PathBuf},
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tar::Builder;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn analyze(path: &String) {
    let code_path: PathBuf = match Path::new(path).canonicalize() {
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

    let docker: Docker = get_docker();

    let folder_name: &str = match code_path.file_name() {
        Some(value) => value
            .to_str()
            .expect("Generic error, failed to convert OsStr to str"),
        None => {
            process::exit(1);
        }
    };

    let remote_container_path: String = format!("/home/enygmah/{}", folder_name);

    let loading = Arc::new(AtomicBool::new(true));
    let loading_clone = Arc::clone(&loading);

    let handle = logger::create_loading_log(
        String::from("Creating tarball from the directory"),
        loading_clone,
    );

    let file: Vec<u8> = create_tarball_from_folder(&code_path, folder_name);

    match docker
        .upload_to_container(
            "enygmah",
            Some(UploadToContainerOptions {
                path: "/home/enygmah/",
                ..Default::default()
            }),
            file.into(),
        )
        .await
    {
        Ok(_) => {
            loading.store(false, Ordering::SeqCst);
            handle.join().unwrap();

            print!("{}", "\x1B[?25h"); // Show the cursor again
            print!("\r{}{}", "\x1B[F", "\r"); // Delete the line that was keep in blank to separate the "loading message"
            std::io::stdout().flush().unwrap();

            logger::create_log(
                "Tarball uploaded to the container to be analysed",
                logger::EnygmahLogType::Info,
            );
            print!("{}", "\x1B[2K");
            std::io::stdout().flush().unwrap();

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
    let mut tarball: Builder<Vec<u8>> = Builder::new(Vec::new());
    // TODO: check first if every file exists (thread panicks when some file/folder is deleted while compressing)
    tarball.append_dir_all(folder_name, path).unwrap();

    tarball.into_inner().unwrap()
}

async fn execute_remote_analysis(container_path: &str, docker: &Docker) {
    println!(""); // add a space

    logger::create_log("Starting analysis...\n", logger::EnygmahLogType::MainStep);

    let m: MultiProgress = MultiProgress::new();

    tokio::join!(
        // TODO: add CppCheck, GoSec and SpotBugs
        create_progress_bar_and_run_scan(Tools::Trivy, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::OsvScanner, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::CppCheck, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::GoSec, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::Semgrep, container_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::Sonarqube, container_path, docker, &m),
    );

    // TODO: Instead of downloading the tarball, we should add this to a local .enygmah folder and serve the results in a web server.
    let mut file: File = File::create("output.tar").await.unwrap();

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
}

async fn create_progress_bar_and_run_scan(
    tool: Tools,
    asset: &str,
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
    scan::run_scan(tool, asset, docker, &spinner).await;
}

async fn cleanup_copied_folder(container_path: &str, docker: &Docker) {
    enygmah_docker::execute_command(docker, format!("rm -rf {}", container_path)).await;
    enygmah_docker::execute_command(docker, String::from("rm -rf /home/enygmah/_outputs/")).await;
}
