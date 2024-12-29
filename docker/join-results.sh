#!/bin/sh

cd /home/enygmah/_outputs/

if [ $(cat ./sonarqube_hotspots.json | jq ".paging.total") == "0" ]; then
  echo "No sonarqube hotspots found"
fi

if [ $(cat ./sonarqube_issues.json | jq ".paging.total") == "0" ]; then
  echo "No sonarqube issues found"
fi

jq -s '{"version": "2.1.0", "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json", "runs": map(.runs) | add}' --indent 2 ./*.sarif > ./output.sarif

find . -maxdepth 1 -type f -name "*.sarif" ! -name "output.sarif" -exec rm {} +

