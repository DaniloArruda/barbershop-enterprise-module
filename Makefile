SHELL = /bin/bash

.DEFAULT_GOAL := help

.PHONY: help

OS = $(shell uname -s)

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

load-local-environment: load-docker-compose setup-kafka ## Create the local environment and add all required configurations

unload-local-environment:## Destroy environment
	docker-compose down

load-docker-compose: ## Run docker-compose up -d
	docker-compose up -d

setup-kafka: ## Create kafka topics
	./local_development/kafka/setup-kafka.sh

publish-appointment-solicited: ## Publish appointment-solicited event
	./local_development/kafka/publish-appointment-solicited.sh

publish-appointment-confirmed: ## Publish appointment-confirmed event
	./local_development/kafka/publish-appointment-confirmed.sh
