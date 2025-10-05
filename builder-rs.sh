#!/bin/bash
set -euo pipefail

OUT_DIR="rs-builder/rs-builder-target"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

docker run --rm \
  -v "$(pwd)/$OUT_DIR:/output" \
  rs-builder
