#!/bin/bash
set -e

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

# Clean previous packages
if [ -d "pkg" ]; then
    rm -rf pkg
fi

if [ -d "pkg-node" ]; then
    rm -rf pkg-node
fi

# Build for both targets
wasm-pack build -t nodejs -d pkg-node
wasm-pack build -t browser -d pkg

# Get the package name
PKG_NAME=$(jq -r .name pkg/package.json | sed 's/\-/_/g')

# Give the pacakges a version tag
if [ -n "$VERSION_TAG" ]; then
    # Overwrite the version in the package.json
    jq --arg v $VERSION_TAG '.version = $v' pkg/package.json > temp.json && mv temp.json pkg/package.json
    jq --arg v $VERSION_TAG '.version = $v' pkg-node/package.json > temp.json && mv temp.json pkg-node/package.json
fi