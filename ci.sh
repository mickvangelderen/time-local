#!/usr/bin/env bash

set -euo pipefail

cargo clippy --fix --allow-dirty --allow-staged
cargo +nightly fmt
cargo test
