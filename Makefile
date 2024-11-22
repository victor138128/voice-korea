ENV ?= local

SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
BASE_DOMAIN ?= biyard.co

DOMAIN ?= voice-korea.$(ENV).$(BASE_DOMAIN)
API_DOMAIN ?= voice-korea-api.$(ENV).$(BASE_DOMAIN)
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(DOMAIN)')].id" --output text --region us-east-1)
API_ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(API_DOMAIN)')].id" --output text --region us-east-1)
HOSTED_ZONE_ID ?= $(shell basename `aws route53 list-hosted-zones-by-name --dns-name $(BASE_DOMAIN) --query "HostedZones[0].Id" --output text`)
WORKSPACE_ROOT ?= $(PWD)

ifeq ("$(ENV)","prod")
	TABLE_NAME = voice-korea-prod
endif

TABLE_NAME = voice-korea-api-dev

BUILD_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) CDN_ID=$(CDN_ID) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE)

BUILD_WEB_CDK_ENV ?= SERVICE=$(SERVICE) ENV=$(ENV) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) CODE_PATH=$(PWD)/.build/platform ENABLE_S3=true ENABLE_DYNAMO=false
BUILD_API_CDK_ENV ?= SERVICE=$(SERVICE)-api ENV=$(ENV) DOMAIN=$(API_DOMAIN) TABLE_NAME=$(TABLE_NAME) ACM_ID=$(API_ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) CODE_PATH=$(PWD)/.build/api ENABLE_DYNAMO=true

run:
	cd package/api && make run &
	cd platform && make run

deploy.web: build cdk-deploy.web s3-sync cdn-invalidate

deploy.api: build-api cdk-deploy.api

deploy-web-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(DOMAIN)/api/version | tr -d \" | cut -d'-' -f1 ))
	$(eval CURRENT_VERSION := $(shell cargo pkgid -p platform | cut -d'#' -f2))
	$(eval SHOULD_UPGRADE := $(shell if [ $(DEPLOYED_VERSION) != $(CURRENT_VERSION) ] ; then echo "true"; else echo "false"; fi))
	$(eval RESULT := $(shell if [ $(SHOULD_UPGRADE) = "true" ] ; then make deploy.web > /dev/null; echo "completed deployement"; else echo "already latest version"; fi))
	@echo "$(RESULT)"

deploy-api-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(API_DOMAIN)/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell cargo pkgid -p api | cut -d'#' -f2))
	$(eval SHOULD_UPGRADE := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "true"; else echo "false"; fi))
	$(eval RESULT := $(shell if [ $(SHOULD_UPGRADE) = "true" ] ; then make deploy.api > /dev/null; echo "completed deployement"; else echo "already latest version"; fi))
	@echo "$(RESULT)"

clean:
	rm -rf .build

.PHONY: build build-api
build: clean
	cd platform && $(BUILD_ENV) make build-lambda
	mkdir -p .build/platform
	cp -r target/dx/platform/release/web .build/platform
	mv .build/platform/web/public .build/platform/public
	mv .build/platform/web/server .build/platform/bootstrap

build-api: clean
	cd package/api && ENV=$(ENV) make build
	mkdir -p .build/api
	cp target/release/api .build/api/bootstrap

fixtures/cdk/node_modules:
	cd fixtures/cdk && npm install

cdk-deploy.web: fixtures/cdk/node_modules
	cd fixtures/cdk && $(BUILD_WEB_CDK_ENV) npm run build
	cd fixtures/cdk && $(BUILD_WEB_CDK_ENV) cdk synth
	cd fixtures/cdk && $(BUILD_WEB_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG)

cdk-deploy.api: fixtures/cdk/node_modules
	cd fixtures/cdk && $(BUILD_API_CDK_ENV) npm run build
	cd fixtures/cdk && $(BUILD_API_CDK_ENV) cdk synth
	cd fixtures/cdk && $(BUILD_API_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG)

s3-sync:
	aws s3 sync .build/platform/public s3://$(DOMAIN) $(AWS_FLAG)

cdn-invalidate:
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

test:
	echo "No tests"
