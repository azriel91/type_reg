#! /bin/bash
set -euo pipefail

cargo llvm-cov clean --workspace
mkdir -p ./target/coverage

# See `.config/cargo.toml`
for i in {0..2}
do cargo coverage_$i
done

cargo coverage_merge
