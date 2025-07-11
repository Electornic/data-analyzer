#!/bin/bash

echo "Testing sample data creation..."

# Clean up any existing sample files
rm -rf sample/
mkdir -p sample

# Build the project
cargo build --release

# Test 1: Create first sample file
echo "Test 1: Creating first sample file..."
echo "demo" | timeout 10s ./target/release/data-analyzer || echo "Demo completed or timed out"

# Check if sample_data.csv was created
if [ -f "sample/sample_data.csv" ]; then
    echo "✓ sample/sample_data.csv created successfully"
else
    echo "✗ sample/sample_data.csv not found"
fi

# Test 2: Create second sample file (should be numbered)
echo "Test 2: Creating second sample file..."
echo "demo" | timeout 10s ./target/release/data-analyzer || echo "Demo completed or timed out"

# Check if sample_data_1.csv was created
if [ -f "sample/sample_data_1.csv" ]; then
    echo "✓ sample/sample_data_1.csv created successfully"
else
    echo "✗ sample/sample_data_1.csv not found"
fi

# Test 3: Create third sample file
echo "Test 3: Creating third sample file..."
echo "demo" | timeout 10s ./target/release/data-analyzer || echo "Demo completed or timed out"

# Check if sample_data_2.csv was created
if [ -f "sample/sample_data_2.csv" ]; then
    echo "✓ sample/sample_data_2.csv created successfully"
else
    echo "✗ sample/sample_data_2.csv not found"
fi

echo "Listing sample directory contents:"
ls -la sample/

echo "Test completed!"