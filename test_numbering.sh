#!/bin/bash

echo "Testing sample data numbering logic..."

# Clean up
rm -rf sample/
mkdir -p sample

# Test the numbering logic by manually creating files and checking the logic
echo "Creating test files to verify numbering logic..."

# Create first file manually
echo "name,age,score" > sample/sample_data.csv
echo "Test,25,85.5" >> sample/sample_data.csv

echo "Created sample/sample_data.csv"
ls -la sample/

# Now test the demo command which should create sample_data_1.csv
echo "Running demo command (should create sample_data_1.csv)..."
echo -e "demo\n5\nexit\n" | cargo run

echo "After running demo:"
ls -la sample/

# Run demo again (should create sample_data_2.csv)
echo "Running demo again (should create sample_data_2.csv)..."
echo -e "demo\n5\nexit\n" | cargo run

echo "After running demo again:"
ls -la sample/

echo "Test completed!"