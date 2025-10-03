#!/bin/bash

# start the cargo as a background
/service/web-server/bin/web-server &

# make this pid 1 run infinitely
tail -f /dev/null
