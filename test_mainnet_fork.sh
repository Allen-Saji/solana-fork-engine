#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Solana Mainnet Fork Testing${NC}"
echo -e "${BLUE}========================================${NC}\n"

BASE_URL="http://localhost:8899/api/v1"

# Test 1: Create a mainnet fork with real accounts
echo -e "${YELLOW}Test 1: Creating fork from mainnet...${NC}"
echo "Loading USDC mint and some popular addresses"

RESPONSE=$(curl -s -X POST "$BASE_URL/forks/mainnet" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "mainnet-tester",
    "accounts": [
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "So11111111111111111111111111111111111111112"
    ]
  }')

echo -e "${GREEN}Response:${NC}"
echo "$RESPONSE" | jq '.'

FORK_ID=$(echo "$RESPONSE" | jq -r '.fork_id')
echo -e "\n${GREEN}Fork ID: $FORK_ID${NC}\n"

# Test 2: Load additional account
echo -e "${YELLOW}Test 2: Loading additional account from mainnet...${NC}"
echo "Loading USDT mint"

RESPONSE2=$(curl -s -X POST "$BASE_URL/fork/load-account" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "mainnet-tester",
    "address": "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"
  }')

if echo "$RESPONSE2" | jq '.' > /dev/null 2>&1; then
  echo -e "${GREEN}Response:${NC}"
  echo "$RESPONSE2" | jq '.'
else
  echo -e "${YELLOW}Raw Response (not JSON):${NC}"
  echo "$RESPONSE2"
fi

echo ""

# Test 3: Load multiple accounts
echo -e "${YELLOW}Test 3: Loading multiple accounts...${NC}"

RESPONSE3=$(curl -s -X POST "$BASE_URL/fork/load-accounts" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "mainnet-tester",
    "addresses": [
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
      "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
    ]
  }')

if echo "$RESPONSE3" | jq '.' > /dev/null 2>&1; then
  echo -e "${GREEN}Response:${NC}"
  echo "$RESPONSE3" | jq '.'
else
  echo -e "${YELLOW}Raw Response (not JSON):${NC}"
  echo "$RESPONSE3"
fi

echo ""

# Test 4: Check balance of loaded account
echo -e "${YELLOW}Test 4: Checking balance of USDC mint account...${NC}"

curl -s -X POST "$BASE_URL/fork/balance/get" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "mainnet-tester",
    "address": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  }' | jq '.'

echo ""

# Test 5: Airdrop to test wallet
echo -e "${YELLOW}Test 5: Airdropping SOL to test wallet...${NC}"

TEST_WALLET="8FE27ioQh3T7o22QsYVT5Re8NnHFqmFNbdqwiF3ywuZQ"

curl -s -X POST "$BASE_URL/fork/airdrop" \
  -H "Content-Type: application/json" \
  -d "{
    \"user_id\": \"mainnet-tester\",
    \"address\": \"$TEST_WALLET\",
    \"sol\": 100
  }" | jq '.'

echo ""

# Test 6: List all forks
echo -e "${YELLOW}Test 6: Listing all forks...${NC}"

curl -s "$BASE_URL/forks" | jq '.'

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   All mainnet fork tests completed!${NC}"
echo -e "${GREEN}========================================${NC}"