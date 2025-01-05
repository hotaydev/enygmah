<div align=center>
  <img align="center" src="./.github/images/icon.png" alt="enygmah Logo" width="200" />
</div>
<h1 align="center">enygmah</h1>
<h4 align="center">The only tool your project needs to ensure security and quality.<br/>Open-source and free.</h4>

<div align="center">
    <a href="https://github.com/hotaydev/enygmah/">Home Page</a> |
    <a href="https://github.com/hotaydev/enygmah/wiki">Documentation</a> |
    <a href="https://github.com/hotaydev/enygmah/discussions">Discussions</a><br/>
    <!-- <a href="https://crowdfunding.lfx.linuxfoundation.org/projects/34506417-17a0-4cc1-a54b-2563d098d860">Crowdfunding - The Linux Foundation</a> -->
</div>
<br></br>

<div align="center">
  <a href="https://github.com/sponsors/hotaydev" alt="GitHub Sponsors"><img src="https://img.shields.io/github/sponsors/hotaydev?color=%23DB61A2"/></a>
  <a href="https://github.com/hotaydev/enygmah/issues" alt="GitHub Issues"><img src="https://img.shields.io/github/issues/hotaydev/enygmah"/></a>
  <a href="https://github.com/hotaydev/enygmah/pulls" alt="GitHub pull requests"><img src="https://img.shields.io/github/issues-pr/hotaydev/enygmah"/></a>
  <a href="https://github.com/hotaydev/enygmah/graphs/contributors" alt="GitHub contributors"><img src="https://img.shields.io/github/contributors-anon/hotaydev/enygmah?color=%230594c6"/></a>
  <!-- <a href="https://www.bestpractices.dev/projects/xxxx"><img src="https://www.bestpractices.dev/projects/xxxx/badge"></a> -->
</div>

---

## Table of Contents

* [<g-emoji class="g-emoji" alias="thinking" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f914.png">🤔</g-emoji> Why enygmah?](#🤔-why-enygmah)
* [<g-emoji class="g-emoji" alias="eyes" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f440.png">👀</g-emoji> How can I use it?](#👀-how-can-i-use-it)
* [<g-emoji class="g-emoji" alias="books" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f4da.png">📚</g-emoji> Learn more](#📚-learn-more)
* [🫶 Support enygmah Development](#🫶-support-enygmah-development)
* [<g-emoji class="g-emoji" alias="bulb" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f4a1.png">💡</g-emoji> Feature requests](#💡-feature-requests)
* [<g-emoji class="g-emoji" alias="star2" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f31f.png">🌟</g-emoji> Contributing to enygmah](#🌟-contributing-to-enygmah)
* [<g-emoji class="g-emoji" alias="sparkles" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/2728.png">✨</g-emoji> Inspiration](#✨-inspiration)
* [<g-emoji class="g-emoji" alias="pray" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f64f.png">🙏</g-emoji> Thank You](#🙏-thank-you)


## 🤔 Why enygmah?

[enygmah](https://github.com/hotaydev/enygmah/) is a [Free and Open Source Software](https://www.gnu.org/philosophy/floss-and-foss.html) (FOSS), created to streamline and **simplify security and code quality processes** through static code analyses (SAST) and dynamic code analyses (DAST). Its goal is to be **user-friendly** and **easy to use** even by those who do not want to be an expert in security.

Enygmah was created to consolidate many processes into a single solution. It offers:

* **Local source security and quality analysis**
* **Remote source security and quality analysis**
* **Web Applications security analysis**
* **Docker images security analysis**
* **Mobile app analysis** (*in development*)
* **Binaries analysis** (*in development*)

And the best part? **It's completely free and open source!**

<a href="https://github.com/hotaydev/enygmah/releases/latest/">
  <img src="https://img.shields.io/badge/Download_enygmah-black?labelColor=black&style=for-the-badge" align="right" alt="Download enygmah"/>
</a>

## 👀 How can I use it?

To start using enygmah, simply go to our [releases](https://github.com/hotaydev/enygmah/releases/latest) page and download the latest version of enygmah (we offer different versions for Linux (or Windows WSL), macOS).

<sub>NOTE: This tool is strictly integrated into Docker, so to use it you need to have Docker installed. For Windows machines, make sure to execute it from inside the WSL.</sub>
<sub>You can use it as a standalone application, running it right from the binary/executable folder, or you can add it to the path of your system.</sub>

<!-- TODO: here we can add a script to quick installation, like the nodesource do with NodeJS -->
---

That's it! You can now start using the enygmah to ensure the security and quality of your projects! We are sure that it will be really helpful to you 🎉

### Available commands

To see the full list of commands you can run `enygmah --help`.

We currently have two main commands:

* `enygmah scan <target>`: Scans a folder, repository (remote or local), a web application or a docker image (it depend's on what you pass as the `<target>` parameter).
  * Example: `enygmah scan .` (current folder)
  * Example: `enygmah scan https://github.com/hotaydev/enygmah.git` (remote repository)
  * Example: `enygmah scan deian` (Docker image)
* `enygmah install`: It downloads all the nedded Docker images. If not runned, it will automatically download the images when running `enygmah scan`.


## 📚 Learn more

* Documentation: [Wiki](https://github.com/hotaydev/enygmah/wiki)
* Discussions: [Github Discussions](https://github.com/hotaydev/enygmah/discussions)
  * [Questions and Answers](https://github.com/hotaydev/enygmah/discussions/categories/q-a)
  * [Announcements](https://github.com/hotaydev/enygmah/discussions/categories/announcements)
  * [Ideas](https://github.com/hotaydev/enygmah/discussions/categories/ideas)

## 🫶 Support enygmah Development

If you find enygmah useful and want to help us keep the project growing, please consider supporting our project with [Github Sponsors](https://github.com/sponsors/hotaydev). Your support shows our contributors that their efforts are appreciated and motivates them to continue their excellent work. Every contribution, no matter how small, helps us keep improving enygmah.

## 💡 Feature requests

We value your input on improving enygmah and making it more useful for you. If you have any ideas or feature requests, please share them in the [enygmah discussions: Ideas](https://github.com/hotaydev/enygmah/discussions/categories/ideas) section or by [opening an Issue](https://github.com/hotaydev/enygmah/issues/new) here on Github.

Your feedback helps us understand our users' needs and prioritize the features that matter most to you. We appreciate your time and effort in sharing your thoughts with us.

We appreciate your support, and we look forward to hearing your ideas!

Also, if you've liked someone's Feature request, upvote it! It helps us prioritize our work 😉

## 🌟 Contributing to enygmah

To start contributing to enygmah, please read [CONTRIBUTING.md](CONTRIBUTING.md).
There are ways to contribute [with code](https://github.com/hotaydev/enygmah/blob/main/CONTRIBUTING.md#code-contributions) and [without code](https://github.com/enygmah/enygmah/blob/main/CONTRIBUTING.md#how-can-i-help). We welcome all contributions, big or small, and we appreciate your time and effort in helping us improve enygmah. We look forward to your contributions 🚀

### 🛠️ Setting Up a Development Environment

To set up a development environment for the enygmah cli tool, you just need to have [Rust](https://www.rust-lang.org/) installed. The environment is the same for macOS/Linux and for Windows (but for Windows it will require WSL, since we interact a lot with Docker).

To test it locally against enygmah itself, you can run it like this: `cargo run -- scan .`

## ✨ Inspiration

enygmah is inspired by several unique tools and projects, including [Snyk](https://snyk.io/), [Sonarqube](https://www.sonarsource.com/products/sonarqube/), [Trivy](https://trivy.dev/) and [ZAP Proxy](https://www.zaproxy.org/).

We owe a huge debt of gratitude to the developers and creators of these projects, and we hope that enygmah can continue to build on their innovative ideas and make them accessible to a broader audience.

Thank you to all those who inspire us, and we look forward to seeing what the enygmah community will create with this tool!

### enygmah is also made possible by the following technologies:
* [Rust](https://www.rust-lang.org/) - The base of our CLI
* [clap](https://github.com/clap-rs/clap) - CLI Framework for Rust
* [bollard](https://github.com/fussybeaver/bollard) - Rust bindings for Docker interaction

### And uses under the wheel these tools ✅:
* [Trivy](https://github.com/aquasecurity/trivy) - Detect secrets, code vulnerabilities and vulnerable dependencies locally and in Docker images.
* [Sonarqube](https://github.com/SonarSource/sonarqube) - Detect Code issues and Hotspots for possible vulnerable code.
* [OsvScanner](https://github.com/google/osv-scanner) - Detect vulnerable dependencies.
* [GoSec](https://github.com/securego/gosec) - Scan for vulnerabilities in Go Lang.
* [WpScan](https://github.com/wpscanteam/wpscan) - Scan for vulnerabilities in Wordpress.
* [OwaspZapProxy](https://github.com/zaproxy/zaproxy) - Dinamically scan for vulnerabilities in web applications.
* [Nikto](https://github.com/sullo/nikto) - Dinamically scan for vulnerabilities in web applications.
* [Nuclei](https://github.com/projectdiscovery/nuclei) - Dinamically scan for vulnerabilities in web applications.
* [Semgrep](https://github.com/semgrep/semgrep) - Scan for code vulnerabilities and vulnerable dependencies in static code.
* [SpotBugs](https://github.com/spotbugs/spotbugs) - Scan for code vulnerabilities and vulnerable dependencies in Java static code.
* [Grype](https://github.com/anchore/grype) - Scan for vulnerabilities in Docker images.
* [CppCheck](https://github.com/danmar/cppcheck) - Scan for code vulnerabilities and vulnerable dependencies in static C/C++ code.

### Tool that we'll implement soon 🛠️:
* [Docker Bench Security](https://github.com/docker/docker-bench-security) - Benchmark Docker Security.
* [Docker Scout](https://github.com/docker/scout-cli) - Scan for vulnerabilities in Docker Images.
* [Snyk](https://github.com/snyk/cli) - Scan for code vulnerabilities, vulnerable dependencies and vulnerabilities in Docker images.
* [MobSF](https://github.com/MobSF/Mobile-Security-Framework-MobSF) - Search for vulnerabilities in Mobile Apps
* [OSS-Fuzz](https://github.com/google/oss-fuzz) - Fuzzer for web applications.
* [Secret Scanner](https://github.com/deepfence/SecretScanner) - Scan for secrets in Containers and File Systems.
* [Threat Mapper](https://github.com/deepfence/ThreatMapper) - Threat Management and Path Enumeration for Cloud Environments.
* [Wapiti](https://wapiti-scanner.github.io/) - Waiting to be stable with python 3.13 to add to this tool.
* [Inspec](https://github.com/inspec/inspec) - Vulnerability analisis in infrastructure.
* [Kubebench](https://github.com/aquasecurity/kube-bench) - Benchmark Kubernetes Security.
* [binskim](https://github.com/microsoft/binskim) - Binary static analysis tool
* [Clair](https://github.com/quay/clair) - Docker Security Scanner.
* [radare2](https://github.com/radareorg/radare2) - Binary static analysis tool

# 🙏 Thank You

We want to express our sincere gratitude to our **[Github Sponsors](https://github.com/sponsors/hotaydev)** and the **[contributors](https://github.com/hotaydev/enygmah/graphs/contributors)** of the project. Your support and contributions allow us to continue developing and improving enygmah. Thank you for being a part of our community and helping us make enygmah the best it can be!

<!-- Leave commented till have more contributors -->
<!--
## 🌟 Contributors

<p align="center">
    <a href="https://github.com/hotaydev/enygmah/graphs/contributors">
        <img src="https://contrib.rocks/image?repo=hotaydev/enygmah&max=300&columns=14" width="600"/></a>
</p>
-->
