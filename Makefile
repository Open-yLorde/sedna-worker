COMPOSE ?= docker compose

.PHONY: build help up-worker-detached up-db-detached up-worker up-db

help:
	@echo "Available commands:"
	@echo "  build-worker"
	@echo "  up-worker"
	@echo "  up-worker-detached"
	@echo "  up-db"
	@echo "  up-db-detached"

build-worker:
	$(COMPOSE) build --no-cache sedna-worker

up-worker:
	$(COMPOSE) up sedna-worker

up-worker-detached:
	$(COMPOSE) up sedna-worker -d

up-db:
	$(COMPOSE) up postgres

up-db-detached:
	$(COMPOSE) up postgres -d