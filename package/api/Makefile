ENV ?= local
PORT ?= 3000
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
JWT_SECRET ?= "JWT_SECRET"
ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= error
endif

ifeq ("$(ENV)","local")
	NONCE_LAB_API_TOKEN ?= eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEsInR5cGUiOiJ2ZW5kb3IiLCJpYXQiOjE3MzI1NDg2NDN9.R5CCPQx8UHDjHWqp4abpBYd2372-iF7btJvap6V4iTA5HvFZgEMjhichDn4ZKXKrR_cJmeyAnYhZpFqfmpME0w
	LOG_LEVEL ?= error
endif

NONCE_LAB_API_ENDPOINT = "http://15.165.76.144:7878"
LOG_LEVEL ?= debug

BUILD_ENV ?= ENV=$(ENV) PORT=$(PORT) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL) JWT_SECRET=$(JWT_SECRET) NONCE_LAB_API_ENDPOINT=${NONCE_LAB_API_ENDPOINT} NONCE_LAB_API_TOKEN=${NONCE_LAB_API_TOKEN}

setup.tool:
	cargo install cargo-watch systemfd toml-cli

run:
	${BUILD_ENV} systemfd --no-pid -s http::${PORT} -- cargo watch -x run --features reload

build:
	$(BUILD_ENV) cargo build -p api --release --features lambda
