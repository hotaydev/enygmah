use crate::helpers::analysis_type;
use crate::helpers::analysis_type::AnalysisType;
use crate::helpers::install_tools;

use super::assets;

pub async fn analyze(asset: &String) {
    install_tools::install_tools().await;
    let analyze = analysis_type::detect_analysis_type(asset).await;
    analysis_type::log_analysis_type(&analyze);

    analyze_asset_based_on_type(asset, analyze);
}

fn analyze_asset_based_on_type(asset: &String, analysis_type: AnalysisType) {
    match analysis_type {
        AnalysisType::WebApp => assets::web_application::analyze(asset),
        AnalysisType::MobileApp => assets::mobile_app::analyze(asset),
        AnalysisType::SourceCode => assets::local_repo::analyze(asset),
        AnalysisType::RemoteRepository => assets::remote_repo::analyze(asset),
        AnalysisType::Binary => assets::binary::analyze(asset),
        AnalysisType::DockerImage => assets::docker_image::analyze(asset),
        AnalysisType::Undetected => assets::undetected::analyze(asset),
    }
}
