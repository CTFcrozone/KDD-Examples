#!/bin/bash

set -euo pipefail

exec >> /proc/1/fd/1 2>&1

pkill -f /service/web-server/bin/web-server || true

# Start the binary as PID 1
exec /service/web-server/bin/web-server

