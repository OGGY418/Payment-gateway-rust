#!/bin/bash

SERVICE=${1:-all}

case $SERVICE in
    api)
        echo "📡 API Server Logs (Ctrl+C to exit):"
        tail -f logs/api.log
        ;;
    worker)
        echo "👷 Worker Logs (Ctrl+C to exit):"
        tail -f logs/worker.log
        ;;
    indexer)
        echo "⛓️  Indexer Logs (Ctrl+C to exit):"
        tail -f logs/indexer.log
        ;;
    all|*)
        echo "📊 All Service Logs (Ctrl+C to exit):"
        tail -f logs/*.log
        ;;
esac
