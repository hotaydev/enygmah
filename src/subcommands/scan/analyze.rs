use crate::helpers::analysis_type::{detect_analysis_type, log_analysis_type, AnalysisType};
use crate::helpers::install_tools;

use super::assets::{
    binary, docker_image, local_repo, mobile_app, remote_repo, undetected, web_application,
};

pub async fn analyze(asset: &String) {
    install_tools::install_tools().await;
    let analyze = detect_analysis_type(asset).await;
    log_analysis_type(&analyze);

    analyze_asset_based_on_type(asset, analyze);
}

pub fn analyze_asset_based_on_type(asset: &String, analysis_type: AnalysisType) {
    match analysis_type {
        AnalysisType::WebApp => web_application::analyze(asset),
        AnalysisType::MobileApp => mobile_app::analyze(asset),
        AnalysisType::SourceCode => local_repo::analyze(asset),
        AnalysisType::RemoteRepository => remote_repo::analyze(asset),
        AnalysisType::Binary => binary::analyze(asset),
        AnalysisType::DockerImage => docker_image::analyze(asset),
        AnalysisType::Undetected => undetected::analyze(asset),
    }
}
