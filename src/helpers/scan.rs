use bollard::Docker;
use indicatif::{ProgressBar, ProgressStyle};

use super::{enygmah_docker, logger, sonarqube, tools::Tools};

pub async fn run_scan(tool: Tools, asset: &str, docker: &Docker, pb: &ProgressBar) {
    match tool {
        Tools::Trivy => trivy(asset, docker, pb).await,
        Tools::Sonarqube => sonarqube(asset, docker, pb).await,
        Tools::Semgrep => semgrep(asset, docker, pb).await,
        Tools::OsvScanner => osv_scanner(asset, docker, pb).await,
        Tools::CppCheck => cppcheck(asset, docker, pb).await,
        Tools::GoSec => gosec(asset, docker, pb).await,
        Tools::SpotBugs => spotbugs(asset, docker, pb).await,
        // Tools::WpScan => println!("{}", asset),
        // Tools::OwaspZapProxy => println!("{}", asset),

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

async fn cppcheck(asset: &str, docker: &Docker, pb: &ProgressBar) {
    pb.set_message("CppCheck   | Scanning...");

    // Execute CppCheck analysis
    enygmah_docker::execute_command(
        docker,
        format!(
            "cppcheck --enable=all --suppress=missingIncludeSystem --inconclusive --error-exitcode=0 --xml --xml-version=2 --quiet {} 2> /home/enygmah/_outputs/cppcheck.xml",
            asset
        ),
    )
    .await;

    // Convert the CppCheck XML report to JSON
    enygmah_docker::execute_command(
        docker,
        String::from("yq -p=xml -o=json /home/enygmah/_outputs/cppcheck.xml > /home/enygmah/_outputs/cppcheck.json"),
    )
    .await;

    // Remove the CppCheck XML report
    enygmah_docker::execute_command(
        docker,
        String::from("rm -f /home/enygmah/_outputs/cppcheck.xml"),
    )
    .await;

    // Remove the CppCheck JSON report if it's empty due to a lack of .c/.cpp files
    enygmah_docker::execute_command(
        docker,
        String::from("if [ `cat /home/enygmah/_outputs/cppcheck.json` = \"null\" ]; then rm -f /home/enygmah/_outputs/cppcheck.json; fi"),
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
            "java -jar /usr/local/bin/spotbugs-src/lib/spotbugs.jar -textui -low -progress -exitcode=0 -xml=/home/enygmah/_outputs/spotbugs.xml {}",
            asset
        ),
    )
    .await;

    // Convert the SpotBugs XML report to JSON
    enygmah_docker::execute_command(
        docker,
        String::from("yq -p=xml -o=json /home/enygmah/_outputs/spotbugs.xml > /home/enygmah/_outputs/spotbugs.json"),
    )
    .await;

    // Remove the SpotBugs XML report
    enygmah_docker::execute_command(
        docker,
        String::from("rm -f /home/enygmah/_outputs/spotbugs.xml"),
    )
    .await;

    // Remove the SpotBugs JSON report if it's empty due to the project being in other langages than Java
    enygmah_docker::execute_command(
        docker,
        String::from("if [ `cat /home/enygmah/_outputs/spotbugs.json` = \"null\" ]; then rm -f /home/enygmah/_outputs/spotbugs.json; fi"),
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
            "gosec -fmt=json -no-fail -nosec -show-ignored -out=/home/enygmah/_outputs/gosec.json {}/...",
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

    // TODO: To get the final results we can use the following API requests:
    // curl -u <user>:<pass> -X GET "http://localhost:9000/api/hotspots/search?project=enygmah&ps=500&p=1&status=TO_REVIEW,REVIEWED"
    // curl -u <user>:<pass> -X GET "http://localhost:9000/api/issues/search?components=enygmah&ps=500&p=1&severities=INFO,MINOR,MAJOR,CRITICAL,BLOCKER&statuses=OPEN,CONFIRMED&types=CODE_SMELL,BUG,VULNERABILITY"
}
