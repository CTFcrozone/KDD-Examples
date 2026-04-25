#!/bin/bash
set -e

echo "=== Restarting db-mgmt ==="

# Find our specific process (not just any cargo)
PID=$(ps aux | grep '/service/db-mgmt' | grep -v grep | awk '{print $2}')

if [ -n "$PID" ]; then
    echo "Stopping db-mgmt (PID: $PID) gracefully..."

    # First try graceful shutdown (SIGTERM)
    kill -TERM "$PID"

    # Wait up to 10 seconds for graceful shutdown
    timeout=10
    while [ $timeout -gt 0 ] && kill -0 "$PID" 2>/dev/null; do
        sleep 1
        ((timeout--))
    done

    # Force kill if still running
    if kill -0 "$PID" 2>/dev/null; then
        echo "Process not responding, forcing kill..."
        kill -9 "$PID"
    fi

    echo "Process stopped."
else
    echo "No db-mgmt process found."
fi

# Start new instance
echo "Starting new instance..."
nohup /service/db-mgmt >> /proc/1/fd/1 2>&1 &
NEW_PID=$!

echo "New instance started with PID: $NEW_PID"
