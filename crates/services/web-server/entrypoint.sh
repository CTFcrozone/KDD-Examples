#!/bin/bash

#not sure about it

# start the cargo as a background
/service/web-server &

# make this pid 1 run infinitely
tail -f /dev/null
