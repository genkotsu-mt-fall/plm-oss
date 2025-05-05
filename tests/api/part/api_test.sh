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

echo "=== 🧪 Creating a part -2- ==="
part_res=$(curl -s -X POST "$API_URL/parts" \
  -H "Content-Type: application/json" \
  -H "$AUTH_HEADER" \
  -H "$ORIGIN_HEADER" \
  -d '{"part_number":"XYZ-789","name":"ボルト","description":"大型用","kind":"部品"}')

echo "$part_res" | jq .
part_id_1=$(echo "$part_res" | jq -r '.data.id')

if [ "$part_id_1" == "null" ] || [ -z "$part_id_1" ]; then
  echo "❌ Part creation failed"
  exit 1
fi
echo "✅ Part created with ID: $part_id_1"

echo "=== 🧪 Creating a part -2- ==="
part_res=$(curl -s -X POST "$API_URL/parts" \
  -H "Content-Type: application/json" \
  -H "$AUTH_HEADER" \
  -H "$ORIGIN_HEADER" \
  -d '{"part_number":"XYZ-789","name":"ボルト","description":"大型用","kind":"部品"}')

echo "$part_res" | jq .
part_id_2=$(echo "$part_res" | jq -r '.data.id')

if [ "$part_id_2" == "null" ] || [ -z "$part_id_2" ]; then
  echo "❌ Part creation failed"
  exit 1
fi
echo "✅ Part created with ID: $part_id_2"

echo "=== 🧪 Getting created part ==="
get_part=$(curl -s -X GET "$API_URL/parts/$part_id_1" \
  -H "$AUTH_HEADER" -H "$ORIGIN_HEADER")
echo "$get_part" | jq .

echo "=== 🧪 Updating part with invalid data ==="
update_res=$(curl -s -X PUT "$API_URL/parts/$part_id_1" \
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

echo "=== 🧪 Signing up as second user ==="
signup_res2=$(curl -s -X POST "$API_URL/signup" \
  -H "Content-Type: application/json" \
  -d '{"login_name":"otheruser","password":"pass456"}')

login_res2=$(curl -s -X POST "$API_URL/login" \
  -H "Content-Type: application/json" \
  -d '{"login_name":"otheruser","password":"pass456"}')

token2=$(echo "$login_res2" | jq -r '.data.token')
AUTH_HEADER2="Authorization: Bearer $token2"

echo "=== 🧪 Trying to delete another user's part ==="
unauth_delete=$(curl -s -X DELETE "$API_URL/parts/$part_id_1" \
  -H "$AUTH_HEADER2")

code=$(echo "$unauth_delete" | jq -r '.code')
if [ "$code" != "401" ]; then
  echo "❌ Unauthorized delete should fail, got: $code"
  exit 1
fi
echo "✅ Unauthorized delete blocked"

echo "=== 🧪 Deleting part ==="
delete_res=$(curl -s -X DELETE "$API_URL/parts/$part_id_1" \
  -H "$AUTH_HEADER")
echo "$delete_res" | jq .
echo "✅ Part deleted"

echo "=== 🧪 Logging in as admin ==="
admin_login_res=$(curl -s -X POST "$API_URL/login" \
  -H "Content-Type: application/json" \
  -d '{"login_name":"admin","password":"admin"}')

echo "$admin_login_res" | jq .
admin_token=$(echo "$admin_login_res" | jq -r '.data.token')
ADMIN_AUTH_HEADER="Authorization: Bearer $admin_token"

if [ "$admin_token" == "null" ] || [ -z "$admin_token" ]; then
  echo "❌ Admin login failed"
  exit 1
fi
echo "✅ Admin login successful"

echo "=== 🧪 Admin updating another user's part ==="
admin_update=$(curl -s -X PUT "$API_URL/parts/$part_id_2" \
  -H "Content-Type: application/json" \
  -H "$ADMIN_AUTH_HEADER" -H "$ORIGIN_HEADER" \
  -d '{"part_number":"ADM-002","name":"ボルト(管理者修正)","description":"管理者更新","kind":"管理用"}')

echo "$admin_update" | jq .
admin_update_code=$(echo "$admin_update" | jq -r '.code')
if [ "$admin_update_code" != "200" ]; then
  echo "❌ Admin should be able to update part, got: $admin_update_code"
  exit 1
fi
echo "✅ Admin was able to update part"

echo "=== 🧪 Admin deleting another user's part ==="
admin_delete=$(curl -s -X DELETE "$API_URL/parts/$part_id_2" \
  -H "$ADMIN_AUTH_HEADER")

echo "$admin_delete" | jq .
admin_delete_code=$(echo "$admin_delete" | jq -r '.code')
if [ "$admin_delete_code" != "204" ]; then
  echo "❌ Admin should be able to delete part, got: $admin_delete_code"
  exit 1
fi
echo "✅ Admin was able to delete part"

echo "🎉 All API tests passed!"
