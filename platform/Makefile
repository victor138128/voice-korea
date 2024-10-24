SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
VERSION ?= v0.1.0
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
HOST ?= 0.0.0.0:8080
PROFILE ?= default

RUST_LOG ?= info

.ONESHELL:

ifneq ("$(PROFILE)","default")
	AWS_FLAG += --profile $(PROFILE)
endif

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
endif

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
AWS_DYNAMODB_KEY ?= "id"
TABLE_NAME ?= $(SERVICE)-$(ENV)
RUSTFLAGS ?=
LOG_LEVEL ?= debug

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL) AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_DYNAMODB_KEY=${AWS_DYNAMODB_KEY} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} TABLE_NAME=$(AWS_DYNAMODB_TABLE)

setup.tool:
	cargo install dioxus-cli --version 0.6.0-alpha.3
	npm i -g tailwindcss

run: public/tailwind.css
	$(BUILD_ENV) dx serve --platform fullstack -p platform -i false

serve: build
	dist/bootstrap

public/tailwind.css:
	tailwindcss -i ./input.css -o ./public/tailwind.css --minify

build: public/tailwind.css
	$(BUILD_ENV) dx build -p platform --platform fullstack
	mv $(WORKSPACE_ROOT)/.build/platform/platform $(WORKSPACE_ROOT)/.build/platform/bootstrap

.ONESHELL:
build-lambda: public/tailwind.css
	$(BUILD_ENV) dx build --platform fullstack --server-feature lambda -p platform
	mv $(WORKSPACE_ROOT)/dist/$(SERVICE) $(WORKSPACE_ROOT)/dist/bootstrap
