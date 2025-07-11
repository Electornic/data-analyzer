#!/bin/bash

echo "Simple test for graph generation..."

# Try to run with a simple input sequence
echo -e "analyze result/sample_data.csv\n3\n4\n2\n\n5\n5\nexit\n" | cargo run &

# Wait a bit and then kill the process
sleep 10
pkill -f "cargo run" 2>/dev/null

# Check results
echo "Checking result directory:"
ls -la result/

echo "Test completed!"