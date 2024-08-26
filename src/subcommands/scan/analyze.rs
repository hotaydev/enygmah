use std::process;

use crate::helpers::analysis_type::{detect_analysis_type, log_analysis_type, AnalysisType};
use crate::helpers::{install_tools, logger};

use super::scan_types::{
    binary, docker_image, local_repo, mobile_app, remote_repo, undetected, web_application,
};

pub async fn analyze(asset: &String) {
    if asset == "_output" {
        logger::create_log("We use `_output` as a special directory name. Try renaming your folder/repo to something different.", logger::EnygmahLogType::Error);
        process::exit(1);
    }

    install_tools::install_tools().await;
    let analyze = detect_analysis_type(asset).await;
    log_analysis_type(&analyze);

    analyze_asset_based_on_type(asset, analyze).await;
}

pub async fn analyze_asset_based_on_type(asset: &String, analysis_type: AnalysisType) {
    match analysis_type {
        AnalysisType::WebApp => web_application::analyze(asset).await,
        AnalysisType::MobileApp => mobile_app::analyze(asset).await,
        AnalysisType::SourceCode => local_repo::analyze(asset).await,
        AnalysisType::RemoteRepository => remote_repo::analyze(asset).await,
        AnalysisType::Binary => binary::analyze(asset).await,
        AnalysisType::DockerImage => docker_image::analyze(asset).await,
        AnalysisType::Undetected => undetected::analyze(asset).await,
    }
}
