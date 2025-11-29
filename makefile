APP_NAME=delivery
HOOKS_DIR := .git-hooks

.PHONY: build test

prepare:
	@git config --local core.hooksPath "$(HOOKS_DIR)"
	@chmod +x $(HOOKS_DIR)/* || true
	@echo "✅ Git hooks подключены (core.hooksPath = $(HOOKS_DIR))"

build: test ## Build application
	cargo build

test: ## Run tests
	cargo test

create-courier:
	@curl localhost:8082/api/v1/couriers --json '{ "name": "JOHN", "speed": 1 }' -H 'content-type: application/json'

generate-server:
	@openapi-generator-cli generate -g rust-axum -i https://gitlab.com/microarch-ru/ddd-in-practice/system-design/-/raw/main/services/delivery/contracts/openapi.yml -c configs/server.cfg.yaml -o internal/generated/servers

generate-geo-client:
	@rm -rf internal/generated/clients/geosrv
	@curl -s -o configs/geo.proto https://gitlab.com/microarch-ru/ddd-in-practice/system-design/-/raw/main/services/geo/contracts/contract.proto
	@protoc --go_out=internal/generated/clients --go-grpc_out=internal/generated/clients configs/geo.proto

generate-basket-queue:
	@rm -rf internal/generated/queues/basketconfirmedpb
	@curl -s -o configs/basket_confirmed.proto https://gitlab.com/microarch-ru/ddd-in-practice/system-design/-/raw/main/services/basket/contracts/basket_confirmed.proto
	@protoc --go_out=internal/generated --go-grpc_out=internal/generated configs/basket_confirmed.proto

generate-order-queue:
	@rm -rf internal/generated/queues/orderstatuschangedpb
	@curl -s -o configs/order_status_changed.proto https://gitlab.com/microarch-ru/ddd-in-practice/system-design/-/raw/main/services/delivery/contracts/order_status_changed.proto
	@protoc --go_out=internal/generated --go-grpc_out=internal/generated configs/order_status_changed.proto
