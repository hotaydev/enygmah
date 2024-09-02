use bollard::Docker;

use super::{enygmah_docker, sonarqube, tools::Tools};

pub async fn run_scan(tool: Tools, asset: &str, docker: &Docker) {
    match tool {
        Tools::Trivy => trivy(asset, docker).await,
        Tools::Sonarqube => sonarqube(asset, docker).await,
        Tools::Semgrep => semgrep(asset, docker).await,
        Tools::OsvScanner => osv_scanner(asset, docker).await,
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

async fn trivy(asset: &str, docker: &Docker) {
    enygmah_docker::execute_command(
        docker,
        format!("trivy fs --scanners vuln,misconfig,secret -f json -o /home/enygmah/_outputs/trivy.json {}", asset),
    ).await;
}

async fn osv_scanner(asset: &str, docker: &Docker) {
    enygmah_docker::execute_command(
        docker,
        format!(
            "osv-scanner scan --format json --output /home/enygmah/_outputs/osv-scanner.json {}",
            asset
        ),
    )
    .await;
}

// TODO: see a way to allow users to do `semgrep login`, being able to run more advanced scans.
async fn semgrep(asset: &str, docker: &Docker) {
    enygmah_docker::execute_command(
        docker,
        format!(
            "semgrep scan --json --output /home/enygmah/_outputs/semgrep.json {}",
            asset
        ),
    )
    .await;
}

async fn sonarqube(asset: &str, docker: &Docker) {
    sonarqube::start(docker, asset).await;
}
