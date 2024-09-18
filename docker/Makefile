
IMAGE_NAME = hotay/enygmah

build:
	docker build -t $(IMAGE_NAME) .

clean:
	docker stop enygmah sonarqube-enygmah; \
	docker rm enygmah sonarqube-enygmah; \
	docker rmi $(IMAGE_NAME); \
	docker builder prune --force

.PHONY: build
