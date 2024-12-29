use bollard::Docker;
use indicatif::{ProgressBar, ProgressStyle};

use super::{enygmah_docker, logger, sonarqube, tools::Tools};

pub async fn run_scan(tool: Tools, asset: &str, docker: &Docker, pb: &ProgressBar) {
    match tool {
        Tools::TrivyFs => trivy_filesystem(asset, docker, pb).await,
        Tools::TrivyDocker => trivy_docker(asset, docker, pb).await,
        Tools::Sonarqube => sonarqube(asset, docker, pb).await,
        Tools::Semgrep => semgrep(asset, docker, pb).await,
        Tools::OsvScanner => osv_scanner(asset, docker, pb).await,
        Tools::CppCheck => cppcheck(asset, docker, pb).await,
        Tools::GoSec => gosec(asset, docker, pb).await,
        Tools::SpotBugs => spotbugs(asset, docker, pb).await,
        Tools::WpScan => wpscan(asset, docker, pb).await,
        Tools::OwaspZapProxy => owaspzapproxy(asset, docker, pb).await,
        Tools::Nikto => nikto(asset, docker, pb).await,
        Tools::Nuclei => nuclei(asset, docker, pb).await,
        // Tools::Wapiti => wapiti(asset, docker, pb).await, // Wapiti can't be used right now, it's too unstable with latest python versions
        Tools::Grype => grype(asset, docker, pb).await,
        // Tools::MobSF => println!("{}", asset),
        // Tools::DockerBenchSecurity => println!("{}", asset),
        // Tools::DockerScout => println!("{}", asset),
        // Tools::Snyk => println!("{}", asset),
    }
}

async fn trivy_filesystem(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Trivy      | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!("trivy fs --scanners vuln,misconfig,secret --exit-code=0 -f sarif --severity=UNKNOWN,LOW,MEDIUM,HIGH,CRITICAL -o /home/enygmah/_outputs/trivy-fs.sarif {}", asset),
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

async fn trivy_docker(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Trivy       | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!("trivy image --format=sarif --output=/home/enygmah/_outputs/trivy-docker.sarif --license-full --exit-code=0 --severity=UNKNOWN,LOW,MEDIUM,HIGH,CRITICAL {}", asset),
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
            "osv-scanner scan --format sarif --output /home/enygmah/_outputs/osv-scanner.sarif {}",
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

async fn cppcheck(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("CppCheck   | Scanning...");

    // Execute CppCheck analysis
    enygmah_docker::execute_command(
        docker,
        format!(
            "cppcheck --enable=all --suppress=missingIncludeSystem --inconclusive --error-exitcode=0 --output-format=sarif --quiet {} 2> /home/enygmah/_outputs/cppcheck.sarif",
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
        "CppCheck",
        logger::EnygmahLogType::Success,
    ));
}

async fn spotbugs(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("SpotBugs   | Scanning...");

    // Execute SpotBugs analysis
    enygmah_docker::execute_command(
        docker,
        format!(
            "java -jar /usr/local/bin/spotbugs-src/lib/spotbugs.jar -textui -low -progress -exitcode=0 -sarif=/home/enygmah/_outputs/spotbugs.sarif {}",
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
        "SpotBugs",
        logger::EnygmahLogType::Success,
    ));
}

async fn gosec(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Gosec      | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "gosec -fmt=sarif -no-fail -nosec -show-ignored -out=/home/enygmah/_outputs/gosec.sarif {}/...",
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
        "Gosec",
        logger::EnygmahLogType::Success,
    ));
}

// TODO: see a way to allow users to do `semgrep login`, being able to run more advanced scans.
async fn semgrep(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Semgrep    | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "semgrep scan --sarif --output /home/enygmah/_outputs/semgrep.sarif {}",
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
}

// TODO: see a way to allow users to use their own WpScan API Key
// TODO: No Sarif Format...
async fn wpscan(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("WpScan     | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "wpscan --format=json --output=/home/enygmah/_outputs/wpscan.json --random-user-agent --clear-cache --update --no-banner --url {}",
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
        "WpScan",
        logger::EnygmahLogType::Success,
    ));
}

// TODO: no Sarif export...
async fn owaspzapproxy(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("ZapProxy   | (It takes a while) Scanning...");
    enygmah_docker::execute_command(docker, String::from("rm -rf /root/.ZAP/")).await;
    enygmah_docker::execute_command(
        docker,
        format!(
            "/usr/local/bin/ZAP-proxy/zap.sh -cmd -quickurl {} -quickout /home/enygmah/_outputs/zap-proxy.json",
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
        "ZapProxy",
        logger::EnygmahLogType::Success,
    ));
}

// TODO: No Sarif export...
async fn nikto(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Nikto      | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!("/usr/local/bin/nikto/program/nikto.pl -followredirects -Format=json -host={} -nointeractive -output=/home/enygmah/_outputs/nikto.json", asset),
    )
    .await;
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◆"])
            .template("{spinner:.green.bold} {msg}")
            .expect("Failed to set spinner template"),
    );
    pb.finish_with_message(logger::create_log_text(
        "Nikto",
        logger::EnygmahLogType::Success,
    ));
}

async fn nuclei(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Nuclei     | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "nuclei -target={} -as -sarif-export=/home/enygmah/_outputs/nuclei.sarif -follow-redirects -max-redirects=2",
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
        "Nuclei",
        logger::EnygmahLogType::Success,
    ));
}

// Wapiti can't be used right now, it's too unstable with latest python versions
// Passing also `--level=2` makes it get more vulnerabilities (possibly), but takes 10x more time.
// TODO: see a way to allow the user to pass an "aggressivity" level
// TODO: No Sarif Export...
// async fn wapiti(asset: &str, docker: &Docker, pb: &ProgressBar) {
//     pb.set_message("Wapiti     | Scanning...");
//     enygmah_docker::execute_command(
//         docker,
//         format!(
//             "wapiti --url={} --scope=domain --flush-session --depth=10 --format=json --output=/home/enygmah/_outputs/wapiti.json",
//             asset
//         ),
//     )
//     .await;
//     pb.set_style(
//         ProgressStyle::default_spinner()
//             .tick_strings(&["◆"])
//             .template("{spinner:.green.bold} {msg}")
//             .expect("Failed to set spinner template"),
//     );
//     pb.finish_with_message(logger::create_log_text(
//         "Wapiti",
//         logger::EnygmahLogType::Success,
//     ));
// }

async fn grype(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("Grype      | Scanning...");
    enygmah_docker::execute_command(
        docker,
        format!(
            "grype {} --output=sarif --show-suppressed --file /home/enygmah/_outputs/grype.sarif",
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
        "Grype",
        logger::EnygmahLogType::Success,
    ));
}
