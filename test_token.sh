#!/bin/bash

set -e  # Exit on error

# Token Operations Test Script
BASE_URL="http://localhost:8899"
USER_ID="alice-$(date +%s)"

echo "=== Solana Fork Engine - Token Operations Test ==="
echo "User ID: $USER_ID"
echo ""

# Check arguments
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "❌ Usage: $0 <base58_keypair> <public_key>"
    echo ""
    echo "Example:"
    echo "  $0 4iCmwH7x47XBjxs3xru2ggTdeNchsGVDDed2vdEKKn18rbiEPdmVCSJcBzsPGxUSJPMjgbcDKS6GABxqCu9biXhT DsUr3d9fV1c3DRfWYgE3Wo66XdHQV8tM6gNGoU1W9azD"
    exit 1
fi

PAYER_KEYPAIR="$1"
OWNER_ADDRESS="$2"

echo "Keypair length: ${#PAYER_KEYPAIR}"
echo "Owner: $OWNER_ADDRESS"
echo ""

# 1. Create fork
echo "1. Creating fork..."
curl -s -X POST "$BASE_URL/api/v1/forks" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\"}" | jq '.'
echo ""

# 2. Airdrop
echo "2. Airdropping SOL..."
curl -s -X POST "$BASE_URL/api/v1/fork/airdrop" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"address\": \"$OWNER_ADDRESS\", \"sol\": 10}" | jq '.'
echo ""

# 3. Create mint
echo "3. Creating token mint..."
MINT_RESP=$(curl -s -X POST "$BASE_URL/api/v1/token/create-mint" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"payer_keypair\": \"$PAYER_KEYPAIR\", \"decimals\": 9}")
echo "$MINT_RESP" | jq '.' 2>/dev/null || echo "Raw response: $MINT_RESP"
MINT_ADDRESS=$(echo "$MINT_RESP" | jq -r '.mint_address' 2>/dev/null)
echo "Mint: $MINT_ADDRESS"
echo ""

# 4. Create token account
echo "4. Creating token account..."
TOKEN_RESP=$(curl -s -X POST "$BASE_URL/api/v1/token/create-account" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"payer_keypair\": \"$PAYER_KEYPAIR\", \"mint_address\": \"$MINT_ADDRESS\", \"owner_address\": \"$OWNER_ADDRESS\"}")
echo "$TOKEN_RESP" | jq '.' 2>/dev/null || echo "Raw response: $TOKEN_RESP"
TOKEN_ACCOUNT=$(echo "$TOKEN_RESP" | jq -r '.token_account' 2>/dev/null)
echo "Token Account: $TOKEN_ACCOUNT"
echo ""

# 5. Mint tokens
echo "5. Minting 1000 tokens..."
curl -s -X POST "$BASE_URL/api/v1/token/mint" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"mint_authority_keypair\": \"$PAYER_KEYPAIR\", \"mint_address\": \"$MINT_ADDRESS\", \"destination_account\": \"$TOKEN_ACCOUNT\", \"amount\": 1000000000000}" | jq '.'
echo ""

# 6. Check balance
echo "6. Checking balance..."
curl -s -X POST "$BASE_URL/api/v1/token/balance" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"token_account\": \"$TOKEN_ACCOUNT\"}" | jq '.'
echo ""

# 7. Create second account
BOB_ADDRESS="H8pMvGWbY8SJXuFUHvtmMVHq3Gz5h1yvb7BdTiLHqCDQ"
echo "7. Creating Bob's token account..."
TOKEN_RESP2=$(curl -s -X POST "$BASE_URL/api/v1/token/create-account" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"payer_keypair\": \"$PAYER_KEYPAIR\", \"mint_address\": \"$MINT_ADDRESS\", \"owner_address\": \"$BOB_ADDRESS\"}")
echo "$TOKEN_RESP2" | jq '.'
TOKEN_ACCOUNT2=$(echo "$TOKEN_RESP2" | jq -r '.token_account')
echo "Bob's Token Account: $TOKEN_ACCOUNT2"
echo ""

# 8. Transfer
echo "8. Transferring 100 tokens..."
curl -s -X POST "$BASE_URL/api/v1/token/transfer" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\": \"$USER_ID\", \"from_keypair\": \"$PAYER_KEYPAIR\", \"source_account\": \"$TOKEN_ACCOUNT\", \"destination_account\": \"$TOKEN_ACCOUNT2\", \"amount\": 100000000000}" | jq '.'
echo ""

echo "✅ Test complete!"
echo "Mint: $MINT_ADDRESS"
echo "Alice: $TOKEN_ACCOUNT"
echo "Bob: $TOKEN_ACCOUNT2"