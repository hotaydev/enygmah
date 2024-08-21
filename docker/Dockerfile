FROM alpine:3.20

WORKDIR /home/enygmah

############################# Installing needed tools for the Security tools we'll use #######################

RUN apk update
RUN apk add --no-cache gcc g++ make git curl wget jq ruby ruby-dev openjdk21 python3 py3-pip python3-dev bash

## Upgrading RubyGems
RUN gem update --system 3.5.17 --no-document

## Upgrading pip from Python
RUN python3 -m pip install --upgrade pip --break-system-packages

## Installing Go Lang
RUN export packageArch=$(if [ "$(apk --print-arch)" == "aarch64" ]; then echo "arm64"; else echo "amd64"; fi) && wget "https://go.dev/dl/go1.23.0.linux-${packageArch}.tar.gz"
RUN tar -C /usr/local -xzf ./go1.23.0.linux-*.tar.gz
RUN rm -rf ./go1.23.0.linux-*.tar.gz
RUN echo 'export PATH="$PATH:/usr/local/go/bin:/root/go/bin"' >> /etc/profile
ENV PATH="/usr/local/go/bin:/root/go/bin:${PATH}"

##############################################################################################################


################################### Installing Security tools we'll use ######################################

# Install Trivy:
RUN curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin v0.18.3

## Installing OSV-Scanner
RUN /usr/local/go/bin/go install github.com/google/osv-scanner/cmd/osv-scanner@v1

## Install GoSec
RUN /usr/local/go/bin/go install github.com/securego/gosec/cmd/gosec@latest

## Install WpScan
RUN gem install wpscan --no-document

## Install CppCheck
RUN apk add --no-cache cppcheck

## Install SpotBugs
RUN wget https://github.com/spotbugs/spotbugs/releases/download/4.8.6/spotbugs-4.8.6.tgz
RUN tar -xzf spotbugs-4.8.6.tgz && rm -rf spotbugs-4.8.6.tgz && mv ./spotbugs-4.8.6 /usr/local/bin/spotbugs
RUN chmod +x /usr/local/bin/spotbugs/bin/spotbugs
# Use it running /usr/local/bin/spotbugs/bin/spotbugs

## Install Semgrep
RUN python3 -m pip install semgrep --break-system-packages
# Use with `semgrep scan`

## Install Owasp Zap Proxy
RUN wget https://github.com/zaproxy/zaproxy/releases/download/v2.15.0/ZAP_2.15.0_Crossplatform.zip
RUN unzip ZAP_2.15.0_Crossplatform.zip && rm -rf ZAP_2.15.0_Crossplatform.zip && mv ZAP_2.15.0 /usr/local/bin/ZAP
# Use it with `bash /usr/local/bin/ZAP/zap.sh -cmd -quickurl "https://www.website.com/" -quickout ~/report.json`

##############################################################################################################

ENTRYPOINT ["/bin/sh"]
