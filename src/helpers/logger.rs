use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use colored::Colorize;

pub enum EnygmahLogType {
    Warn,
    Info,
    Success,
    Error,
    Ask,
    MainStep,
    Loading,
}

pub fn create_log(text: &str, level: EnygmahLogType) {
    println!("{}", create_log_text(text, level));
}

pub fn create_loading_log(text: String, loading: Arc<AtomicBool>) -> thread::JoinHandle<()> {
    print!("{}\n", "\x1B[?25l"); // Hide the cursor for the loading animation
    std::io::stdout().flush().unwrap();

    let handle = thread::spawn(move || {
        let loading_steps = vec![".", "..", "..."];
        let delay = Duration::from_millis(500);

        while loading.load(Ordering::SeqCst) {
            for step in &loading_steps {
                if !loading.load(Ordering::SeqCst) {
                    break;
                }
                let pretty_message = format!("{}{}", text.clone(), step);
                print!(
                    "\r{}    ",
                    create_log_text(&pretty_message, EnygmahLogType::Loading)
                );
                std::io::stdout().flush().unwrap();
                thread::sleep(delay);
            }
        }

        // Clear the line after stopping
        print!("\r    \r");
        std::io::stdout().flush().unwrap();
    });

    handle
}

pub fn create_log_text(text: &str, level: EnygmahLogType) -> String {
    let level_text = match level {
        EnygmahLogType::Info => " INFO ".on_cyan().white().bold(),
        EnygmahLogType::Warn => " WARN ".on_yellow().white().bold(),
        EnygmahLogType::Success => " SUCCESS ".on_green().white().bold(),
        EnygmahLogType::MainStep => " MAIN ".on_green().white().bold(),
        EnygmahLogType::Loading => " LOADING ".on_green().white().bold(),
        EnygmahLogType::Error => " ERROR ".on_red().white().bold(),
        EnygmahLogType::Ask => " Ask? ".on_purple().white().bold(),
    };
    format!("{} {}", level_text, text)
}
