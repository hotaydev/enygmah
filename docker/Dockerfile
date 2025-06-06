FROM alpine:3.21@sha256:56fa17d2a7e7f168a043a2712e63aed1f8543aeafdcee47c58dcffe38ed51099

LABEL maintainer="Taylor Hoffmann <taylor@hotay.dev>"
LABEL version="1.0.1"
LABEL description="Automate vulnerability analysis within several tools"
LABEL license="AGPL-3.0"
LABEL tags="security, vulnerability, analysis"

WORKDIR /home/enygmah

############################# Installing needed tools for the Security tools we'll use #######################

# Needs NodeJS for sonar-scanner analysis on NodeJS projects
RUN apk update && \
    apk add --no-cache gcc g++ make git wget jq ruby ruby-dev openjdk21 python3 py3-pip python3-dev bash curl libc6-compat openssl perl perl-net-ssleay libffi-dev libressl-dev bsd-compat-headers docker nodejs npm cmake

## Upgrading RubyGems and pip
RUN gem update --system 3.6.3 --no-document && \
    python3 -m pip install --upgrade pip --break-system-packages

## Installing Rust and Cargo (used for Wapiti)
## Wapiti can't be used right now, it's too unstable with latest python versions
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

## Installing Go Lang
RUN export packageArch=$(if [ "$(apk --print-arch)" == "aarch64" ]; then echo "arm64"; else echo "amd64"; fi) && wget --secure-protocol=TLSv1_2 --max-redirect=1 "https://go.dev/dl/go1.23.5.linux-${packageArch}.tar.gz" && \
    tar -C /usr/local -xzf ./go*.linux-*.tar.gz && \
    rm -rf ./go*.linux-*.tar.gz && \
    echo 'export PATH="$PATH:/usr/local/go/bin:/root/go/bin:/usr/local/bin/sonar-scanner/bin:/root/.cargo/env"' >> /etc/profile && \
    PATH="/usr/local/go/bin:/root/go/bin:/usr/local/bin/sonar-scanner/bin:/root/.cargo/env:${PATH}"

ENV PATH="/usr/local/go/bin:/root/go/bin:/usr/local/bin/sonar-scanner/bin:/root/.cargo/env:${PATH}"

##############################################################################################################


################################### Installing Security tools we'll use ######################################

# Install Trivy:
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 -O- -q https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin v0.58.2

## Installing OSV-Scanner
RUN /usr/local/go/bin/go install github.com/google/osv-scanner/cmd/osv-scanner@v1

## Install GoSec
RUN /usr/local/go/bin/go install github.com/securego/gosec/v2/cmd/gosec@latest

## Install Nuclei
RUN /usr/local/go/bin/go install github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest

## Install Nikto
RUN git clone --depth=1 https://github.com/sullo/nikto.git /usr/local/bin/nikto

## Install WpScan
RUN gem install wpscan --no-document

## Install CppCheck (build it, since the APK registry version doesn't have serif export - it's too old)
RUN cd /tmp/ && \
    git clone https://github.com/danmar/cppcheck.git && \
    cd cppcheck && git fetch --tags && git checkout $(git describe --tags `git rev-list --tags --max-count=1`) && \
    mkdir build && cd build && cmake .. && make -j$(nproc) && make install && \
    rm -rf /tmp/cppcheck

## Install SpotBugs
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 https://github.com/spotbugs/spotbugs/releases/download/4.9.0/spotbugs-4.9.0.tgz && \
    tar -xzf spotbugs-*.tgz && rm -rf spotbugs-*.tgz && mv ./spotbugs-* /usr/local/bin/spotbugs-src

## Install sonar-scanner
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 https://binaries.sonarsource.com/Distribution/sonar-scanner-cli/sonar-scanner-cli-6.2.1.4610.zip && \
    unzip sonar-scanner-cli-*.zip && \
    rm sonar-scanner-cli-*.zip && \
    mv sonar-scanner-* /usr/local/bin/sonar-scanner

## Install Semgrep
RUN python3 -m pip install semgrep --break-system-packages

## Install Wapiti
## Wapiti can't be used right now, it's too unstable with latest python versions
# RUN . /root/.cargo/env && \
#     pip install cryptography --global-option=build_ext --global-option="-L/usr/local/opt/openssl/lib" --global-option="-I/usr/local/opt/openssl/include" --break-system-packages && \
#     python3 -m pip install wapiti3 --break-system-packages

## Install Owasp Zap Proxy
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 https://github.com/zaproxy/zaproxy/releases/download/v2.16.0/ZAP_2.16.0_Crossplatform.zip && \
    unzip ZAP_*_Crossplatform.zip && \
    rm -rf ZAP_*_Crossplatform.zip && \
    mv ZAP_* /usr/local/bin/ZAP-proxy

## Install Docker Scout CLI (not actually working since it needs Docker authentication)
RUN mkdir /root/.docker/ && \
    curl -sSfL https://raw.githubusercontent.com/docker/scout-cli/main/install.sh | sh -s --

## Install Grype
RUN curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b /usr/local/bin

## Copy results aggregator to Docker image
COPY ./join-results.sh /usr/local/bin/join-results
RUN chmod +x /usr/local/bin/join-results

##############################################################################################################

###################################### Cleanup cache and garbage #############################################

RUN /usr/local/go/bin/go clean && \
    rm -rf /root/.cache && \
    gem sources -c

##############################################################################################################


ENTRYPOINT ["tail", "-f", "/dev/null"]
