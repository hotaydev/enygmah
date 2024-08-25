use crate::helpers::analysis_type;
use crate::helpers::install_tools;

pub async fn analyze(asset: &String) {
    install_tools::install_tools().await;
    let analyze = analysis_type::detect_analysis_type(asset).await;
    analysis_type::log_analysis_type(&analyze);
}
