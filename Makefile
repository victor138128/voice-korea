ENV ?= local

SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
BASE_DOMAIN ?= biyard.co

DOMAIN ?= voice-korea.$(ENV).$(BASE_DOMAIN)
# TABLE_NAME ?= $(SERVICE)-$(ENV)
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'$(DOMAIN)')].id" --output text --region us-east-1)
HOSTED_ZONE_ID ?= $(shell basename `aws route53 list-hosted-zones-by-name --dns-name $(BASE_DOMAIN) --query "HostedZones[0].Id" --output text`)
WORKSPACE_ROOT ?= $(PWD)

ifeq ("$(ENV)","prod")
	TABLE_NAME = voice-korea-prod
endif

ifeq ("$(ENV)","dev")
	TABLE_NAME = voice-korea-dev
endif

BUILD_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) TABLE_NAME=$(TABLE_NAME) CDN_ID=$(CDN_ID) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE)

run:
	cd platform && make run

deploy: build cdk-build cdk-deploy s3-sync cdn-invalidate

clean:
	rm -rf .build

.PHONY: build
build: clean
	cd platform && $(BUILD_ENV) make build-lambda
	mv .build/platform/platform .build/platform/bootstrap

fixtures/cdk/node_modules:
	cd fixtures/cdk && npm install

cdk-build: fixtures/cdk/node_modules
	cd fixtures/cdk && $(BUILD_ENV) npm run build
	cd fixtures/cdk && $(BUILD_ENV) cdk synth

cdk-deploy:
	cd fixtures/cdk && yes | $(BUILD_ENV) cdk deploy --require-approval never $(AWS_FLAG)

s3-sync:
	aws s3 sync .build/platform/public s3://$(DOMAIN) $(AWS_FLAG)

cdn-invalidate:
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

test:
	echo "No tests"
