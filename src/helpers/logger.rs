use colored::Colorize;

pub enum EnygmahLogType {
    Warn,
    Info,
    Success,
    Error,
}

pub fn create_log(text: &str, level: EnygmahLogType) {
    let level_text = match level {
        EnygmahLogType::Info => " INFO ".on_cyan().white().bold(),
        EnygmahLogType::Warn => " WARN ".on_yellow().white().bold(),
        EnygmahLogType::Success => " SUCCESS ".on_green().white().bold(),
        EnygmahLogType::Error => " ERROR ".on_red().white().bold(),
    };
    println!("{} {}", level_text, text);
}
