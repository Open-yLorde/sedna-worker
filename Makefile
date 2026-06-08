COMPOSE ?= docker compose

.PHONY: help build up up-detached logs up-db logs-db

help:
	@echo "Available commands:"
	@echo "  build"
	@echo "  up"
	@echo "  up-detached"
	@echo "  logs"
	@echo "  up-db"
	@echo "  logs-db"

build:
	$(COMPOSE) build sedna-worker

up:
	$(COMPOSE) up --build sedna-worker

up-detached:
	$(COMPOSE) up -d --build sedna-worker

logs:
	$(COMPOSE) logs sedna-worker -f

up-db:
	$(COMPOSE) up postgres -d

logs-db:
	$(COMPOSE) logs postgres -f