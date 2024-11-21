ENV ?= local
PORT ?= 3000
COMMIT ?= $(shell git rev-parse --short HEAD)
LOG_LEVEL ?= info
VERSION ?= $(shell cargo pkgid | cut -d'#' -f2)

BUILD_ENV ?= ENV=$(ENV) PORT=$(PORT) VERSION=${VERSION} COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL)

setup.tool:
	cargo install cargo-watch systemfd

run:
	${BUILD_ENV} systemfd --no-pid -s http::${PORT} -- cargo watch -x run --features reload

build:
	cargo build
