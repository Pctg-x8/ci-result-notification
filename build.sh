#!/bin/bash

set -xe

cargo build --release --target x86_64-unknown-linux-gnu
(cd target/x86_64-unknown-linux-gnu/release && zip ../../../package.zip bootstrap)
