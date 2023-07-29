#!/usr/bin/env bash

DMNTK_PORT=22023 dmntk srv -c never --port=22024 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1