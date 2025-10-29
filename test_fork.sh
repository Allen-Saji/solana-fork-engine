#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

BASE_URL="http://localhost:8899"
ADDRESS="11111111111111111111111111111111"

echo -e "${BLUE}=== Solana Fork Engine Test Script ===${NC}\n"

# Test 1: Get fork info
echo -e "${GREEN}1. Getting fork info...${NC}"
curl -s $BASE_URL/api/v1/fork/info | jq .
echo -e "\n"

# Test 2: Check initial balance
echo -e "${GREEN}2. Checking initial balance...${NC}"
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ADDRESS\"}" | jq .
echo -e "\n"

# Test 3: Set balance to 1000 SOL
echo -e "${GREEN}3. Setting balance to 1000 SOL...${NC}"
curl -s -X POST $BASE_URL/api/v1/fork/balance/set \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ADDRESS\", \"lamports\": 1000000000000}" | jq .
echo -e "\n"

# Test 4: Check new balance
echo -e "${GREEN}4. Checking new balance...${NC}"
curl -s -X POST $BASE_URL/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ADDRESS\"}" | jq .
echo -e "\n"

# Test 5: Get full account info
echo -e "${GREEN}5. Getting full account info...${NC}"
curl -s -X POST $BASE_URL/api/v1/fork/account \
  -H "Content-Type: application/json" \
  -d "{\"address\": \"$ADDRESS\"}" | jq .
echo -e "\n"

echo -e "${BLUE}=== Tests Complete ===${NC}"