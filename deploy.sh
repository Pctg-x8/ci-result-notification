#!/bin/bash

set -xe

CF_ARTIFACTS_BUCKET_NAME=ct2infra-cf-artifacts
STACK_NAME=CIResultNotificationGHA

aws cloudformation package --template-file app.cf --s3-bucket $CF_ARTIFACTS_BUCKET_NAME --s3-prefix lambda/ci-notification --output-template-file transformed.cf
aws cloudformation deploy --stack-name $STACK_NAME --template-file transformed.cf --capabilities CAPABILITY_IAM
