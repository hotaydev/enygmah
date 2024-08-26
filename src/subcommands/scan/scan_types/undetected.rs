use crate::helpers::{
    analysis_type::{get_analysis_text, get_analysis_type, AnalysisType},
    logger::{self, EnygmahLogType},
};
use dialoguer::Select;

use super::{binary, docker_image, local_repo, mobile_app, remote_repo, web_application};

// TODO: ask the user for the asset type
pub async fn analyze(asset: &String) {
    let items = vec![
        get_analysis_text(AnalysisType::WebApp),
        get_analysis_text(AnalysisType::MobileApp),
        get_analysis_text(AnalysisType::Binary),
        get_analysis_text(AnalysisType::DockerImage),
        get_analysis_text(AnalysisType::SourceCode),
        get_analysis_text(AnalysisType::RemoteRepository),
    ];

    let selection = Select::new()
        .with_prompt(logger::create_log_text(
            "Select the asset you want to analyze",
            EnygmahLogType::Ask,
        ))
        .items(&items)
        .interact()
        .unwrap();

    logger::create_log(
        &format!("Choosed analysis type is: {}", items[selection]),
        EnygmahLogType::Info,
    );

    match get_analysis_type(&items[selection]) {
        AnalysisType::WebApp => web_application::analyze(asset).await,
        AnalysisType::MobileApp => mobile_app::analyze(asset).await,
        AnalysisType::SourceCode => local_repo::analyze(asset).await,
        AnalysisType::RemoteRepository => remote_repo::analyze(asset).await,
        AnalysisType::Binary => binary::analyze(asset).await,
        AnalysisType::DockerImage => docker_image::analyze(asset).await,
        AnalysisType::Undetected => {}
    }
}
