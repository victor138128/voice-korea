ENV ?= local
PORT ?= 3000
COMMIT ?= $(shell git rev-parse --short HEAD)
LOG_LEVEL ?= info
VERSION ?= v0.1.0


BUILD_ENV ?= ENV=$(ENV) PORT=$(PORT) VERSION=${VERSION} COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL)


setup.tool:
	cargo install cargo-watch systemfd

run:
	${BUILD_ENV} systemfd --no-pid -s http::${PORT} -- cargo watch -x run --features reload

build:
	cargo build