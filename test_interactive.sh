#!/bin/bash

# Test script to verify the interactive interface
echo "Testing the interactive data analyzer program..."
echo ""

# Run the program with some test input
echo -e "help\nexit" | cargo run

echo ""
echo "Test completed. The program should have shown:"
echo "1. Greeting message: '안녕하세요. 데이터 분석 프로그램입니다'"
echo "2. Usage information"
echo "3. Command prompt"
echo "4. Help information when 'help' was entered"
echo "5. Exit message when 'exit' was entered"