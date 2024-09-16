use crate::helpers::logger;
use log::debug;

pub async fn analyze(path: &String) {
    debug!("{}", path);
    logger::create_log(
        "We are currently only analyzing repositories (local and remote) and web applications. This analysis will be implemente soon.",
        logger::EnygmahLogType::Warn,
    );
    logger::create_log(
        "To be up-to-date with improvements on this tool, check https://github.com/hotaydev/enygmah",
        logger::EnygmahLogType::Info,
    );
}
