#!/bin/bash

echo "Testing data-analyzer demo..."

# Clean up any existing files
rm -rf result/*

# Run the demo (automatically answer with demo and then exit)
echo -e "demo\n5\n" | cargo run

# Check if files were created in result directory
echo "Checking files in result directory:"
ls -la result/

echo "Demo test completed!"