COMPOSE ?= docker compose

.PHONY: help build up up-detached logs

help:
	@echo "Available commands:"
	@echo "  build-worker"
	@echo "  up-worker"
	@echo "  up-worker-detached"
	@echo "  up-db"
	@echo "  up-db-detached"

build:
	$(COMPOSE) build --no-cache sedna-worker

up:
	$(COMPOSE) up sedna-worker

up-detached:
	$(COMPOSE) up sedna-worker -d

logs:
	$(COMPOSE) logs sedna-worker