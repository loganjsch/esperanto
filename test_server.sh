#!/bin/bash

# Exit immediately if any command fails
set -e




echo "--- 1. Compiling project..."
# We use `cargo build` which is faster if no changes were made
cargo build -p esperanto-server --quiet

echo "--- 2. Starting server in background..."
# Run the compiled binary in the background
./target/debug/esperanto-server &

# Capture the Process ID (PID) of the last background command
SERVER_PID=$!

# Give the server a moment to start up
sleep 2

# Check if the server is still running
if ! ps -p $SERVER_PID > /dev/null; then
    echo "!!! Server failed to start!"
    exit 1
fi

echo "--- 3. Sending test request to http://localhost:3000/verify/nitro_enclave"
# Use curl to test the endpoint. The -f flag makes curl fail with an error code
# if the server returns an HTTP error (like 404), which would stop the script.
# We expect a 501, which curl considers a "success" at the HTTP level.
RESPONSE_CODE=$(curl --silent --output /dev/null --write-out "%{http_code}" \
  -X POST http://localhost:3000/verify/nitro_enclave \
  -H "Content-Type: application/json" \
  --data '{"policyId": "test", "attestationDocument": "test"}')

echo "--- 4. Server responded with HTTP Status Code: $RESPONSE_CODE"

# Clean up by killing the server process
echo "--- 5. Killing server (PID: $SERVER_PID)..."
kill $SERVER_PID

# Check if the response code was what we expected
if [ "$RESPONSE_CODE" -eq 501 ]; then
    echo "--- ✅ Test Successful! Server responded correctly."
    exit 0
else
    echo "--- ❌ Test Failed! Expected HTTP 501, but got $RESPONSE_CODE."
    exit 1
fi