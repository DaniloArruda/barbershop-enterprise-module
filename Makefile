SHELL = /bin/bash

.DEFAULT_GOAL := help

.PHONY: help

OS = $(shell uname -s)

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

load-local-environment: load-docker-compose create-kafka ## Create the local environment and add all required configurations

unload-local-environment:## Destroy localstack
	docker-compose down

load-docker-compose: ##  Run kafka, Zookeper, redis, dynamo and wiremock
	docker-compose up -d

create-kafka: ## Create kafka topics
	./local_development/kafka/setup-kafka.sh

publish-appointment-solicited: ## Create kafka topics
	./local_development/kafka/publish-appointment-solicited.sh

publish-appointment-confirmed: ## Create kafka topics
	./local_development/kafka/publish-appointment-confirmed.sh
