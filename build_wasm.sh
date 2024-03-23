#!/bin/bash
# chmod +x build_wasm.sh

# set -e: Exit the script if any command returns a non-zero status
set -e

# Retrieve the package name from Cargo.toml
PROJECT_NAME=$(grep '^name = ' Cargo.toml | head -1 | cut -d '"' -f2)

if [ -z "$PROJECT_NAME" ]; then
    echo "Failed to retrieve the project name from Cargo.toml." >&2
    exit 1
fi

echo "Project name: $PROJECT_NAME"

# Build command
cargo build --profile wasm-release --target wasm32-unknown-unknown

# Execute wasm-bindgen command
wasm-bindgen --out-name "$PROJECT_NAME" \
  --out-dir wasm/target \
  --target web "target/wasm32-unknown-unknown/wasm-release/${PROJECT_NAME}.wasm"

# Optimization command
wasm-opt -Oz --output "wasm/target/optimized.wasm" "wasm/target/${PROJECT_NAME}_bg.wasm"

# Rename the optimized file
mv "wasm/target/optimized.wasm" "wasm/target/${PROJECT_NAME}_bg.wasm"

echo "Build and optimization of ${PROJECT_NAME} are complete."