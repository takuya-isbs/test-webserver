#!/bin/sh
set -eux

cd /app
cargo clippy
cargo test
