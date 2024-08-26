use crate::{
    helpers::{
        analysis_type::{get_analysis_text, get_analysis_type, AnalysisType},
        logger::{self, EnygmahLogType},
    },
    subcommands::scan::analyze::analyze_asset_based_on_type,
};
use dialoguer::Select;

// TODO: ask the user for the asset type
pub fn analyze(asset: &String) {
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

    analyze_asset_based_on_type(asset, get_analysis_type(&items[selection]));
}
