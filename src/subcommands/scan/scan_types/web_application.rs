use std::time::Duration;

use bollard::Docker;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::helpers::{enygmah_docker::get_docker, logger, scan, tools::Tools};

pub async fn analyze(url: &String) {
    let docker: Docker = get_docker();
    execute_remote_analysis(url, &docker).await;
}

pub async fn execute_remote_analysis(url_path: &str, docker: &Docker) {
    println!(""); // add a space

    logger::create_log("Starting analysis...\n", logger::EnygmahLogType::MainStep);

    let m: MultiProgress = MultiProgress::new();

    tokio::join!(
        create_progress_bar_and_run_scan(Tools::WpScan, url_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::OwaspZapProxy, url_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::Nikto, url_path, docker, &m),
        create_progress_bar_and_run_scan(Tools::Nuclei, url_path, docker, &m),
        // create_progress_bar_and_run_scan(Tools::Wapiti, url_path, docker, &m),
    );
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
