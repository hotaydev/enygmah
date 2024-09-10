use bollard::Docker;
use indicatif::{ProgressBar, ProgressStyle};

use super::{enygmah_docker, logger, sonarqube, tools::Tools};

pub async fn run_scan(tool: Tools, asset: &str, docker: &Docker, pb: &ProgressBar) {
    match tool {
        Tools::Trivy => trivy(asset, docker, pb).await,
        Tools::Sonarqube => sonarqube(asset, docker, pb).await,
        Tools::Semgrep => semgrep(asset, docker, pb).await,
        Tools::OsvScanner => osv_scanner(asset, docker, pb).await,
        // Tools::GoSec => println!("{}", asset),
        // Tools::WpScan => println!("{}", asset),
        // Tools::OwaspZapProxy => println!("{}", asset),
        // Tools::SpotBugs => println!("{}", asset),
        // Tools::CppCheck => println!("{}", asset),

        // Tools::MobSF => println!("{}", asset),
        // Tools::Nikto => println!("{}", asset),
        // Tools::Nuclei => println!("{}", asset),
        // Tools::DockerBenchSecurity => println!("{}", asset),
        // Tools::DockerScout => println!("{}", asset),
        // Tools::Snyk => println!("{}", asset),
    }
}

async fn trivy(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Trivy      | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!("trivy fs --scanners vuln,misconfig,secret -f json -o /home/enygmah/_outputs/trivy.json {}", asset),
    ).await;
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◆"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    pb.finish_with_message(logger::create_log_text(
        "Trivy",
        logger::EnygmahLogType::Success,
    ));
}

async fn osv_scanner(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("OsvScanner | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "osv-scanner scan --format json --output /home/enygmah/_outputs/osv-scanner.json {}",
            asset
        ),
    )
    .await;
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◆"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    pb.finish_with_message(logger::create_log_text(
        "OsvScanner",
        logger::EnygmahLogType::Success,
    ));
}

// TODO: see a way to allow users to do `semgrep login`, being able to run more advanced scans.
async fn semgrep(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Semgrep    | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "semgrep scan --json --output /home/enygmah/_outputs/semgrep.json {}",
            asset
        ),
    )
    .await;
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◆"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    pb.finish_with_message(logger::create_log_text(
        "Semgrep",
        logger::EnygmahLogType::Success,
    ));
}

async fn sonarqube(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Sonarqube  | Pulling docker image...");
    sonarqube::download_sonarqube_if_needed(docker).await;

    pb.set_message("Sonarqube  | Scanning...");
    sonarqube::start(docker, asset).await;
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◆"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    pb.finish_with_message(logger::create_log_text(
        "Sonarqube",
        logger::EnygmahLogType::Success,
    ));

    // To get the final results we can use the following API requests:
    // curl -u <user>:<pass> -X GET "http://localhost:9000/api/hotspots/search?project=enygmah&ps=500&p=1&status=TO_REVIEW,REVIEWED"
    // curl -u <user>:<pass> -X GET "http://localhost:9000/api/issues/search?components=enygmah&ps=500&p=1&severities=INFO,MINOR,MAJOR,CRITICAL,BLOCKER&statuses=OPEN,CONFIRMED&types=CODE_SMELL,BUG,VULNERABILITY"
}
