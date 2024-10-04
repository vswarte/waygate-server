#!/bin/sh
if [ -z "$WAYGATE_SERVER_SECRET_KEY" ]; then
    echo "Generating new key pair...\n"
    echo "Update your .env file with the following values and restart the container:\n"
    ./waygate-generate-keys
    # wait so container won't exit and trigger a restart
    while true; do sleep 1000; done
fi

./waygate-server
