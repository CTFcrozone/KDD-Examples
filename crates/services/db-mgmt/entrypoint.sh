#!/bin/bash
set -e  # Exit on error

# Start the application in background
echo "Starting db-mgmt..."
/service/db-mgmt &
APP_PID=$!

# Setup signal handling
_term() {
    echo "Caught termination signal, stopping web-server..."
    kill -TERM "$APP_PID" 2>/dev/null
    wait "$APP_PID"
    exit 0
}

trap _term SIGTERM SIGINT

# Keep container alive
# Use wait instead of tail -f for proper signal handling
echo "Container running, waiting for db-mgmt (PID: $APP_PID)..."
wait "$APP_PID"

# If web-server exits, the container will exit
echo "db-mgmt process ended"
