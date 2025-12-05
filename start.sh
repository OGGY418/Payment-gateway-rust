#!/bin/bash

set -e  # Exit on error

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Solana Payment Gateway - Starting Services"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ .env file not found!"
    echo "   Creating from template..."
    cp .env.example .env
    echo ""
    echo "⚠️  Please edit .env and add your WALLET_ADDRESS"
    echo "   Run: cargo run --bin setup-wallet"
    exit 1
fi

# Check if WALLET_ADDRESS is set
if grep -q "your_wallet_address_here" .env; then
    echo "❌ WALLET_ADDRESS not configured in .env!"
    echo "   Run: cargo run --bin setup-wallet"
    echo "   Then edit .env with your wallet address"
    exit 1
fi

# Load environment variables
source .env

echo "✅ Configuration loaded"
echo ""

# Check Redis
echo "🔍 Checking Redis..."
if redis-cli ping > /dev/null 2>&1; then
    echo "✅ Redis is running"
else
    echo "❌ Redis not running!"
    echo "   Start Redis: redis-server"
    echo "   Or install: brew install redis (macOS)"
    exit 1
fi
echo ""

# Check PostgreSQL
echo "🔍 Checking PostgreSQL..."
if psql -lqt 2>/dev/null | cut -d \| -f 1 | grep -qw payment_gateway; then
    echo "✅ Database 'payment_gateway' exists"
else
    echo "⚠️  Database 'payment_gateway' not found"
    echo "   Creating database..."
    createdb payment_gateway
    echo "✅ Database created"
fi
echo ""

# Build project
echo "🔨 Building project..."
cargo build --release
echo "✅ Build complete"
echo ""

# Create logs directory
mkdir -p logs

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Starting all services..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start API Server
echo "🌐 Starting API Server..."
cargo run --release --bin payment-gateway-rust > logs/api.log 2>&1 &
API_PID=$!
echo "   PID: $API_PID"
sleep 2

# Start Worker
echo "👷 Starting Worker..."
cargo run --release --bin worker > logs/worker.log 2>&1 &
WORKER_PID=$!
echo "   PID: $WORKER_PID"
sleep 1

# Start Indexer
echo "⛓️  Starting Indexer..."
cargo run --release --bin indexer > logs/indexer.log 2>&1 &
INDEXER_PID=$!
echo "   PID: $INDEXER_PID"
sleep 1

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All services started!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Process IDs:"
echo "   API Server: $API_PID"
echo "   Worker:     $WORKER_PID"
echo "   Indexer:    $INDEXER_PID"
echo ""
echo "📡 API running at: http://localhost:3000"
echo ""
echo "📊 View logs:"
echo "   API:     tail -f logs/api.log"
echo "   Worker:  tail -f logs/worker.log"
echo "   Indexer: tail -f logs/indexer.log"
echo ""
echo "🛑 Stop services: ./stop.sh"
echo ""

# Save PIDs to file for stop script
echo "$API_PID" > .pids
echo "$WORKER_PID" >> .pids
echo "$INDEXER_PID" >> .pids

echo "✅ All services are running in background"
echo "   Check logs/ folder for output"
echo ""
