#!/bin/bash

# Rust-Haskell Store Interoperability Test Runner
# This script builds and runs both the Haskell server and Rust client

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}================================================${NC}"
echo -e "${CYAN}Rust-Haskell Store Interoperability Test${NC}"
echo -e "${CYAN}================================================${NC}"
echo ""

# Check if stack is installed
if ! command -v stack &> /dev/null; then
    echo -e "${RED}Error: Stack (Haskell build tool) is not installed${NC}"
    echo "Please install Stack: https://docs.haskellstack.org/en/stable/README/"
    exit 1
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo (Rust build tool) is not installed${NC}"
    echo "Please install Rust: https://rustup.rs/"
    exit 1
fi

# Build Haskell server
echo -e "${YELLOW}[1/4] Building Haskell server...${NC}"
cd haskell-server
if stack build; then
    echo -e "${GREEN}✓ Haskell server built successfully${NC}"
else
    echo -e "${RED}✗ Failed to build Haskell server${NC}"
    exit 1
fi
cd ..
echo ""

# Build Rust client
echo -e "${YELLOW}[2/4] Building Rust client...${NC}"
cd rust-client
if cargo build --release; then
    echo -e "${GREEN}✓ Rust client built successfully${NC}"
else
    echo -e "${RED}✗ Failed to build Rust client${NC}"
    exit 1
fi
cd ..
echo ""

# Start Haskell server in background
echo -e "${YELLOW}[3/4] Starting Haskell server...${NC}"
cd haskell-server
stack run &
SERVER_PID=$!
cd ..

# Wait for server to start
echo -e "${BLUE}Waiting for server to be ready...${NC}"
for i in {1..30}; do
    if curl -s http://localhost:3000/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Server is ready${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}✗ Server failed to start within 30 seconds${NC}"
        kill $SERVER_PID 2>/dev/null || true
        exit 1
    fi
    sleep 1
done
echo ""

# Run Rust client tests
echo -e "${YELLOW}[4/4] Running interoperability tests...${NC}"
echo ""
cd rust-client
if cargo run --release; then
    TEST_RESULT=0
    echo ""
    echo -e "${GREEN}✓ All tests completed successfully!${NC}"
else
    TEST_RESULT=1
    echo ""
    echo -e "${RED}✗ Tests failed${NC}"
fi
cd ..

# Stop server
echo ""
echo -e "${BLUE}Stopping Haskell server...${NC}"
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true
echo -e "${GREEN}✓ Server stopped${NC}"

echo ""
echo -e "${CYAN}================================================${NC}"
if [ $TEST_RESULT -eq 0 ]; then
    echo -e "${GREEN}Test run completed successfully!${NC}"
else
    echo -e "${RED}Test run failed!${NC}"
fi
echo -e "${CYAN}================================================${NC}"

exit $TEST_RESULT