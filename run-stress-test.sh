#!/bin/bash

# Ensure Docker is installed
if ! command -v docker &> /dev/null
then
    echo "Docker could not be found. Please install Docker to run the stress test via this script, or install k6 locally."
    exit 1
fi

echo "Starting Open-Donate Stress Test with k6..."
echo "Target: ${BASE_URL:-http://localhost:3000}"

# Run k6 using docker, mounting the current directory to access stress.js
docker run --rm -i --network host -e BASE_URL="${BASE_URL:-http://localhost:3000}" grafana/k6 run - < load-test/stress.js
