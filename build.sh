#!/bin/bash
set -e

echo "Building CSS with Tailwind..."
npm run css:build

echo "Building Leptos project..."
cargo leptos build "$@"

echo "Build complete!"
