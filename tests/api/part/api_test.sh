#!/bin/bash
set -e

API_URL="http://backend:3000"
ORIGIN_HEADER="Origin: http://localhost:5173"

echo "=== 🧪 Running health check ==="
curl -sf "$API_URL/healthz" | grep "OK" >/dev/null || {
  echo "❌ Health check failed"
  exit 1
}
echo "✅ Health check passed"

echo "=== 🧪 Signing up ==="
signup_res=$(curl -s -X POST "$API_URL/signup" \
  -H "Content-Type: application/json" \
  -d '{"login_name":"testuser","password":"password123"}')

echo "$signup_res" | jq .
code=$(echo "$signup_res" | jq -r '.code')

if [ "$code" == "201" ]; then
  echo "✅ Signup successful"
elif [ "$code" == "409" ]; then
  echo "⚠️  Already signed up"
else
  echo "❌ Unexpected signup response"
  exit 1
fi

echo "=== 🧪 Logging in ==="
login_res=$(curl -s -X POST "$API_URL/login" \
  -H "Content-Type: application/json" \
  -d '{"login_name":"testuser","password":"password123"}')

echo "$login_res" | jq .
token=$(echo "$login_res" | jq -r '.data.token')

if [ "$token" == "null" ] || [ -z "$token" ]; then
  echo "❌ Login failed"
  exit 1
fi
echo "✅ Login successful"

AUTH_HEADER="Authorization: Bearer $token"

echo "=== 🧪 Creating a part ==="
part_res=$(curl -s -X POST "$API_URL/parts" \
  -H "Content-Type: application/json" \
  -H "$AUTH_HEADER" \
  -H "$ORIGIN_HEADER" \
  -d '{"part_number":"XYZ-789","name":"ボルト","description":"大型用","kind":"部品"}')

echo "$part_res" | jq .
part_id=$(echo "$part_res" | jq -r '.data.id')

if [ "$part_id" == "null" ] || [ -z "$part_id" ]; then
  echo "❌ Part creation failed"
  exit 1
fi
echo "✅ Part created with ID: $part_id"

echo "=== 🧪 Getting created part ==="
get_part=$(curl -s -X GET "$API_URL/parts/$part_id" \
  -H "$AUTH_HEADER" -H "$ORIGIN_HEADER")
echo "$get_part" | jq .

echo "=== 🧪 Updating part with invalid data ==="
update_res=$(curl -s -X PUT "$API_URL/parts/$part_id" \
  -H "Content-Type: application/json" \
  -H "$AUTH_HEADER" -H "$ORIGIN_HEADER" \
  -d '{"part_number":"","name":"ボルト","description":"大型用","kind":"部品"}')

echo "$update_res" | jq .
update_code=$(echo "$update_res" | jq -r '.code')
if [ "$update_code" != "400" ]; then
  echo "❌ Expected validation error, got: $update_code"
  exit 1
fi
echo "✅ Validation for update worked"

echo "=== 🧪 Deleting part ==="
delete_res=$(curl -s -X DELETE "$API_URL/parts/$part_id" \
  -H "$AUTH_HEADER")
echo "$delete_res" | jq .
echo "✅ Part deleted"

echo "🎉 All API tests passed!"
