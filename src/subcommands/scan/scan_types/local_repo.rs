use std::{path::Path, process};

use crate::helpers::{docker::get_docker, logger};

pub fn analyze(path: &String) {
    let code_path = Path::new(path);
    if !code_path.is_dir() {
        logger::create_log("Folder path not found", logger::EnygmahLogType::Error);
        process::exit(1);
    }

    let docker = get_docker();
}
