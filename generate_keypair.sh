#!/bin/bash

# This script generates test keypairs for the fork engine

echo "Generating test keypairs..."
echo ""

# Generate keypair using Solana SDK
cargo run --quiet --example generate_keypair