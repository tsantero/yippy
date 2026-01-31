#!/bin/bash
set -e

cargo build --quiet

echo "=== test_set_print ==="
./target/debug/yippy tests/test_set_print.yaml

echo ""
echo "=== test_interpolation ==="
./target/debug/yippy tests/test_interpolation.yaml

echo ""
echo "=== test_if ==="
./target/debug/yippy tests/test_if.yaml
