#!/bin/bash

echo "🛑 Stopping Solana Payment Gateway Services..."
echo ""

if [ -f .pids ]; then
    while read pid; do
        if ps -p $pid > /dev/null 2>&1; then
            echo "   Stopping PID: $pid"
            kill $pid 2>/dev/null || true
        fi
    done < .pids
    rm .pids
    echo ""
    echo "✅ All services stopped"
else
    echo "⚠️  No PID file found. Trying to stop by name..."
    pkill -f "payment-gateway-rust" || true
    pkill -f "worker" || true
    pkill -f "indexer" || true
    echo "✅ Done"
fi
echo ""
