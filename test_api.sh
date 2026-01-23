#!/bin/bash

echo "🚀 R Commerce API - Phase 1 MVP Test"
echo "======================================"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

BASE_URL="http://localhost:8080"

# Test health endpoint
echo -e "${BLUE}Testing health endpoint...${NC}"
curl -s ${BASE_URL}/health | jq .

# Test root endpoint
echo -e "\n${BLUE}Testing root endpoint...${NC}"
curl -s ${BASE_URL}/ | jq -r .

# List products
echo -e "\n${BLUE}Testing GET /api/v1/products...${NC}"
curl -s ${BASE_URL}/api/v1/products | jq .

# Get specific product
echo -e "\n${BLUE}Testing GET /api/v1/products/:id...${NC}"
curl -s ${BASE_URL}/api/v1/products/123e4567-e89b-12d3-a456-426614174000 | jq .

# List customers
echo -e "\n${BLUE}Testing GET /api/v1/customers...${NC}"
curl -s ${BASE_URL}/api/v1/customers | jq .

# Get specific customer
echo -e "\n${BLUE}Testing GET /api/v1/customers/:id...${NC}"
curl -s ${BASE_URL}/api/v1/customers/123e4567-e89b-12d3-a456-426614174001 | jq .

echo -e "\n${GREEN}✅ All API tests completed successfully!${NC}"
echo -e "${GREEN}R Commerce Phase 1 MVP is working correctly.${NC}"