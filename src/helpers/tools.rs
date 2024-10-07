pub enum Tools {
    // https://github.com/aquasecurity/trivy
    TrivyFs,

    // https://github.com/aquasecurity/trivy
    TrivyDocker,

    // https://github.com/SonarSource/sonarqube
    Sonarqube,

    // https://github.com/google/osv-scanner
    OsvScanner,

    // https://github.com/securego/gosec
    GoSec,

    // https://github.com/docker/docker-bench-security
    // DockerBenchSecurity,

    // https://github.com/docker/scout-cli
    // DockerScout, // Need to authenticate, so we'll use it later

    // https://github.com/snyk/cli
    // Snyk, // Need to authenticate, so we'll use it later (also it doesn't work well in Alpine Linux)

    // https://github.com/wpscanteam/wpscan
    WpScan,

    // https://github.com/zaproxy/zaproxy
    OwaspZapProxy,

    // https://github.com/sullo/nikto
    Nikto,

    // https://github.com/projectdiscovery/nuclei
    Nuclei,

    // https://github.com/semgrep/semgrep
    Semgrep,

    // https://github.com/spotbugs/spotbugs
    SpotBugs,

    // https://github.com/MobSF/Mobile-Security-Framework-MobSF
    // MobSF,

    // https://github.com/google/oss-fuzz
    // OSS-Fuzz

    // https://github.com/deepfence/SecretScanner
    // SecretScanner

    // https://github.com/deepfence/ThreatMapper
    // ThreatMapper

    // https://wapiti-scanner.github.io/
    Wapiti,

    // https://github.com/inspec/inspec
    // Inspec, // Used for writing custom rules for Docker Images

    // https://github.com/aquasecurity/kube-bench
    // Kubebench (Docker)

    // https://github.com/microsoft/binskim
    // binskim (Binary)

    // https://github.com/anchore/grype
    // grype (Docker)

    // https://github.com/quay/clair
    // Clair (Docker)

    // https://github.com/radareorg/radare2
    // radare2 // (see how to use it for binary analysis)

    // https://github.com/danmar/cppcheck
    CppCheck,
}
