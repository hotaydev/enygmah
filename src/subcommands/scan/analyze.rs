use crate::helpers::install_tools;

pub async fn analyze(asset: &String) {
    install_tools::install_tools().await;
    println!("{}", asset);
}
