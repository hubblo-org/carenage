API_CARGO=./api/Cargo.toml
CARENAGED_CARGO=./carenaged/Cargo.toml
CARENAGE_CLI_CARGO=./carenage-cli/Cargo.toml
DATABASE_CARGO=./database/Cargo.toml
POSTGRES_CONTAINER=carenage-database-1
PREFIX=/usr/local/bin
CHECK_DATABASE_RUNNING=$(shell docker inspect -f '{{.State.Status}}' $(POSTGRES_CONTAINER))

build_debug:
	@echo "Building debug"
	@echo "Building binary for API web server"
	@cargo build --manifest-path $(API_CARGO)
	@echo "Building binary for Carenage daemon"
	@cargo build --manifest-path $(CARENAGED_CARGO)
	@echo "Building binary for Carenage CLI"
	@cargo build --manifest-path $(CARENAGE_CLI_CARGO)

build_release:
	@echo "Building release"
	@echo "Building binary for API web server"
	@cargo build --manifest-path $(API_CARGO) --release
	@echo "Building binary for Carenage daemon"
	@cargo build --manifest-path $(CARENAGED_CARGO) --release
	@echo "Building binary for Carenage CLI"
	@cargo build --manifest-path $(CARENAGE_CLI_CARGO) --release

compose_dev:
	@echo "Launching Docker development environment orchestration"
	@docker compose --profile dev up -d

compose_prod:
	@echo "Launching Docker production environment orchestration"
	@docker compose --profile prod up -d

compose_dev_stop:
	@echo "Stopping Docker development environment orchestration"
	@docker compose --profile dev down

compose_prod_stop:
	@echo "Stopping Docker production environment orchestration"
	@docker compose --profile prod down

install_debug:
	@echo "Installing debug binaries"
	@cp target/debug/api $(PREFIX)/
	@cp target/debug/carenaged $(PREFIX)/
	@cp target/debug/carenage-cli $(PREFIX)/

install_release:
	@echo "Installing debug binaries"
	@cp target/release/api $(PREFIX)/
	@cp target/release/carenaged $(PREFIX)/
	@cp target/release/carenage-cli $(PREFIX)/

test:
	@echo "Running tests. A test database Docker container must be running, otherwise some tests will never run!"
ifeq ($(CHECK_DATABASE_RUNNING), running) 
	cargo test --manifest-path $(API_CARGO) &&\
	cargo test --manifest-path $(CARENAGED_CARGO) -- --test-threads=1 &&\
	cargo test --manifest-path $(DATABASE_CARGO) 
else 
	echo exit 1 
endif
