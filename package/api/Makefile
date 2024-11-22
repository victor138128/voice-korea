ENV ?= local
PORT ?= 3000
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell cargo pkgid | cut -d'#' -f2)

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= error
endif
LOG_LEVEL ?= debug

BUILD_ENV ?= ENV=$(ENV) PORT=$(PORT) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL)

setup.tool:
	cargo install cargo-watch systemfd

run:
	${BUILD_ENV} systemfd --no-pid -s http::${PORT} -- cargo watch -x run --features reload

build:
	$(BUILD_ENV) cargo build -p api --release --features lambda
