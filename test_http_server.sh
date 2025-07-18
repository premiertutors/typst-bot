#!/bin/bash

# Test script for the HTTP server

echo "Testing the HTTP server..."

# Test 1: Valid Typst document
echo "Test 1: Valid Typst document"
response=$(curl -s -X POST localhost:8080/render -d '{"code":"= Hello World\n\nThis is a test document with *bold* and _italic_ text."}' -H 'Content-Type: application/json')
if echo "$response" | grep -q '"images"'; then
    echo "✓ Valid document test passed"
else
    echo "✗ Valid document test failed"
    echo "$response"
fi

# Test 2: Invalid Typst code
echo "Test 2: Invalid Typst code"
response=$(curl -s -X POST localhost:8080/render -d '{"code":"#invalid-function()"}' -H 'Content-Type: application/json')
if echo "$response" | grep -q "unknown function"; then
    echo "✓ Error handling test passed"
else
    echo "✗ Error handling test failed"
    echo "$response"
fi

# Test 3: Empty code
echo "Test 3: Empty code"
response=$(curl -s -X POST localhost:8080/render -d '{"code":""}' -H 'Content-Type: application/json')
if echo "$response" | grep -q '"images"'; then
    echo "✓ Empty code test passed"
else
    echo "✗ Empty code test failed"
    echo "$response"
fi

echo "All tests completed!"
