ENV ?= local
SERVICE ?= VOICE-KOREA-WATCHER
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)

BASE_DOMAIN ?= biyard.co
LOG_LEVEL ?= info

API_DOMAIN ?= https://voice-korea-api.$(ENV).$(BASE_DOMAIN)

ifeq ("$(ENV)","local")
	LOG_LEVEL ?= debug
	API_DOMAIN = http://0.0.0.0:3000
endif

BUILD_ENV ?= SERVICE=${SERVICE} ENV=$(ENV) VERSION=${VERSION} COMMIT=${COMMIT} API_DOMAIN=${API_DOMAIN}
run:
	$(BUILD_ENV) cargo run

build:
	$(BUILD_ENV) cargo build --release -p watcher --features lambda