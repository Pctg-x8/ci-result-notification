#!/bin/bash

set -xe

BASE=s3://ct2infra-autodeploy-secrets/Pctg-x8/ci-result-notification

aws s3 sync ./.secrets $BASE/.secrets --delete
aws s3 sync ./src/character $BASE/src/character --delete --exclude "*" --include "internal.rs"
