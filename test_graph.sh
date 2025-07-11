#!/bin/bash

echo "Testing graph generation..."

# Run analyzer with sample data and create a histogram
# Input sequence: analyze result/sample_data.csv -> 3 (graph menu) -> 4 (histogram) -> 2 (age column) -> enter (default bins) -> 5 (exit graph menu) -> 5 (exit analysis)
echo -e "analyze result/sample_data.csv\n3\n4\n2\n\n5\n5\nexit\n" | timeout 30 cargo run

# Check if PNG files were created in result directory
echo "Checking for PNG files in result directory:"
ls -la result/*.png 2>/dev/null || echo "No PNG files found"

echo "Graph generation test completed!"