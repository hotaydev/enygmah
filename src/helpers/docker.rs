use std::{collections::HashMap, process};

use bollard::{image::CreateImageOptions, Docker};
use futures_util::StreamExt;
use indicatif::{HumanBytes, MultiProgress, ProgressBar, ProgressStyle};

use super::logger;

// TODO: there's a lot of duplicated code related to Docker Daemon functions, consolidate them here.

pub async fn pull_docker_image(docker: &Docker, image_name: &str) {
    logger::create_log(
        &format!("{} docker image not found, pulling...", image_name),
        logger::EnygmahLogType::Warn,
    );

    let pull_options = Some(CreateImageOptions {
        from_image: image_name,
        tag: "latest",
        ..Default::default()
    });

    let mut stream = docker.create_image(pull_options, None, None);

    // Initialize MultiProgress to manage multiple progress bars
    let multi_progress = MultiProgress::new();
    let progress_style: ProgressStyle = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {wide_bar:.cyan/blue} {msg}")
        .unwrap()
        .progress_chars("##-");

    // HashMap to store progress bars by id
    let mut progress_bars: HashMap<String, (ProgressBar, i64, i64)> = HashMap::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(create_image_info) => {
                if let Some(id) = &create_image_info.id {
                    if let Some(progress_detail) = &create_image_info.progress_detail {
                        let current = progress_detail.current.unwrap_or(0);
                        let total = progress_detail.total.unwrap_or(0);

                        if total != 0 && total != 0 {
                            // Get or create a progress bar for this id
                            let (progress_bar, _, _) =
                                progress_bars.entry(id.clone()).or_insert_with(|| {
                                    let pb = multi_progress.add(ProgressBar::new(total as u64));
                                    pb.set_style(progress_style.clone());
                                    pb.set_message(format!("layer {}", id.clone()));
                                    (pb, 0, 0)
                                });

                            // Update the progress bar with the current value
                            progress_bar.set_position(current as u64);
                            progress_bar.set_length(total as u64);

                            if current == total {
                                progress_bar.finish_with_message("Done");
                            } else {
                                progress_bar.set_message(format!(
                                    "{}/{}",
                                    HumanBytes(current as u64),
                                    HumanBytes(total as u64)
                                ));
                            }
                        }
                    }
                }
            }
            Err(err) => {
                logger::create_log(
                    &format!(
                        "Failed pulling {} docker image. Error logs:\n{}",
                        image_name, err,
                    ),
                    logger::EnygmahLogType::Error,
                );
                process::exit(1);
            }
        }
    }

    multi_progress
        .clear()
        .expect("Failed removing progress bars...");
    logger::create_log("Docker image pulled!\n", logger::EnygmahLogType::Success);
}
