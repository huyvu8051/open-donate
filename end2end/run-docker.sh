#!/bin/bash
apt-get update
apt-get install -y socat
socat TCP-LISTEN:3000,fork TCP:host.docker.internal:3000 &
socat TCP-LISTEN:8080,fork TCP:host.docker.internal:8080 &
sleep 1
npm install
npx playwright test --project=webkit --reporter=line
