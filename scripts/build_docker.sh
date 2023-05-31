#!/bin/bash

# Build Wow for Linux AMD64
docker build --no-cache -t "wow-server:0.1.0" -f Dockerfile.server .
docker build --no-cache -t "wow-client:0.1.0" -f Dockerfile.client .


