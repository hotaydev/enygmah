use crate::helpers::{docker::get_docker, logger};
use bollard::container::UploadToContainerOptions;
use log::debug;
use std::{fs::File, io::Read, path::Path, process};
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

    let upload_container_options = Some(UploadToContainerOptions {
        path: format!("/home/enygmah/analyze/{}", folder_name),
        ..Default::default()
    });

    let mut file = create_tarball_from_folder(&code_path, folder_name);
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    match docker
        .upload_to_container("enygmah", upload_container_options, contents.into())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            logger::create_log("Ocurred and error while sending the folder to be analyzed in the container. Re-run with -v to see the verbose debug output.", logger::EnygmahLogType::Error);
            debug!("{}", err);
            process::exit(1);
        }
    };
}

fn create_tarball_from_folder(path: &Path, folder_name: &str) -> File {
    let file = File::create(format!("{}.tar.gz", folder_name)).unwrap();
    let mut tarball = Builder::new(file);

    for single_file_path in path.read_dir().unwrap() {
        tarball
            .append_path(single_file_path.unwrap().path())
            .unwrap();
    }

    tarball.into_inner().unwrap()
}
