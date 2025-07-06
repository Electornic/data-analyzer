#!/bin/bash

# Test script to verify the demo command works in interactive mode
echo "Testing the demo command in interactive mode..."
echo ""

# Run the program with demo command
echo -e "demo\nexit" | cargo run

echo ""
echo "Demo test completed."