#!/bin/bash

# Replace these with your generated keys
ALICE_PUBKEY="4oZzKmePZ4iXdZqgkL6pJqSxsamxxghtmRZdY9khvoY8"
ALICE_PRIVKEY='[140,98,214,56,117,106,132,42,212,190,15,200,65,109,2,95,28,116,98,142,236,180,0,114,239,46,13,161,200,239,15,137,56,129,19,249,188,181,101,50,136,240,241,246,11,118,230,28,14,7,226,187,232,80,201,179,246,177,56,175,234,140,165,173]'  # e.g., [1,2,3,...]
BOB_PUBKEY="BQjhFa7SfB5ZN3i4iN8cp7S1QoLFbutSFG3CzYCnWSYA"

BASE_URL="http://localhost:8899"

echo "🧪 Testing Solana Fork Engine - Transfers"
echo "=========================================="
echo ""

# Step 1: Check Alice's initial balance
echo "1️⃣ Checking Alice's initial balance..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ALICE_PUBKEY\"}" | jq .
echo ""

# Step 2: Airdrop 1000 SOL to Alice
echo "2️⃣ Airdropping 1000 SOL to Alice..."
curl -s -X POST $BASE_URL/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ALICE_PUBKEY\", \"sol\": 1000}" | jq .
echo ""

# Step 3: Check Alice's balance after airdrop
echo "3️⃣ Checking Alice's balance after airdrop..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ALICE_PUBKEY\"}" | jq .
echo ""

# Step 4: Check Bob's initial balance
echo "4️⃣ Checking Bob's initial balance..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$BOB_PUBKEY\"}" | jq .
echo ""

# Step 5: Transfer 100 SOL from Alice to Bob
echo "5️⃣ Transferring 100 SOL from Alice to Bob..."
curl -s -X POST $BASE_URL/api/v1/fork/transfer \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ALICE_PUBKEY\",
    \"to\": \"$BOB_PUBKEY\",
    \"amount_sol\": 100,
    \"private_key\": \"$ALICE_PRIVKEY\"
  }" | jq .
echo ""

# Step 6: Check Alice's balance after transfer
echo "6️⃣ Checking Alice's balance after transfer..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ALICE_PUBKEY\"}" | jq .
echo ""

# Step 7: Check Bob's balance after transfer
echo "7️⃣ Checking Bob's balance after transfer..."
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$BOB_PUBKEY\"}" | jq .
echo ""

# Step 8: Check fork stats
echo "8️⃣ Checking fork stats..."
curl -s $BASE_URL/api/v1/fork/info | jq .
echo ""

echo "✅ Test complete!"