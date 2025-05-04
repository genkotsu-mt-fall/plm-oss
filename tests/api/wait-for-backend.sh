#!/bin/bash
set -e

API_URL="http://backend:3000"

echo "⏳ Waiting for backend at $API_URL/healthz..."

for i in {1..30}; do
  status=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/healthz" || true)
  if [ "$status" = "200" ]; then
    echo "✅ Backend is up!"
    exit 0
  fi
  echo "Waiting... ($i) status=$status"
  sleep 2
done

echo "❌ Backend did not become healthy in time."
exit 1
