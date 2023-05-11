#!/bin/bash

sh pub.sh
podman run --name frontend -d \
    -v /home/kaiaxc/Documents/newgit/libby/publish:/usr/share/nginx/html:ro \
 -p 8080:80 \
    nginx