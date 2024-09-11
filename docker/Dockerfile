FROM alpine:3.20@sha256:beefdbd8a1da6d2915566fde36db9db0b524eb737fc57cd1367effd16dc0d06d

LABEL maintainer="Taylor Hoffmann <taylor@hotay.dev>"
LABEL version="1.0.0"
LABEL description="Automate vulnerability analysis within several tools"
LABEL license="AGPL-3.0"
LABEL tags="security, vulnerability, analysis"

WORKDIR /home/enygmah

############################# Installing needed tools for the Security tools we'll use #######################

RUN apk update && \
    apk add --no-cache gcc g++ make git wget jq ruby ruby-dev openjdk21 python3 py3-pip python3-dev bash curl libc6-compat openssl

## Upgrading RubyGems and pip
RUN gem update --system 3.5.18 --no-document && \
    python3 -m pip install --upgrade pip --break-system-packages

## Installing Go Lang
RUN export packageArch=$(if [ "$(apk --print-arch)" == "aarch64" ]; then echo "arm64"; else echo "amd64"; fi) && wget --secure-protocol=TLSv1_2 --max-redirect=1 "https://go.dev/dl/go1.23.1.linux-${packageArch}.tar.gz" && \
    tar -C /usr/local -xzf ./go*.linux-*.tar.gz && \
    rm -rf ./go*.linux-*.tar.gz && \
    echo 'export PATH="$PATH:/usr/local/go/bin:/root/go/bin:/usr/local/bin/sonar-scanner/bin"' >> /etc/profile && \
    PATH="/usr/local/go/bin:/root/go/bin:/usr/local/bin/sonar-scanner/bin:${PATH}"

ENV PATH="/usr/local/go/bin:/root/go/bin:/usr/local/bin/sonar-scanner/bin:${PATH}"

##############################################################################################################


################################### Installing Security tools we'll use ######################################

# Install Trivy:
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 -O- -q https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin v0.55.0

## Installing OSV-Scanner
RUN /usr/local/go/bin/go install github.com/google/osv-scanner/cmd/osv-scanner@v1

## Install GoSec
RUN /usr/local/go/bin/go install github.com/securego/gosec/v2/cmd/gosec@latest

## Install WpScan
RUN gem install wpscan --no-document

## Install CppCheck
RUN apk add --no-cache cppcheck && \
    export packageArch=$(if [ "$(apk --print-arch)" == "aarch64" ]; then echo "arm64"; else echo "amd64"; fi) && \    
    wget "https://github.com/mikefarah/yq/releases/latest/download/yq_linux_${packageArch}" -O /usr/bin/yq && chmod +x /usr/bin/yq

## Install SpotBugs
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 https://github.com/spotbugs/spotbugs/releases/download/4.8.6/spotbugs-4.8.6.tgz && \
    tar -xzf spotbugs-*.tgz && rm -rf spotbugs-*.tgz && mv ./spotbugs-* /usr/local/bin/spotbugs-src && \
    chmod +x /usr/local/bin/spotbugs-src/bin/spotbugs && \
    echo -e "#!/bin/bash\n\nsh /usr/local/bin/spotbugs-src/bin/spotbugs\n" > /usr/local/bin/spotbugs && \
    chmod +x /usr/local/bin/spotbugs

## Install sonar-scanner
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 https://binaries.sonarsource.com/Distribution/sonar-scanner-cli/sonar-scanner-cli-6.1.0.4477.zip && \
    unzip sonar-scanner-cli-*.zip && \
    rm sonar-scanner-cli-*.zip && \
    mv sonar-scanner-* /usr/local/bin/sonar-scanner

## Install Semgrep
RUN python3 -m pip install semgrep --break-system-packages
# Use with `semgrep scan`

## Install Owasp Zap Proxy
RUN wget --secure-protocol=TLSv1_2 --max-redirect=1 https://github.com/zaproxy/zaproxy/releases/download/v2.15.0/ZAP_2.15.0_Crossplatform.zip && \
    unzip ZAP_*_Crossplatform.zip && \
    rm -rf ZAP_*_Crossplatform.zip && \
    mv ZAP_* /usr/local/bin/ZAP-proxy && \
    echo -e "#!/bin/bash\n\nbash /usr/local/bin/ZAP-proxy/zap.sh\n" > /usr/local/bin/zap && \
    chmod +x /usr/local/bin/zap

##############################################################################################################

###################################### Cleanup cache and garbage #############################################

RUN /usr/local/go/bin/go clean && \
    rm -rf /root/.cache && \
    gem sources -c

##############################################################################################################


ENTRYPOINT ["tail", "-f", "/dev/null"]
