component_test:
	@echo "Running components' tests with Vitest"
	npm test

e2e_test:
	@echo "Running end-to-end tests with Playwright"
	npx playwright test

install: 
	npm install

build_prod:
	@echo "Building production executable"
	npm run build

run_prod:
	@echo "Trying to run production environment..."
ifneq ($(wildcard build),)
	@echo "Running production environment"; \
	NODE_ENV=production node build
else
	@echo "Build not present, run 'make build_prod' first!"; \
	exit 1
endif

run_dev:
	@echo "Running development environment"
	npm run dev
