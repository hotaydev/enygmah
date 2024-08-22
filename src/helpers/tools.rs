pub enum Tools {
    /// https://github.com/aquasecurity/trivy
    Trivy,

    /// https://github.com/SonarSource/sonarqube
    // Sonarqube,

    /// https://github.com/google/osv-scanner
    OsvScanner,

    /// https://github.com/securego/gosec
    GoSec,

    /// https://github.com/docker/docker-bench-security
    // DockerBenchSecurity,

    /// https://github.com/snyk/cli
    // Snyk, // Need to authenticate, so we'll use it later (also it doesn't work weel in Alpine Linux)

    /// https://github.com/wpscanteam/wpscan
    WpScan,

    /// https://github.com/zaproxy/zaproxy
    OwaspZapProxy,

    /// https://github.com/sullo/nikto
    // Nikto,

    /// https://github.com/projectdiscovery/nuclei
    // Nuclei,

    /// https://github.com/semgrep/semgrep
    Semgrep,

    /// https://github.com/spotbugs/spotbugs
    SpotBugs,

    /// https://github.com/MobSF/Mobile-Security-Framework-MobSF
    // MobSF,

    /// https://github.com/splintchecker/splint
    // Splint, // Some problems installing it into Docker Container

    /// https://github.com/danmar/cppcheck
    CppCheck,
}
