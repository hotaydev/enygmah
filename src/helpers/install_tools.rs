use bollard::{image::ListImagesOptions, Docker};

use log::{debug, error};
use std::process;

pub async fn install_tools() {
    // Detect OS: https://doc.rust-lang.org/std/env/consts/constant.OS.html
    // Progress bars: https://docs.rs/indicatif/latest/indicatif/

    // Here we need to download the Docker image containing all the tools and run the container
    // https://github.com/fussybeaver/bollard?tab=readme-ov-file

    let docker: Docker = match Docker::connect_with_local_defaults() {
        Ok(res) => {
            debug!("Docker instance: {:#?}", res);
            res
        }
        Err(err) => {
            // TODO: handle each case apart
            error!(
                "Ensure that Docker is installed and running.\nError: {}",
                err
            );
            process::exit(1);
        }
    };

    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    for image in images {
        println!("-> {:?}", image);
    }
}
