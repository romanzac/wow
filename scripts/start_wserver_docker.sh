#!/bin/sh

# Start script for docker version of Word of Wisdom server

docker run --name wow-server \
  -e WOW_SERVER_LISTEN=0.0.0.0 \
  -e WOW_SERVER_PORT=3333 \
  -e WOW_SERVER_QUOTES_FILE=quotes.json \
  -e WOW_SERVER_LOG_LEVEL=info \
  -p 3333:3333 \
  wow-server:0.1.0
