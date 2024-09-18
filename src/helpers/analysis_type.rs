use bollard::image::ListImagesOptions;
use colored::Colorize;
use log::debug;
use std::path::Path;
use url::Url;

use super::{enygmah_docker::get_docker, logger};

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

// TODO: add an option to allow the user to say which kind of application it is, without trying to detect
pub async fn detect_analysis_type(asset: &String) -> AnalysisType {
    if let Ok(parsed_url) = Url::parse(asset) {
        // TODO: we can test also if the input is just <user>/<repo>, without http, ssh, or .git at the final

        // Test if the URL ends with .git
        let url_path = parsed_url.path();
        if url_path.starts_with("git@") || url_path.ends_with(".git") {
            return AnalysisType::RemoteRepository;
        }

        // It can also be a URL from a Git Registry, but without the .git extension
        if let Some(host) = parsed_url.host_str() {
            if host.contains("github.com")
                || host.contains("gitlab.com")
                || host.contains("bitbucket.org")
                || host.contains("git.launchpad.net")
                || host.contains("pagure.io")
                || host.contains("codeberg.org")
                || host.contains("gitea.com")
                || (host.contains("dev.azure.com") && url_path.contains("_git"))
            {
                return AnalysisType::RemoteRepository;
            }
        }

        // If it isn't from a Git Repo, it's probably a Web Application
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
            &format!(
                "Asset to be analyzed is a {}",
                "Web Application".bold().underline()
            ),
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::MobileApp => logger::create_log(
            &format!(
                "Asset to be analyzed is a {}",
                "Mobile App".bold().underline()
            ),
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::SourceCode => logger::create_log(
            &format!(
                "Asset to be analyzed is a {}",
                "Local Source Code".bold().underline()
            ),
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::RemoteRepository => logger::create_log(
            &format!(
                "Asset to be analyzed is a {}",
                "Remote Source Code".bold().underline()
            ),
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::Binary => logger::create_log(
            &format!("Asset to be analyzed is a {}", "Binary".bold().underline()),
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::DockerImage => logger::create_log(
            &format!(
                "Asset to be analyzed is a {}",
                "Docker Image".bold().underline()
            ),
            logger::EnygmahLogType::Info,
        ),
        AnalysisType::Undetected => logger::create_log(
            "Type of the asset being analyzed wasn't detected.",
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
