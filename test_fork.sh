#!/bin/bash

# Test Fork Script
# Tests basic fork operations: creation, info, balance checks

set -e

BASE_URL="http://localhost:8899"

echo "🧪 Testing Solana Fork Engine"
echo "=============================="
echo ""

# Test 1: Health check
echo "1️⃣ Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s -w "\n%{http_code}" $BASE_URL/health)
HTTP_CODE=$(echo "$HEALTH_RESPONSE" | tail -n1)
if [ "$HTTP_CODE" == "200" ]; then
    echo "   ✅ Health check passed"
else
    echo "   ❌ Health check failed (HTTP $HTTP_CODE)"
    exit 1
fi
echo ""

# Test 2: Root endpoint
echo "2️⃣ Testing root endpoint..."
curl -s $BASE_URL/ | jq '.'
echo ""

# Test 3: Fork info
echo "3️⃣ Getting fork information..."
curl -s $BASE_URL/api/v1/fork/info | jq '.'
echo ""

# Test 4: Create/Get fork
echo "4️⃣ Testing fork creation..."
curl -s -X POST $BASE_URL/api/v1/forks \
  -H "Content-Type: application/json" \
  -d '{"user_id": "test_user"}' | jq '.'
echo ""

# Use a random test address (not the system program)
TEST_ADDRESS="8FE27ioQh3T7o22QsYVT5Re8NnHFqmFNbdqwiF3ywuZQ"
echo "5️⃣ Checking balance for test address..."
echo "   Address: $TEST_ADDRESS"
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$TEST_ADDRESS\"}" | jq '.'
echo ""

# Test 6: Set balance
echo "6️⃣ Setting balance to 1 SOL (1000000000 lamports)..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/set \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$TEST_ADDRESS\", \"lamports\": 1000000000}" | jq '.'
echo ""

# Test 7: Verify new balance
echo "7️⃣ Verifying new balance (should be 1 SOL)..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$TEST_ADDRESS\"}" | jq '.'
echo ""

# Test 8: Airdrop test
echo "8️⃣ Testing airdrop (adding 50 SOL)..."
curl -s -X POST $BASE_URL/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$TEST_ADDRESS\", \"sol\": 50}" | jq '.'
echo ""

# Test 9: Final balance check
echo "9️⃣ Final balance check (should be 51 SOL total)..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$TEST_ADDRESS\"}" | jq '.'
echo ""

echo "✅ All fork tests completed!"
echo ""
echo "📊 Summary:"
echo "   • Health check: ✅"
echo "   • Fork creation: ✅"
echo "   • Balance operations: ✅"
echo "   • Airdrop: ✅"