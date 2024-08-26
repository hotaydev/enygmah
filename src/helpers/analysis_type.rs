use bollard::{image::ListImagesOptions, Docker};
use log::debug;
use std::{path::Path, process};
use url::Url;

use super::{docker::get_docker, logger};

pub enum AnalysisType {
    /// Web Application for a dinamyc analysis
    WebApp,

    /// Mobile Application, for example an .apk or .aab
    MobileApp,

    /// Local Source Code
    SourceCode,

    /// Remote Source Code - Repository - Currently only from GitHub.com
    RemoteRepository,

    /// Binary program, such as an executable file from Windows, Go Lang or Rust
    Binary,

    /// Analyze a Docker Image
    DockerImage,

    /// Was not possible to detect the type
    Undetected,
}

pub async fn detect_analysis_type(asset: &String) -> AnalysisType {
    if let Ok(parsed_url) = Url::parse(asset) {
        // Check if the URL is from github.com
        if let Some(host) = parsed_url.host_str() {
            if host == "github.com" {
                return AnalysisType::RemoteRepository;
            }
        }

        return AnalysisType::WebApp;
    }

    let possible_path = Path::new(asset);
    if possible_path.is_dir() {
        return AnalysisType::SourceCode;
    } else if possible_path.is_file() {
        let mobile_app_extensions: [&str; 3] = ["apk", "aab", "ipa"];

        if let Some(extension) = possible_path.extension().and_then(|e| e.to_str()) {
            if mobile_app_extensions.contains(&extension) {
                return AnalysisType::MobileApp;
            }
        } else {
            debug!("File has no extension");
        }

        return AnalysisType::Binary;
    }

    return match asset_is_a_docker_image(asset).await {
        Some(_) => AnalysisType::DockerImage,
        None => AnalysisType::Undetected,
    };
}

pub fn log_analysis_type(asset_type: &AnalysisType) {
    match asset_type {
        AnalysisType::WebApp => logger::create_log(
            "Asset to be analyzed is a Web Application",
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::MobileApp => logger::create_log(
            "Asset to be analyzed is a Mobile App",
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::SourceCode => logger::create_log(
            "Asset to be analyzed is a Local Source Code",
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::RemoteRepository => logger::create_log(
            "Asset to be analyzed is a Remote Source Code",
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::Binary => logger::create_log(
            "Asset to be analyzed is a Binary",
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::DockerImage => logger::create_log(
            "Asset to be analyzed is a Docker Image",
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::Undetected => logger::create_log(
            "Type of the asset being analyzed wasn't detected.\n",
            logger::EnygmahLogType::Warn,
        ),
    }
}

async fn asset_is_a_docker_image(asset: &String) -> Option<String> {
    let docker = get_docker();

    let images = docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    let mut docker_image_id: Option<String> = None;
    for image in images {
        for tag in image.repo_tags.iter() {
            if tag.contains(&*asset) {
                docker_image_id = Some(image.id.clone());
                break;
            }
        }
    }

    docker_image_id
}

pub fn get_analysis_text(asset_type: AnalysisType) -> String {
    return match asset_type {
        AnalysisType::WebApp => String::from("Web Application"),
        AnalysisType::MobileApp => String::from("Mobile App"),
        AnalysisType::SourceCode => String::from("Local Source Code"),
        AnalysisType::RemoteRepository => String::from("Remote Code - Repository"),
        AnalysisType::Binary => String::from("Binary Application"),
        AnalysisType::DockerImage => String::from("Docker Image"),
        AnalysisType::Undetected => String::from("Undetected"),
    };
}

pub fn get_analysis_type(asset_text: &str) -> AnalysisType {
    return match asset_text {
        "Web Application" => AnalysisType::WebApp,
        "Mobile App" => AnalysisType::MobileApp,
        "Local Source Code" => AnalysisType::SourceCode,
        "Remote Code - Repository" => AnalysisType::RemoteRepository,
        "Binary Application" => AnalysisType::Binary,
        "Docker Image" => AnalysisType::DockerImage,
        _ => AnalysisType::Undetected,
    };
}
