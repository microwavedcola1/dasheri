#!/usr/bin/env bash

set -euo pipefail
(cd ../mango-v3; cargo build-bpf; cp -v ./target/deploy/mango.so  \
  ../dasheri/programs/dasheri/tests/fixtures);

cargo test-bpf