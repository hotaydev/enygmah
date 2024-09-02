use colored::Colorize;

pub enum EnygmahLogType {
    Warn,
    Info,
    Success,
    Error,
    Ask,
    MainStep,
}

pub fn create_log(text: &str, level: EnygmahLogType) {
    println!("{}", create_log_text(text, level));
}

pub fn create_log_text(text: &str, level: EnygmahLogType) -> String {
    let level_text = match level {
        EnygmahLogType::Info => " INFO ".on_cyan().white().bold(),
        EnygmahLogType::Warn => " WARN ".on_yellow().white().bold(),
        EnygmahLogType::Success => " SUCCESS ".on_green().white().bold(),
        EnygmahLogType::MainStep => " MAIN ".on_green().white().bold(),
        EnygmahLogType::Error => " ERROR ".on_red().white().bold(),
        EnygmahLogType::Ask => " Ask? ".on_purple().white().bold(),
    };
    format!("{} {}", level_text, text)
}
